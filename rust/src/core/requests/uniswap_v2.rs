use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use ethers_core::types::{Address, U128};

use crate::{
    core::types::{default_chains, uniswap_v2::ReserveEvent, ChainId},
    query::Bound,
    utils::serialize_comma_separated,
};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetPairsRequest {
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
    pub pair_address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub factory_address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token0__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token1__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub tokens__in: HashSet<Address>,
}

impl Default for GetPairsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pair_address__in: HashSet::new(),
            factory_address__in: HashSet::new(),
            token0__in: HashSet::new(),
            token1__in: HashSet::new(),
            tokens__in: HashSet::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetPricesRequest {
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
    pub pair_address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub pair_factory_address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub event__in: HashSet<ReserveEvent>,

    #[serde(default)]
    pub reserve0__gte: Option<U128>,

    #[serde(default)]
    pub reserve0__lte: Option<U128>,

    #[serde(default)]
    pub reserve1__gte: Option<U128>,

    #[serde(default)]
    pub reserve1__lte: Option<U128>,

    #[serde(default)]
    pub price__gte: Option<f64>,

    #[serde(default)]
    pub price__lte: Option<f64>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub sender__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub receiver__in: HashSet<Address>,

    #[serde(default)]
    pub amount0__gte: Option<f64>,

    #[serde(default)]
    pub amount0__lte: Option<f64>,

    #[serde(default)]
    pub amount1__gte: Option<f64>,

    #[serde(default)]
    pub amount1__lte: Option<f64>,

    #[serde(default)]
    pub lp_amount__gte: Option<f64>,

    #[serde(default)]
    pub lp_amount__lte: Option<f64>,

    #[serde(default)]
    pub protocol_fee__gte: Option<f64>,

    #[serde(default)]
    pub protocol_fee__lte: Option<f64>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token0_address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token0_symbol__in: HashSet<String>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token1_address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token1_symbol__in: HashSet<String>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub tokens_address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub tokens_symbol__in: HashSet<String>,
}

impl Default for GetPricesRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pair_address__in: HashSet::new(),
            pair_factory_address__in: HashSet::new(),
            event__in: HashSet::new(),
            reserve0__gte: None,
            reserve0__lte: None,
            reserve1__gte: None,
            reserve1__lte: None,
            price__gte: None,
            price__lte: None,
            sender__in: HashSet::new(),
            receiver__in: HashSet::new(),
            amount0__gte: None,
            amount0__lte: None,
            amount1__gte: None,
            amount1__lte: None,
            lp_amount__gte: None,
            lp_amount__lte: None,
            protocol_fee__gte: None,
            protocol_fee__lte: None,
            token0_address__in: HashSet::new(),
            token0_symbol__in: HashSet::new(),
            token1_address__in: HashSet::new(),
            token1_symbol__in: HashSet::new(),
            tokens_address__in: HashSet::new(),
            tokens_symbol__in: HashSet::new(),
        }
    }
}
