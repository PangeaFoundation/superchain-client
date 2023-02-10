use ethers::types::{
    Address,
    H256,
    U256,
};
use serde::Deserialize;

/// A uniswap v3 `Pool` baed on Pool creation event
#[derive(Clone, Debug, Deserialize)]
pub struct Pool {
    pub block_number: u64,
    pub address: Address,
    pub transaction_hash: H256,
    pub transaction_index: u64,
    pub timestamp: i64,
    pub token0: Address,
    pub token1: Address,
    pub fee: i32,
    pub tick_spacing: i32,
    pub tick: i32,
    pub price: f64,
}

/// A uniswap v3 price baed on events
#[derive(Clone, Debug, Deserialize)]
pub struct Price {
    pub block_number: u64,
    pub pool: Address,
    pub sender: Address,
    pub recipient: Address,
    pub amount0: U256,
    pub amount1: U256,
    pub price: f64,
    pub liquidity: f64,
    pub liquidity_raw: u128,
    pub tick: i32,
    pub virtual0: f64,
    pub virtual1: f64,
    pub virtual1_raw: U256,
    pub timestamp: i64,
    pub transaction_hash: H256,
    pub transaction_index: u64,
}
