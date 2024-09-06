use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(AsRefStr, Clone, Copy, Debug, Default, Hash, Deserialize, Serialize, PartialEq, Eq)]
pub enum TransactionType {
    #[default]
    #[serde(alias = "unknown")]
    Unknown = 0,
    #[serde(alias = "script")]
    Script = 1,
    #[serde(alias = "create")]
    Create = 2,
    #[serde(alias = "mint")]
    Mint = 3,
    #[serde(alias = "upgrade")]
    Upgrade = 4,
    #[serde(alias = "upload")]
    Upload = 5,
}

impl TransactionType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Unknown => "unknown",
            TransactionType::Script => "script",
            TransactionType::Create => "create",
            TransactionType::Mint => "mint",
            TransactionType::Upgrade => "upgrade",
            TransactionType::Upload => "upload",
        }
    }
}

impl TryFrom<i32> for TransactionType {
    type Error = crate::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(TransactionType::Unknown),
            1 => Ok(TransactionType::Script),
            2 => Ok(TransactionType::Create),
            3 => Ok(TransactionType::Mint),
            4 => Ok(TransactionType::Upgrade),
            5 => Ok(TransactionType::Upload),
            _ => Err(crate::Error::UnknownTransactionType(v)),
        }
    }
}

#[derive(AsRefStr, Clone, Copy, Debug, Default, Hash, Deserialize, Serialize, PartialEq, Eq)]
pub enum OrderChangeType {
    #[default]
    #[serde(alias = "open")]
    Open = 0,
    #[serde(alias = "cancel")]
    Cancel = 1,
    #[serde(alias = "match")]
    Match = 2,
}

#[derive(AsRefStr, Clone, Copy, Debug, Default, Hash, Deserialize, Serialize, PartialEq, Eq)]
pub enum OrderType {
    #[default]
    #[serde(alias = "buy")]
    Buy = 0,
    #[serde(alias = "sell")]
    Sell = 1,
}

#[derive(AsRefStr, Clone, Copy, Debug, Default, Hash, Deserialize, Serialize, PartialEq, Eq)]
pub enum ReceiptType {
    #[default]
    Call = 0,
    Return = 1,
    ReturnData = 2,
    Panic = 3,
    Revert = 4,
    Log = 5,
    LogData = 6,
    Transfer = 7,
    TransferOut = 8,
    ScriptResult = 9,
    MessageOut = 10,
    Mint = 11,
    Burn = 12,
}
