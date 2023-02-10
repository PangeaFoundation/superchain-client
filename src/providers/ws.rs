use std::{
    collections::{
        btree_map::Entry,
        BTreeMap,
    },
    fmt::Debug,
};

use async_trait::async_trait;
use base64::{
    engine::general_purpose::STANDARD as BASE64,
    Engine,
};
use ethers::types::H160;
use futures::{
    future,
    select,
    stream::Fuse,
    Sink,
    SinkExt,
    Stream,
    StreamExt,
    TryStreamExt,
};
use futures_channel::mpsc;
use reqwest::header;
use tokio_tungstenite::connect_async;
use tracing::{
    debug,
    error,
    trace,
    warn,
};
use tungstenite::{
    client::IntoClientRequest,
    Message,
};
use uuid::Uuid;

use crate::core::{
    error::{
        Error,
        Result,
    },
    provider::{
        EthereumProvider,
        Provider,
        StreamResponse,
        UniswapV2Provider,
        UniswapV3Provider,
    },
    types::{
        ethereum::BlockHeader,
        query::QueryOptions,
        uniswap_v2,
        uniswap_v3,
    },
};

const WS_PATH: &str = "websocket";

type WsMsg = Result<tungstenite::Message, tungstenite::Error>;
type WsResult = Result<Vec<u8>>;
type OperationMsg = (Uuid, Operation, mpsc::UnboundedSender<WsResult>);

#[derive(Clone, Debug)]
pub struct WsProvider {
    operations: mpsc::UnboundedSender<OperationMsg>,
}

impl WsProvider {
    async fn request<T>(&self, operation: Operation) -> StreamResponse<T>
    where
        T: serde::de::DeserializeOwned + 'static,
    {
        let raw_data_stream = self.raw_request(operation).await?;
        let raw_data_stream = raw_data_stream.into_async_read();

        let stream = csv_async::AsyncDeserializer::from_reader(raw_data_stream);
        let records = stream
            .into_deserialize::<T>()
            .map(|r| {
                match r {
                    Ok(r) => Ok(r),
                    Err(e) => Err(Error::from(e)),
                }
            })
            .fuse()
            .boxed();

        Ok(records)
    }

    async fn raw_request(
        &self,
        operation: Operation,
    ) -> Result<impl Stream<Item = Result<Vec<u8>, std::io::Error>> + Send> {
        let (sink, stream) = mpsc::unbounded();
        let id = Uuid::new_v4();

        self.operations
            .unbounded_send((id, operation, sink))
            .map_err(|_| Error::BackendShutDown)?;

        let stream = stream
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            .filter_map(|data| {
                match data {
                    Ok(data) => {
                        if data.is_empty() {
                            return future::ready(None);
                        }
                        future::ready(Some(Ok(data)))
                    }
                    Err(e) => future::ready(Some(Err(e))),
                }
            });

        Ok(stream)
    }

    /// Returns true if the WS connection is active, false otherwise
    pub fn ready(&self) -> bool {
        !self.operations.is_closed()
    }
}

#[async_trait]
impl Provider for WsProvider {
    async fn try_new(
        endpoint: String,
        is_secure: bool,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Self> {
        let mut req = format!(
            "{}://{endpoint}/{WS_PATH}",
            if is_secure { "wss" } else { "ws" },
        )
        .into_client_request()?;

        if let (Some(username), Some(password)) = (username, password) {
            let auth = format!("{username}:{password}");
            let encoded = BASE64.encode(auth);

            req.headers_mut().append(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Basic {encoded}"))
                    .expect("Only non-ascii chars result in an error"),
            );
        }

        let (ws, _) = connect_async(req).await?;

        let (sink, stream) = mpsc::unbounded();

        let bw = BackgroundWorker::new(ws, stream);
        tokio::spawn(bw.main_loop());

        Ok(Self { operations: sink })
    }
}

#[async_trait]
impl EthereumProvider for WsProvider {
    async fn get_height(&self) -> Result<u64> {
        let stream = self.raw_request(Operation::GetHeight).await?;
        futures::pin_mut!(stream);
        let bytes = stream
            .next()
            .await
            .transpose()?
            .ok_or_else(|| Error::Custom("empty response from websocket".to_owned()))?;
        let bytes: [u8; 8] = TryFrom::try_from(&*bytes)
            .map_err(|_| Error::Custom("failed to collect bytes for height bytes".to_owned()))?;
        Ok(u64::from_ne_bytes(bytes))
    }

