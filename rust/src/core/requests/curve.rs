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
pub struct GetCrvTokenRequest {
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
    pub symbol__in: HashSet<String>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub name__in: HashSet<String>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub pool_address__in: HashSet<Address>,

    #[serde(default)]
    pub decimals__gte: Option<u8>,

    #[serde(default)]
    pub decimals__lte: Option<u8>,
}

impl Default for GetCrvTokenRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            address__in: HashSet::new(),
            symbol__in: HashSet::new(),
            name__in: HashSet::new(),
            pool_address__in: HashSet::new(),
            decimals__gte: None,
            decimals__lte: None,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetCrvPoolRequest {
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
    pub token__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub owner__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub base_pool__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub coins__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub base_coins__in: HashSet<Address>,

    #[serde(default)]
    pub fee__gte: Option<U256>,

    #[serde(default)]
    pub fee__lte: Option<U256>,

    #[serde(default)]
    pub admin_fee__gte: Option<U256>,

    #[serde(default)]
    pub admin_fee__lte: Option<U256>,

    #[serde(default)]
    pub initial_a__gte: Option<U256>,

    #[serde(default)]
    pub initial_a__lte: Option<U256>,

    #[serde(default)]
    pub future_a__gte: Option<U256>,

    #[serde(default)]
    pub future_a__lte: Option<U256>,

    #[serde(default)]
    pub initial_a_time__gte: Option<U256>,

    #[serde(default)]
    pub initial_a_time__lte: Option<U256>,

    #[serde(default)]
    pub future_a_time__gte: Option<U256>,

    #[serde(default)]
    pub future_a_time__lte: Option<U256>,

    #[serde(default)]
    pub n_coins__gte: Option<u8>,

    #[serde(default)]
    pub n_coins__lte: Option<u8>,
}

impl Default for GetCrvPoolRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pool_address__in: HashSet::new(),
            token__in: HashSet::new(),
            owner__in: HashSet::new(),
            base_pool__in: HashSet::new(),
            coins__in: HashSet::new(),
            base_coins__in: HashSet::new(),
            fee__gte: None,
            fee__lte: None,
            admin_fee__gte: None,
            admin_fee__lte: None,
            initial_a__gte: None,
            initial_a__lte: None,
            future_a__gte: None,
            future_a__lte: None,
            initial_a_time__gte: None,
            initial_a_time__lte: None,
            future_a_time__gte: None,
            future_a_time__lte: None,
            n_coins__gte: None,
            n_coins__lte: None,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetCrvPriceRequest {
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
    pub buyer__in: HashSet<Address>,

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

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub sold_address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub sold_symbol__in: HashSet<String>,

    #[serde(default)]
    pub sold_decimals__gte: Option<u8>,

    #[serde(default)]
    pub sold_decimals__lte: Option<u8>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub bought_address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub bought_symbol__in: HashSet<String>,

    #[serde(default)]
    pub bought_decimals__gte: Option<u8>,

    #[serde(default)]
    pub bought_decimals__lte: Option<u8>,

    #[serde(default)]
    pub price__gte: Option<f64>,

    #[serde(default)]
    pub price__lte: Option<f64>,

    #[serde(default)]
    pub tokens_sold__gte: Option<f64>,

    #[serde(default)]
    pub tokens_sold__lte: Option<f64>,

    #[serde(default)]
    pub tokens_bought__gte: Option<f64>,

    #[serde(default)]
    pub tokens_bought__lte: Option<f64>,
}

impl Default for GetCrvPriceRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pool_address__in: HashSet::new(),
            buyer__in: HashSet::new(),
            tokens_address__in: HashSet::new(),
            tokens_symbol__in: HashSet::new(),
            sold_address__in: HashSet::new(),
            sold_symbol__in: HashSet::new(),
            sold_decimals__gte: None,
            sold_decimals__lte: None,
            bought_address__in: HashSet::new(),
            bought_symbol__in: HashSet::new(),
            bought_decimals__gte: None,
            bought_decimals__lte: None,
            price__gte: None,
            price__lte: None,
            tokens_sold__gte: None,
            tokens_sold__lte: None,
            tokens_bought__gte: None,
            tokens_bought__lte: None,
        }
    }
}
