use std::collections::HashSet;

use ethers_core::types::{Address, U256};
use serde::{Deserialize, Serialize};

use crate::{
    core::types::{default_chains, ChainId},
    query::Bound,
    utils::serialize_comma_separated,
};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetTransfersRequest {
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
    pub address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub to__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub from__in: HashSet<Address>,

    #[serde(default)]
    pub value__lte: Option<U256>,

    #[serde(default)]
    pub value__gte: Option<U256>,
}

impl Default for GetTransfersRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            address__in: HashSet::default(),
            to__in: HashSet::default(),
            from__in: HashSet::default(),
            value__lte: None,
            value__gte: None,
        }
    }
}
