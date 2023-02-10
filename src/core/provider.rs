use std::pin::Pin;

use async_trait::async_trait;
use ethers::types::H160;
use futures::Stream;

use super::{
    error::Result,
    types::{
        ethereum::BlockHeader,
        query::QueryOptions,
        uniswap_v2,
        uniswap_v3,
    },
};

pub type StreamResponse<T> = Result<Pin<Box<dyn Stream<Item = Result<T>> + Send>>>;

#[async_trait]
pub trait Provider: Sized {
    async fn try_new(
        endpoint: String,
        is_secure: bool,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Self>;
}

#[async_trait]
pub trait EthereumProvider {
    async fn get_headers(&self, opts: QueryOptions) -> StreamResponse<BlockHeader>;
    async fn get_height(&self) -> Result<u64>;
}

#[async_trait]
pub trait UniswapV2Provider {
    async fn get_pairs(&self, opts: QueryOptions) -> StreamResponse<uniswap_v2::Pair>;
    async fn get_pair(&self, pair: H160) -> Result<uniswap_v2::Pair>;
    async fn get_prices<I>(
        &self,
        pairs: I,
        opts: QueryOptions,
    ) -> StreamResponse<uniswap_v2::Price>
    where
        I: IntoIterator<Item = H160> + Send;
    async fn get_raw_reserves<I>(
        &self,
        pairs: I,
        opts: QueryOptions,
    ) -> StreamResponse<uniswap_v2::Reserves>
    where
        I: IntoIterator<Item = H160> + Send;
    async fn get_trades<I>(&self, pair: I, opts: QueryOptions) -> StreamResponse<uniswap_v2::Trade>
    where
        I: IntoIterator<Item = H160> + Send;
}

#[async_trait]
pub trait UniswapV3Provider {
    async fn get_pools(&self, opts: QueryOptions) -> StreamResponse<uniswap_v3::Pool>;
    async fn get_pool(&self, pool: H160) -> Result<uniswap_v3::Pool>;
    async fn get_prices<I>(
        &self,
        pools: I,
        opts: QueryOptions,
    ) -> StreamResponse<uniswap_v3::Price>
    where
        I: IntoIterator<Item = H160> + Send;
}
