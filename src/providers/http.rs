use std::str::FromStr;

use async_trait::async_trait;
use base64::{
    engine::general_purpose::STANDARD as BASE64,
    Engine,
};
use ethers::types::H160;
use futures::{
    StreamExt,
    TryStreamExt,
};
use reqwest::header;

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
        query::{
            Bound,
            QueryOptions,
        },
        uniswap_v2,
        uniswap_v3,
    },
};

const API_PATH: &str = "api/v1/";

pub struct HttpProvider {
    inner: reqwest::Client,
    base_url: reqwest::Url,
}

impl HttpProvider {
    async fn request<T>(&self, url: reqwest::Url, opts: QueryOptions) -> StreamResponse<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        let raw_data_stream = self
            .inner
            .get(url)
            .query(&opts)
            .send()
            .await?
            .error_for_status()?
            .bytes_stream()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));

        let stream = csv_async::AsyncDeserializer::from_reader(raw_data_stream.into_async_read())
            .into_deserialize::<T>()
            .map_err(Error::from)
            .fuse()
            .boxed();

        Ok(stream)
    }

    fn url(&self, path: &str) -> Result<reqwest::Url> {
        self.base_url.join(path).map_err(Error::from)
    }
}

#[async_trait]
impl Provider for HttpProvider {
    async fn try_new(
        endpoint: String,
        is_secure: bool,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let (Some(username), Some(password)) = (username, password) {
            let auth = format!("{username}:{password}");
            let encoded = BASE64.encode(auth);
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Basic {encoded}"))
                    .expect("Only non-ascii chars result in an error"),
            );
        }

        let base_url = reqwest::Url::from_str(&format!(
            "{}://{}/{}",
            if is_secure { "https" } else { "http" },
            endpoint,
            API_PATH
        ))?;

        let inner = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .https_only(is_secure)
            .build()?;

        Ok(Self { inner, base_url })
    }
}

const ETHEREUM_HEIGHT_PATH: &str = "eth/height";
const ETHEREUM_HEADERS_PATH: &str = "eth/headers";

#[async_trait]
impl EthereumProvider for HttpProvider {
    async fn get_height(&self) -> Result<u64> {
        let url = self.url(ETHEREUM_HEIGHT_PATH)?;

        let height = self
            .inner
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?
            .parse()
            .map_err(|_| Error::Custom("Failed to parse height".to_string()))?;

        Ok(height)
    }

    async fn get_headers(&self, opts: QueryOptions) -> StreamResponse<BlockHeader> {
        let url = self.url(ETHEREUM_HEADERS_PATH)?;
        self.request(url, opts).await
    }
}

const UNISWAP_V2_PAIRS_PATH: &str = "uniswap/v2/pairs";
const UNISWAP_V2_PAIR_PRICES_PATH: &str = "prices";
const UNISWAP_V2_PAIR_RESERVES_PATH: &str = "raw/reserves";
const UNISWAP_V2_PAIR_TRADES_PATH: &str = "trades";

#[async_trait]
impl UniswapV2Provider for HttpProvider {
    async fn get_pairs(&self, opts: QueryOptions) -> StreamResponse<uniswap_v2::Pair> {
        let url = self.url(UNISWAP_V2_PAIRS_PATH)?;
        self.request(url, opts).await
    }

    async fn get_pair(&self, pair_address: H160) -> Result<uniswap_v2::Pair> {
        let url = self.url(&format!("{UNISWAP_V2_PAIRS_PATH}/{pair_address:#x}"))?;
        let opts = QueryOptions::default().start(Bound::Exact(0));
        let stream = self.request::<uniswap_v2::Pair>(url, opts).await?;
        futures::pin_mut!(stream);

        let Some(pair) = stream.next().await else {
            return Err(Error::Custom(format!("Pair {pair_address} not found")));
        };

        match pair {
            Ok(pair) => Ok(pair),
            // fixme - this is not a good way to handle this
            Err(err) => Err(Error::Custom(format!("Failed to get pair: {err}"))),
        }
    }

    async fn get_prices<I>(&self, pairs: I, opts: QueryOptions) -> StreamResponse<uniswap_v2::Price>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        let pairs: Vec<H160> = pairs.into_iter().collect();
        if pairs.is_empty() {
            return Err(Error::Custom("No pairs provided".to_string()));
        }

        let url = self.url(&format!(
            "{}/{:#x}/{}",
            UNISWAP_V2_PAIRS_PATH, pairs[0], UNISWAP_V2_PAIR_PRICES_PATH
        ))?;
        self.request(url, opts).await
    }

    async fn get_raw_reserves<I>(
        &self,
        pairs: I,
        opts: QueryOptions,
    ) -> StreamResponse<uniswap_v2::Reserves>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        let pairs: Vec<H160> = pairs.into_iter().collect();
        if pairs.is_empty() {
            return Err(Error::Custom("No pairs provided".to_string()));
        }

        let url = self.url(&format!(
            "{}/{:#x}/{}",
            UNISWAP_V2_PAIRS_PATH, pairs[0], UNISWAP_V2_PAIR_RESERVES_PATH
        ))?;
        self.request(url, opts).await
    }

    async fn get_trades<I>(&self, pairs: I, opts: QueryOptions) -> StreamResponse<uniswap_v2::Trade>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        let pairs: Vec<H160> = pairs.into_iter().collect();
        if pairs.is_empty() {
            return Err(Error::Custom("No pairs provided".to_string()));
        }

        let url = self.url(&format!(
            "{}/{:#x}/{}",
            UNISWAP_V2_PAIRS_PATH, pairs[0], UNISWAP_V2_PAIR_TRADES_PATH
        ))?;
        self.request(url, opts).await
    }
}

const UNISWAP_V3_POOL_PATH: &str = "uniswap/v3/pools";
const UNISWAP_V3_POOL_PRICES_PATH: &str = "prices";

#[async_trait]
impl UniswapV3Provider for HttpProvider {
    async fn get_pools(&self, opts: QueryOptions) -> StreamResponse<uniswap_v3::Pool> {
        let url = self.url(UNISWAP_V3_POOL_PATH)?;
        self.request(url, opts).await
    }

    async fn get_pool(&self, pool_address: H160) -> Result<uniswap_v3::Pool> {
        let url = self.url(&format!("{UNISWAP_V3_POOL_PATH}/{pool_address:#x}"))?;
        let opts = QueryOptions::default().start(Bound::Exact(0));
        let stream = self.request::<uniswap_v3::Pool>(url, opts).await?;
        futures::pin_mut!(stream);

        let Some(pool) = stream.next().await else {
            return Err(Error::Custom(format!("Pair {pool_address} not found")));
        };

        match pool {
            Ok(pool) => Ok(pool),
            // fixme - this is not a good way to handle this
            Err(err) => Err(Error::Custom(format!("Failed to get pool: {err}"))),
        }
    }

    async fn get_prices<I>(&self, pools: I, opts: QueryOptions) -> StreamResponse<uniswap_v3::Price>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        let pools: Vec<H160> = pools.into_iter().collect();
        if pools.is_empty() {
            return Err(Error::Custom("No pools provided".to_string()));
        }

        let url = self.url(&format!(
            "{}/{:#x}/{}",
            UNISWAP_V3_POOL_PATH, pools[0], UNISWAP_V3_POOL_PRICES_PATH
        ))?;
        self.request(url, opts).await
    }
}
