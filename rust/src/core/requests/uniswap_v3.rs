use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use ethers_core::types::Address;

use crate::{
    core::types::{default_chains, ChainId},
    query::Bound,
    utils::serialize_comma_separated,
};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetPoolsRequest {
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
    pub pool_address__in: HashSet<Address>,

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

    #[serde(default)]
    pub fee__gte: Option<i32>,
    #[serde(default)]
    pub fee__lte: Option<i32>,

    #[serde(default)]
    pub tick__gte: Option<i32>,
    #[serde(default)]
    pub tick__lte: Option<i32>,

    #[serde(default)]
    pub price__gte: Option<f64>,
    #[serde(default)]
    pub price__lte: Option<f64>,

    #[serde(default)]
    pub tick_spacing__gte: Option<i32>,
    #[serde(default)]
    pub tick_spacing__lte: Option<i32>,
}

impl Default for GetPoolsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pool_address__in: HashSet::default(),
            factory_address__in: HashSet::default(),
            token0__in: HashSet::default(),
            token1__in: HashSet::default(),
            tokens__in: HashSet::default(),
            fee__gte: None,
            fee__lte: None,
            tick__gte: None,
            tick__lte: None,
            price__gte: None,
            price__lte: None,
            tick_spacing__gte: None,
            tick_spacing__lte: None,
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
    pub pool_address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub pool_factory_address__in: HashSet<Address>,

    #[serde(default)]
    pub virtual0__gte: Option<f64>,

    #[serde(default)]
    pub virtual0__lte: Option<f64>,

    #[serde(default)]
    pub virtual1__gte: Option<f64>,

    #[serde(default)]
    pub virtual1__lte: Option<f64>,

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
    pub liquidity__gte: Option<f64>,

    #[serde(default)]
    pub liquidity__lte: Option<f64>,

    #[serde(default)]
    pub tick__gte: Option<i32>,

    #[serde(default)]
    pub tick__lte: Option<i32>,

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
            price__gte: None,
            price__lte: None,
            sender__in: HashSet::new(),
            receiver__in: HashSet::new(),
            amount0__gte: None,
            amount0__lte: None,
            amount1__gte: None,
            amount1__lte: None,
            token0_address__in: HashSet::new(),
            token0_symbol__in: HashSet::new(),
            token1_address__in: HashSet::new(),
            token1_symbol__in: HashSet::new(),
            tokens_address__in: HashSet::new(),
            tokens_symbol__in: HashSet::new(),
            pool_address__in: HashSet::new(),
            pool_factory_address__in: HashSet::new(),
            virtual0__gte: None,
            virtual0__lte: None,
            virtual1__gte: None,
            virtual1__lte: None,
            liquidity__gte: None,
            liquidity__lte: None,
            tick__gte: None,
            tick__lte: None,
        }
    }
}
