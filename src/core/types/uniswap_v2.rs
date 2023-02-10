use ethers::types::{
    Address,
    H256,
    U128,
    U256,
};
use serde::{
    Deserialize,
    Serialize,
};
use serde_repr::Deserialize_repr;

/// A uniswap v2 `Pair` baed on `PairCreated` event
/// <https://docs.uniswap.org/protocol/V2/reference/smart-contracts/factory#paircreated>
#[derive(Clone, Debug, Deserialize)]
pub struct Pair {
    pub block_number: u64,
    // factory address
    pub address: Address,
    pub transaction_hash: H256,
    pub transaction_index: i64,
    pub token0: Address,
    pub token1: Address,
    pub pair: Address,
    pub pair_index: U256,
    pub timestamp: i64,
}

/// LpEvent is the event type of uniswap v2
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize_repr)]
#[repr(u8)]
pub enum LpEvent {
    Mint = 0,
    Burn = 1,
    Swap = 2,
    #[default]
    Sync = 3,
}

/// A uniswap v2 price baed on events
#[derive(Clone, Debug, Deserialize)]
pub struct Price {
    pub block_number: u64,
    pub address: Address,
    pub transaction_hash: H256,
    pub transaction_index: i64,
    pub event: LpEvent,
    // actually u112
    pub reserve0: U128,
    // actually u112
    pub reserve1: U128,
    pub price: f64,
    pub sender: Option<Address>,
    pub receiver: Option<Address>,
    pub amount0: Option<U256>,
    pub amount1: Option<U256>,
    // pub lp_amount: Option<f64>,  fixme: current deploy version have some bug
    pub protocol_fee: Option<U256>,
    pub factory: Address,
    pub pool_pair_block_number: u64,
    pub pool_pair_transaction_hash: H256,
    pub pool_pair_transaction_index: u64,
    pub pool_pair_timestamp: i64,
    pub token0: Address,
    pub token1: Address,
    pub token0_decimals: u8,
    pub token1_decimals: u8,
    pub token0_name: String,
    pub token1_name: String,
    pub token0_symbol: String,
    pub token1_symbol: String,
    pub timestamp: i64,
}

/// The direction of transaction
#[derive(Clone, Copy, Debug, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum Side {
    Buy,
    Sell,
}

/// A uniswap v2 reserves baed on events
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Reserves {
    pub block_number: u64,
    pub address: Address,
    pub transaction_hash: H256,
    pub transaction_index: u64,
    pub timestamp: i64,
    pub event: LpEvent,
    // actually u112
    pub reserve0: U128,
    // actually u112
    pub reserve1: U128,
    pub amount0: Option<U256>,
    pub amount1: Option<U256>,
    pub lp_amount: U256,
    pub sender: Option<Address>,
    pub receiver: Option<Address>,
    pub protocol_fee: Option<U256>,
}

/// A uniswap v2 trade baed on events
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Trade {
    pub block_number: u64,
    pub address: Address,
    pub transaction_hash: H256,
    pub transaction_index: u64,
    pub sender: Address,
    pub receiver: Address,
    pub price: f64,
    pub last_traded_price: f64,
    pub volume0: f64,
    pub volume1: f64,
    pub fixed0: U256,
    pub fixed1: U256,
    pub decimals0: u8,
    pub decimals1: u8,
    pub side: Side,
    pub timestamp: i64,
}