    async fn get_headers(&self, opts: QueryOptions) -> StreamResponse<BlockHeader> {
        self.request(Operation::GetBlockHeaders { opts }).await
    }
}

#[async_trait]
impl UniswapV2Provider for WsProvider {
    async fn get_pairs(&self, opts: QueryOptions) -> StreamResponse<uniswap_v2::Pair> {
        self.request(Operation::GetPairs {
            pairs: vec![],
            opts,
        })
        .await
    }

    async fn get_pair(&self, _pair_address: H160) -> Result<uniswap_v2::Pair> {
        Err(Error::Custom("not implemented".to_owned()))
    }

    async fn get_prices<I>(&self, pairs: I, opts: QueryOptions) -> StreamResponse<uniswap_v2::Price>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        let pairs = pairs.into_iter().map(|pair| pair.0).collect();
        self.request(Operation::GetPrices { pairs, opts }).await
    }

    async fn get_raw_reserves<I>(
        &self,
        pairs: I,
        opts: QueryOptions,
    ) -> StreamResponse<uniswap_v2::Reserves>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        let pairs = pairs.into_iter().map(|pair| pair.0).collect();
        self.request(Operation::GetReserves { pairs, opts }).await
    }

    async fn get_trades<I>(&self, pairs: I, opts: QueryOptions) -> StreamResponse<uniswap_v2::Trade>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        let pairs = pairs.into_iter().map(|pair| pair.0).collect();
        self.request(Operation::GetTrades { pairs, opts }).await
    }
}

#[async_trait]
impl UniswapV3Provider for WsProvider {
    async fn get_pools(&self, opts: QueryOptions) -> StreamResponse<uniswap_v3::Pool> {
        self.request(Operation::GetPools {
            pools: vec![],
            opts,
        })
        .await
    }

    async fn get_pool(&self, _pool_address: H160) -> Result<uniswap_v3::Pool> {
        Err(Error::Custom("not implemented".to_owned()))
    }

    async fn get_prices<I>(&self, pools: I, opts: QueryOptions) -> StreamResponse<uniswap_v3::Price>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        let pools = pools.into_iter().map(|pool| pool.0).collect();
        self.request(Operation::GetV3Prices { pools, opts }).await
    }
}

struct BackgroundWorker<S> {
    ws: Fuse<S>,
    operations: Fuse<mpsc::UnboundedReceiver<OperationMsg>>,
    subscriptions: BTreeMap<Uuid, mpsc::UnboundedSender<Result<Vec<u8>>>>,
}

