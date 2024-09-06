use async_trait::async_trait;
use futures::StreamExt;

use super::{
    error::ResponseError,
    provider::{
        BtcProvider, ChainProvider, Provider, StreamResponse, UniswapV2Provider, UniswapV3Provider,
    },
    requests::{
        self,
        blocks::GetBlocksRequest,
        btc::{GetBtcBlocksRequest, GetBtcTxsRequest},
        fuel::{GetFuelReceiptsRequest, GetUtxoRequest},
        logs::GetLogsRequest,
        txs::GetTxsRequest,
        uniswap_v2::GetPairsRequest,
        uniswap_v3::GetPoolsRequest,
    },
    types::{format::Format, status::Status},
};
use crate::{
    provider::{CurveProvider, Erc20Provider, FuelProvider},
    requests::{
        curve::{GetCrvPoolRequest, GetCrvPriceRequest, GetCrvTokenRequest},
        erc20::{GetErc20ApprovalsRequest, GetErc20Request, GetErc20TransferssRequest},
        fuel::{GetFuelBlocksRequest, GetFuelLogsRequest, GetFuelTxsRequest, GetSparkOrderRequest},
        transfers::GetTransfersRequest,
    },
};

pub struct Client<T> {
    inner: T,
}

impl<T> Client<T>
where
    T: Provider,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub async fn get_status(&self) -> StreamResponse<Status> {
        let raw_data_stream = self.inner.get_status_by_format(Format::JsonStream).await?;
        let raw_data_stream = ResponseError::map_stream(raw_data_stream);

        let records = raw_data_stream
            .map(|chunk_result| {
                chunk_result.and_then(|chunk| Ok(serde_json::from_slice::<Status>(&chunk)?))
            })
            .boxed();

        Ok(records)
    }
}

#[async_trait]
impl<T> ChainProvider for Client<T>
where
    T: ChainProvider + Send + Sync,
{
    async fn get_blocks_by_format(
        &self,
        request: GetBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_blocks_by_format(request, format, deltas)
            .await
    }

    async fn get_logs_by_format(
        &self,
        request: GetLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner.get_logs_by_format(request, format, deltas).await
    }

    async fn get_txs_by_format(
        &self,
        request: GetTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner.get_txs_by_format(request, format, deltas).await
    }

    async fn get_transfers_by_format(
        &self,
        request: GetTransfersRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_transfers_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> UniswapV2Provider for Client<T>
where
    T: UniswapV2Provider + Send + Sync,
{
    async fn get_pairs_by_format(
        &self,
        request: GetPairsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_pairs_by_format(request, format, deltas)
            .await
    }

    async fn get_prices_by_format(
        &self,
        request: requests::uniswap_v2::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_prices_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> UniswapV3Provider for Client<T>
where
    T: UniswapV3Provider + Send + Sync,
{
    async fn get_pools_by_format(
        &self,
        request: GetPoolsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_pools_by_format(request, format, deltas)
            .await
    }

    async fn get_prices_by_format(
        &self,
        request: requests::uniswap_v3::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_prices_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> CurveProvider for Client<T>
where
    T: CurveProvider + Send + Sync,
{
    async fn get_tokens_by_format(
        &self,
        request: GetCrvTokenRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_tokens_by_format(request, format, deltas)
            .await
    }

    async fn get_pools_by_format(
        &self,
        request: GetCrvPoolRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_pools_by_format(request, format, deltas)
            .await
    }

    async fn get_prices_by_format(
        &self,
        request: GetCrvPriceRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_prices_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> Erc20Provider for Client<T>
where
    T: Erc20Provider + Send + Sync,
{
    async fn get_erc20_by_format(
        &self,
        request: GetErc20Request,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_erc20_by_format(request, format, deltas)
            .await
    }

    async fn get_erc20_approval_by_format(
        &self,
        request: GetErc20ApprovalsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_erc20_approval_by_format(request, format, deltas)
            .await
    }

    async fn get_erc20_transfers_by_format(
        &self,
        request: GetErc20TransferssRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_erc20_transfers_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> FuelProvider for Client<T>
where
    T: FuelProvider + Send + Sync,
{
    async fn get_fuel_blocks_by_format(
        &self,
        request: GetFuelBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_fuel_blocks_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_logs_by_format(
        &self,
        request: GetFuelLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_fuel_logs_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_txs_by_format(
        &self,
        request: GetFuelTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_fuel_txs_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_receipts_by_format(
        &self,
        request: GetFuelReceiptsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_fuel_receipts_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_unspent_utxos_by_format(
        &self,
        request: GetUtxoRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_fuel_unspent_utxos_by_format(request, format, deltas)
            .await
    }

    async fn get_fuel_spark_orders_by_format(
        &self,
        request: GetSparkOrderRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_fuel_spark_orders_by_format(request, format, deltas)
            .await
    }
}

#[async_trait]
impl<T> BtcProvider for Client<T>
where
    T: BtcProvider + Send + Sync,
{
    async fn get_btc_blocks_by_format(
        &self,
        request: GetBtcBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_btc_blocks_by_format(request, format, deltas)
            .await
    }

    async fn get_btc_txs_by_format(
        &self,
        request: GetBtcTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.inner
            .get_btc_txs_by_format(request, format, deltas)
            .await
    }
}
