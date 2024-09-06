use std::{collections::HashSet, fmt, str::FromStr};

use num_derive::FromPrimitive as FromPrimitiveDerive;
use num_traits::FromPrimitive;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use strum::{AsRefStr, EnumString};

pub mod format;
pub mod fuel;
pub mod query;
pub mod status;
pub mod uniswap_v2;

#[derive(
    AsRefStr, EnumString, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, FromPrimitiveDerive,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum ChainId {
    #[strum(to_string = "none")]
    Any = 0,
    #[strum(to_string = "ETH")]
    #[default]
    ETH = 1,
    #[strum(to_string = "OPT")]
    OPT = 10,
    #[strum(to_string = "BNB")]
    BNB = 56,
    #[strum(to_string = "FUEL")]
    FUEL = 122,
    #[strum(to_string = "MATIC")]
    MATIC = 137,
    #[strum(to_string = "BTC")]
    BTC = 198,
    #[strum(to_string = "MOVE")]
    MEVM = 336,
    #[strum(to_string = "ARB")]
    ARB = 42161,
    #[strum(to_string = "AVAX")]
    AVAX = 43114,
    #[strum(to_string = "SEPETH")]
    SEPETH = 1115511,
}

impl ChainId {
    pub fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    pub fn chain_code(&self) -> String {
        match self {
            Self::Any => "ANY".to_string(),
            Self::ETH => "ETH".to_string(),
            Self::AVAX => "AVAX".to_string(),
            Self::BNB => "BNB".to_string(),
            Self::OPT => "OPT".to_string(),
            Self::ARB => "ARB".to_string(),
            Self::MATIC => "MATIC".to_string(),
            Self::FUEL => "FUEL".to_string(),
            Self::MEVM => "MEVM".to_string(),
            Self::SEPETH => "SEPETH".to_string(),
            Self::BTC => "BTC".to_string(),
        }
    }

    pub fn chain_name(&self) -> String {
        match self {
            Self::Any => "Any".to_string(),
            Self::ETH => "Ethereum".to_string(),
            Self::AVAX => "Avalanche".to_string(),
            Self::BNB => "Binance Smart Chain".to_string(),
            Self::OPT => "Optimism".to_string(),
            Self::ARB => "Arbitrum".to_string(),
            Self::MATIC => "Polygon".to_string(),
            Self::FUEL => "Fuel".to_string(),
            Self::MEVM => "MEVM".to_string(),
            Self::SEPETH => "Sepior".to_string(),
            Self::BTC => "Bitcoin".to_string(),
        }
    }
}

impl Serialize for ChainId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.chain_code().as_str())
    }
}

struct ChainIdVisitor;

impl<'de> Visitor<'de> for ChainIdVisitor {
    type Value = ChainId;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a valid ChainId as an integer or string")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ChainId::from_i32(value).ok_or_else(|| serde::de::Error::custom("Invalid ChainId value"))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ChainId::from_i32(value as i32)
            .ok_or_else(|| serde::de::Error::custom("Invalid ChainId value"))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ChainId::from_i32(value as i32)
            .ok_or_else(|| serde::de::Error::custom("Invalid ChainId value"))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ChainId::from_i32(value as i32)
            .ok_or_else(|| serde::de::Error::custom("Invalid ChainId value"))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ChainId::from_i32(v as i32).ok_or_else(|| serde::de::Error::custom("Invalid ChainId value"))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ChainId::from_i32(v as i32).ok_or_else(|| serde::de::Error::custom("Invalid ChainId value"))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(int_value) = value.parse::<i32>() {
            return ChainId::from_i32(int_value)
                .ok_or_else(|| serde::de::Error::custom("Invalid ChainId value"));
        }

        ChainId::from_str(&value).map_err(|_| serde::de::Error::custom("Invalid ChainId value"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(int_value) = value.parse::<i32>() {
            return ChainId::from_i32(int_value)
                .ok_or_else(|| serde::de::Error::custom("Invalid ChainId value"));
        }

        ChainId::from_str(value).map_err(|_| serde::de::Error::custom("Invalid ChainId value"))
    }
}

impl<'de> Deserialize<'de> for ChainId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ChainIdVisitor)
    }
}

pub fn default_chains() -> HashSet<ChainId> {
    HashSet::from([ChainId::ETH])
}
