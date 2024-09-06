use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use ethers_core::types::{Address, U256};

use crate::{
    core::types::{default_chains, ChainId},
    query::Bound,
    utils::serialize_comma_separated,
};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetTxsRequest {
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

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub from__in: HashSet<Address>,
    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub to__in: HashSet<Address>,

    #[serde(default)]
    pub value__gte: Option<U256>,
    #[serde(default)]
    pub value__lte: Option<U256>,

    #[serde(default)]
    pub gas_price__gte: Option<U256>,
    #[serde(default)]
    pub gas_price__lte: Option<U256>,

    #[serde(default)]
    pub gas__gte: Option<U256>,
    #[serde(default)]
    pub gas__lte: Option<U256>,

    #[serde(default)]
    pub max_fee_per_gas__gte: Option<U256>,
    #[serde(default)]
    pub max_fee_per_gas__lte: Option<U256>,

    #[serde(default)]
    pub max_priority_fee_per_gas__gte: Option<U256>,
    #[serde(default)]
    pub max_priority_fee_per_gas__lte: Option<U256>,
}

impl Default for GetTxsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            from__in: HashSet::new(),
            to__in: HashSet::new(),
            value__gte: None,
            value__lte: None,
            gas_price__gte: None,
            gas_price__lte: None,
            gas__gte: None,
            gas__lte: None,
            max_fee_per_gas__gte: None,
            max_fee_per_gas__lte: None,
            max_priority_fee_per_gas__gte: None,
            max_priority_fee_per_gas__lte: None,
        }
    }
}
