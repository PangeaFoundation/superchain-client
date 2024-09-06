use std::{collections::HashSet, str::FromStr};

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use futures::{StreamExt, TryStreamExt};
use reqwest::header;

use crate::{
    core::{
        error::{Error, Result},
        provider::{ChainProvider, Provider, StreamResponse, UniswapV2Provider, UniswapV3Provider},
        requests::{
            self, blocks::GetBlocksRequest, logs::GetLogsRequest, txs::GetTxsRequest,
            uniswap_v2::GetPairsRequest, uniswap_v3::GetPoolsRequest,
        },
        types::format::Format,
    },
    provider::{BtcProvider, CurveProvider, Erc20Provider, FuelProvider},
    requests::{
        btc::{GetBtcBlocksRequest, GetBtcTxsRequest},
        erc20::{GetErc20ApprovalsRequest, GetErc20Request, GetErc20TransferssRequest},
        fuel::{
            GetFuelBlocksRequest, GetFuelLogsRequest, GetFuelReceiptsRequest, GetFuelTxsRequest,
            GetSparkOrderRequest, GetUtxoRequest,
        },
        transfers::GetTransfersRequest,
    },
    ChainId,
};

const API_PATH: &str = "v1/api/";

pub struct HttpProvider {
    inner: reqwest::Client,
    base_url: reqwest::Url,
}

impl HttpProvider {
    async fn request<R>(
        &self,
        url: reqwest::Url,
        request: R,
        format: Format,
    ) -> StreamResponse<Vec<u8>>
    where
        R: serde::Serialize,
    {
        let raw_data_stream = self
            .inner
            .get(url)
            .query(&request)
            .query(&[("format", format)])
            .send()
            .await?
            // .error_for_status()?
            .bytes_stream()
            .map_err(Error::from)
            .map_ok(|bytes| bytes.to_vec())
            .boxed();

        Ok(raw_data_stream)
    }

    fn url(&self, path: &str) -> Result<reqwest::Url> {
        self.base_url.join(path).map_err(Error::from)
    }
}

const STATUS_PATH: &str = "status";

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
            "{}://{endpoint}/{API_PATH}",
            if is_secure { "https" } else { "http" }
        ))?;

        let inner = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .https_only(is_secure)
            .build()?;

        Ok(Self { inner, base_url })
    }

    async fn get_status_by_format(&self, format: Format) -> StreamResponse<Vec<u8>> {
        let url = self.url(STATUS_PATH)?;
        self.request(url, (), format).await
    }
}

const ETHEREUM_BLOCKS_PATH: &str = "blocks";
const ETHEREUM_LOGS_PATH: &str = "logs";
const ETHEREUM_TRANSACTIONS_PATH: &str = "transactions";
const ETHEREUM_TRANSFERS_PATH: &str = "transfers";

