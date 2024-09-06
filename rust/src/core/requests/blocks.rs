use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    core::types::{default_chains, ChainId},
    query::Bound,
    utils::serialize_comma_separated,
};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetBlocksRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    // Inclusive lower bound if is Some for block timestamp
    #[serde(default)]
    pub from_timestamp: Option<i64>,
    // Exclusive upper bound if is Some for block timestamp
    #[serde(default)]
    pub to_timestamp: Option<i64>,
}

impl Default for GetBlocksRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            from_timestamp: None,
            to_timestamp: None,
        }
    }
}