impl<S> BackgroundWorker<S>
where
    S: Send + Sync + Stream<Item = WsMsg> + Sink<Message, Error = tungstenite::Error> + Unpin,
{
    pub fn new(ws: S, operations: mpsc::UnboundedReceiver<OperationMsg>) -> Self {
        Self {
            ws: ws.fuse(),
            operations: operations.fuse(),
            subscriptions: BTreeMap::default(),
        }
    }

    /// Returns whether the all work has been completed.
    ///
    /// If this method returns `true`, then the `operations` channel has been
    /// closed and all requests have been completed.
    fn is_done(&self) -> bool {
        self.operations.is_done() && self.subscriptions.is_empty()
    }

    // Shutdown
    async fn shutdown(&mut self) {
        let _ = self.ws.close().await;

        for (_, sub) in self.subscriptions.iter() {
            sub.close_channel();
        }
        self.subscriptions.clear();
    }

    pub async fn main_loop(mut self) {
        loop {
            if self.is_done() {
                debug!("work complete");
                break;
            }

            if self.ws.is_done() {
                error!("WebSocket closed unexpectedly");
                self.shutdown().await;
                break;
            }

            if let Err(e) = self.tick().await {
                error!("Received a WebSocket error: {:?}", e);
                self.shutdown().await;
                break;
            }
        }
    }

    async fn tick(&mut self) -> Result<()> {
        select! {
            // Handle operations
            operation = self.operations.select_next_some() => {
                self.operate(operation).await?;
            },
            // Handle ws messages
            resp = self.ws.next() => match resp {
                Some(Ok(resp)) => self.handle(resp).await?,
                Some(Err(err)) => {
                    tracing::error!(?err);
                    return Err(Error::UnexpectedClose);
                }
                None => {
                    return Err(Error::UnexpectedClose);
                },
            }
        };

        Ok(())
    }

    async fn operate(&mut self, operation: OperationMsg) -> Result<()> {
        let (id, operation, sink) = operation;

        let request = Request { id, operation };
        let payload = serde_json::to_vec(&request)?;

        if self.subscriptions.insert(id, sink).is_some() {
            warn!("Replacing already-registered subscription with id {:?}", id);
        }

        if let Err(e) = self.ws.send(Message::Binary(payload)).await {
            error!("WS connection error: {:?}", e);
            let sink = self.subscriptions.remove(&id);
            if let Some(sink) = sink {
                sink.close_channel();
            }
        }

        Ok(())
    }

    async fn handle(&mut self, resp: Message) -> Result<()> {
        match resp {
            Message::Text(_) => Err(Error::UnexpectedMessage),
            Message::Frame(_) => Ok(()), // Server is allowed to send Raw frames
            Message::Ping(inner) => self.handle_ping(inner).await,
            Message::Pong(_) => Ok(()), // Server is allowed to send unsolicited pongs.
            Message::Close(_) => Err(Error::UnexpectedClose),
            Message::Binary(buf) => self.handle_binary(buf).await,
        }
    }

    async fn handle_ping(&mut self, inner: Vec<u8>) -> Result<()> {
        self.ws.send(Message::Pong(inner)).await?;
        Ok(())
    }

    async fn handle_binary(&mut self, data: Vec<u8>) -> Result<()> {
        trace!("received message {:?} bytes", data.len());
        let (header, data) = Header::try_from_data(data)?;

        let msg = if header.marker.contains(MsgMarker::END) {
            let sink = self.subscriptions.remove(&header.id);
            if let Some(sink) = sink {
                sink.close_channel();
            }
            return Ok(());
        } else if header.marker.contains(MsgMarker::START) {
            return Ok(());
        } else if header.marker.contains(MsgMarker::ERROR) {
            match String::from_utf8(data) {
                Ok(s) => Err(Error::ErrorMsg(s)),
                Err(_) => Err(Error::UnexpectedMessageFormat),
            }
        } else if header.marker.contains(MsgMarker::CONTINUE) {
            Ok(data)
        } else {
            Err(Error::UnexpectedMessageFormat)
        };

        let id = header.id;
        if let Entry::Occupied(stream) = self.subscriptions.entry(id) {
            if let Err(err) = stream.get().unbounded_send(msg) {
                if err.is_disconnected() {
                    // subscription channel was closed on the receiver end
                    stream.remove();
                }
                return Err(Error::Custom(format!("failed to send message: {err:?}")));
            }
        }

        Ok(())
    }
}

#[derive(serde::Serialize)]
struct Request {
    id: Uuid,
    #[serde(flatten)]
    operation: Operation,
}

#[derive(serde::Serialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
enum Operation {
    GetPairs {
        pairs: Vec<[u8; 20]>,
        #[serde(flatten)]
        opts: QueryOptions,
    },
    GetPrices {
        pairs: Vec<[u8; 20]>,
        #[serde(flatten)]
        opts: QueryOptions,
    },
    GetReserves {
        pairs: Vec<[u8; 20]>,
        #[serde(flatten)]
        opts: QueryOptions,
    },
    GetTrades {
        pairs: Vec<[u8; 20]>,
        #[serde(flatten)]
        opts: QueryOptions,
    },
    GetBlockHeaders {
        #[serde(flatten)]
        opts: QueryOptions,
    },
    GetHeight,
    GetPools {
        pools: Vec<[u8; 20]>,
        #[serde(flatten)]
        opts: QueryOptions,
    },
    GetV3Prices {
        pools: Vec<[u8; 20]>,
        #[serde(flatten)]
        opts: QueryOptions,
    },
}

struct Header {
    marker: MsgMarker,
    id: Uuid,
    _counter: u32,
}

impl Header {
    const SIZE: usize = 21;

    fn try_from_data(mut data: Vec<u8>) -> Result<(Self, Vec<u8>)> {
        let data_len = data.len();
        if data_len < Self::SIZE {
            return Err(Error::UnexpectedMessageFormat);
        }

        let header = &data[(data_len - Self::SIZE)..];

        let marker = MsgMarker::from_bits(header[0]).ok_or(Error::UnexpectedMessageFormat)?;
        let id: [u8; 16] = header[1..17]
            .try_into()
            .map_err(|_| Error::UnexpectedMessageFormat)?;
        let id = Uuid::from_bytes_le(id);
        let _counter = u32::from_be_bytes(header[17..].try_into().unwrap());

        let header = Self {
            marker,
            id,
            _counter,
        };
        data.truncate(data_len - Self::SIZE);

        Ok((header, data))
    }
}

bitflags::bitflags! {
    struct MsgMarker: u8 {
        const START        = 0b00000001;
        const CONTINUE     = 0b00000010;
        const END          = 0b00000100;
        const ERROR        = 0b10000000;
        const SUBSCRIPTION = 0b01000000;
    }
}