#[async_trait]
impl ChainProvider for HttpProvider {
    async fn get_blocks_by_format(
        &self,
        request: GetBlocksRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ETHEREUM_BLOCKS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_logs_by_format(
        &self,
        request: GetLogsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ETHEREUM_LOGS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_txs_by_format(
        &self,
        request: GetTxsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ETHEREUM_TRANSACTIONS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_transfers_by_format(
        &self,
        request: GetTransfersRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ETHEREUM_TRANSFERS_PATH)?;
        self.request(url, request, format).await
    }
}

const UNISWAP_V2_PAIRS_PATH: &str = "uniswap/v2/pairs";
const UNISWAP_V2_PRICES_PATH: &str = "uniswap/v2/prices";

#[async_trait]
impl UniswapV2Provider for HttpProvider {
    async fn get_pairs_by_format(
        &self,
        request: GetPairsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(UNISWAP_V2_PAIRS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_prices_by_format(
        &self,
        request: requests::uniswap_v2::GetPricesRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(UNISWAP_V2_PRICES_PATH)?;
        self.request(url, request, format).await
    }
}

const UNISWAP_V3_POOLS_PATH: &str = "uniswap/v3/pools";
const UNISWAP_V3_PRICES_PATH: &str = "uniswap/v3/prices";

#[async_trait]
impl UniswapV3Provider for HttpProvider {
    async fn get_pools_by_format(
        &self,
        request: GetPoolsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(UNISWAP_V3_POOLS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_prices_by_format(
        &self,
        request: requests::uniswap_v3::GetPricesRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(UNISWAP_V3_PRICES_PATH)?;
        self.request(url, request, format).await
    }
}

const CURVE_TOKENS_PATH: &str = "curve/tokens";
const CURVE_POOLS_PATH: &str = "curve/pools";
const CURVE_PRICES_PATH: &str = "curve/prices";

#[async_trait]
impl CurveProvider for HttpProvider {
    async fn get_tokens_by_format(
        &self,
        request: requests::curve::GetCrvTokenRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(CURVE_TOKENS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_pools_by_format(
        &self,
        request: requests::curve::GetCrvPoolRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(CURVE_POOLS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_prices_by_format(
        &self,
        request: requests::curve::GetCrvPriceRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(CURVE_PRICES_PATH)?;
        self.request(url, request, format).await
    }
}

const ERC20_TOKENS_PATH: &str = "erc20";
const ERC20_APPROVALS_PATH: &str = "erc20/approvals";
const ERC20_TRANSFERS_PATH: &str = "erc20/transfers";

#[async_trait]
impl Erc20Provider for HttpProvider {
    async fn get_erc20_by_format(
        &self,
        request: GetErc20Request,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ERC20_TOKENS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_erc20_approval_by_format(
        &self,
        request: GetErc20ApprovalsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ERC20_APPROVALS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_erc20_transfers_by_format(
        &self,
        request: GetErc20TransferssRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        let url = self.url(ERC20_TRANSFERS_PATH)?;
        self.request(url, request, format).await
    }
}

const FUEL_BLOCKS_PATH: &str = "blocks";
const FUEL_LOGS_PATH: &str = "logs";
const FUEL_TRANSACTIONS_PATH: &str = "transactions";
const FUEL_UNSPENT_UTXOS_PATH: &str = "transactions/outputs";
const FUEL_RECEIPTS_PATH: &str = "receipts";
const FUEL_SPARK_ORDER_PATH: &str = "spark/orders";

#[async_trait]
impl FuelProvider for HttpProvider {
    async fn get_fuel_blocks_by_format(
        &self,
        mut request: GetFuelBlocksRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::FUEL]);
        let url = self.url(FUEL_BLOCKS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_logs_by_format(
        &self,
        mut request: GetFuelLogsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::FUEL]);
        let url = self.url(FUEL_LOGS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_txs_by_format(
        &self,
        mut request: GetFuelTxsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::FUEL]);
        let url = self.url(FUEL_TRANSACTIONS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_receipts_by_format(
        &self,
        mut request: GetFuelReceiptsRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::FUEL]);
        let url = self.url(FUEL_RECEIPTS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_unspent_utxos_by_format(
        &self,
        mut request: GetUtxoRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::FUEL]);
        let url = self.url(FUEL_UNSPENT_UTXOS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_fuel_spark_orders_by_format(
        &self,
        mut request: GetSparkOrderRequest,
        format: Format,
        _: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::FUEL]);
        let url = self.url(FUEL_SPARK_ORDER_PATH)?;
        self.request(url, request, format).await
    }
}

const BTC_BLOCKS_PATH: &str = "blocks";
const BTC_TRANSACTIONS_PATH: &str = "transactions";
#[async_trait]
impl BtcProvider for HttpProvider {
    async fn get_btc_blocks_by_format(
        &self,
        mut request: GetBtcBlocksRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::BTC]);
        let url = self.url(BTC_BLOCKS_PATH)?;
        self.request(url, request, format).await
    }

    async fn get_btc_txs_by_format(
        &self,
        mut request: GetBtcTxsRequest,
        format: Format,
        _deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::BTC]);
        let url = self.url(BTC_TRANSACTIONS_PATH)?;
        self.request(url, request, format).await
    }
}
