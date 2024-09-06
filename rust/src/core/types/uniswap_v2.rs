use serde::{Deserialize, Serialize};
use strum::AsRefStr;

/// LpEvent is the event type of uniswap v2
#[derive(AsRefStr, Clone, Copy, Debug, Default, Hash, Deserialize, Serialize, PartialEq, Eq)]
pub enum ReserveEvent {
    #[serde(alias = "mint")]
    Mint = 0,
    #[serde(alias = "burn")]
    Burn = 1,
    #[serde(alias = "swap")]
    Swap = 2,
    #[default]
    #[serde(alias = "sync")]
    Sync = 3,
}

impl ReserveEvent {
    pub const fn as_str(&self) -> &'static str {
        match self {
            ReserveEvent::Mint => "mint",
            ReserveEvent::Burn => "burn",
            ReserveEvent::Swap => "swap",
            ReserveEvent::Sync => "sync",
        }
    }
}

impl TryFrom<i32> for ReserveEvent {
    type Error = crate::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(ReserveEvent::Mint),
            1 => Ok(ReserveEvent::Burn),
            2 => Ok(ReserveEvent::Swap),
            3 => Ok(ReserveEvent::Sync),
            _ => Err(crate::Error::UnknownReserveEvent(v)),
        }
    }
}
