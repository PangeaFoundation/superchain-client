use ethers::types::U256;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockHeader {
    pub hash: U256,
    pub block_number: u64,
    pub timestamp: i64,
}
