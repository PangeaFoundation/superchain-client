use async_trait::async_trait;
use ethers::types::H160;

use super::{
    error::Result,
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

pub struct Client<T> {
    inner: T,
}

impl<T> Client<T>
where
    T: Provider + Send,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<T> EthereumProvider for Client<T>
where
    T: EthereumProvider + Send + Sync,
{
    async fn get_headers(&self, opts: QueryOptions) -> StreamResponse<BlockHeader> {
        self.inner.get_headers(opts).await
    }

    async fn get_height(&self) -> Result<u64> {
        self.inner.get_height().await
    }
}

#[async_trait]
impl<T> UniswapV2Provider for Client<T>
where
    T: UniswapV2Provider + Send + Sync,
{
    async fn get_pairs(&self, opts: QueryOptions) -> StreamResponse<uniswap_v2::Pair> {
        self.inner.get_pairs(opts).await
    }

    async fn get_pair(&self, pair: H160) -> Result<uniswap_v2::Pair> {
        self.inner.get_pair(pair).await
    }

    async fn get_prices<I>(&self, pairs: I, opts: QueryOptions) -> StreamResponse<uniswap_v2::Price>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        self.inner.get_prices(pairs, opts).await
    }

    async fn get_raw_reserves<I>(
        &self,
        pairs: I,
        opts: QueryOptions,
    ) -> StreamResponse<uniswap_v2::Reserves>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        self.inner.get_raw_reserves(pairs, opts).await
    }

    async fn get_trades<I>(&self, pairs: I, opts: QueryOptions) -> StreamResponse<uniswap_v2::Trade>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        self.inner.get_trades(pairs, opts).await
    }
}

#[async_trait]
impl<T> UniswapV3Provider for Client<T>
where
    T: UniswapV3Provider + Send + Sync,
{
    async fn get_pools(&self, opts: QueryOptions) -> StreamResponse<uniswap_v3::Pool> {
        self.inner.get_pools(opts).await
    }

    async fn get_pool(&self, pool: H160) -> Result<uniswap_v3::Pool> {
        self.inner.get_pool(pool).await
    }

    async fn get_prices<I>(&self, pools: I, opts: QueryOptions) -> StreamResponse<uniswap_v3::Price>
    where
        I: IntoIterator<Item = H160> + Send,
    {
        self.inner.get_prices(pools, opts).await
    }
}
