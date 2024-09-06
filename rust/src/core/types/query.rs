use std::ops::Mul;

use regex::Regex;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Bound {
    /// The range starts/ends at this block height (inclusive)
    Exact(i64),
    /// The range should start/end at the latest block height
    #[default]
    Latest,
    // The bound is defined from the latest block height
    FromLatest(u64),
    /// Real-time 
    Subscribe,
}

impl Bound {
    const LATEST: &'static str = "latest";
    const NONE: &'static str = "none";

    pub const fn none() -> Self {
        Self::Subscribe
    }
}

impl PartialOrd for Bound {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match (self, other) {
            (Self::Exact(lhs), Self::Exact(rhs)) => lhs.partial_cmp(rhs),
            (Self::FromLatest(lhs), Self::FromLatest(rhs)) => lhs.partial_cmp(rhs),
            (Self::Latest, Self::Latest) => Some(Ordering::Equal),
            (Self::Subscribe, Self::Subscribe) => None,
            (_, Self::Subscribe) => Some(Ordering::Less),
            _ => None,
        }
    }
}

impl serde::Serialize for Bound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Exact(n) => serializer.serialize_i64(*n),
            Self::FromLatest(n) => serializer.serialize_i64(-(*n as i64)),
            Self::Latest => serializer.serialize_str(Self::LATEST),
            Self::Subscribe => serializer.serialize_str(Self::NONE),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Bound {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Bound;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("A block range bound")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let re = Regex::new(r#"^latest\s*-\s*(\d+)$"#).unwrap();
                let from_latest = re.captures(v).and_then(|c| c.get(1).map(|m| m.as_str()));

                match v {
                    Bound::LATEST => return Ok(Bound::Latest),
                    Bound::NONE => return Ok(Bound::Subscribe),
                    _ => {}
                }

                if let Some(from_latest) = from_latest {
                    if let Ok(from_latest) = from_latest.parse::<u64>() {
                        if from_latest > 0 {
                            return Ok(Bound::FromLatest(from_latest));
                        }
                    }
                }

                if let Ok(exact) = v.parse::<i64>() {
                    return Ok(Bound::Exact(exact));
                }

                if let Ok(exact) = v.parse::<i64>() {
                    if exact < 0 {
                        return Ok(Bound::FromLatest(exact.unsigned_abs()));
                    }
                }

                Err(E::invalid_value(serde::de::Unexpected::Str(v), &self))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v < 0 {
                    Ok(Bound::FromLatest(v.unsigned_abs()))
                } else {
                    Ok(Bound::Exact(v))
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl std::fmt::Debug for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exact(n) => std::fmt::Debug::fmt(n, f),
            Self::FromLatest(n) => std::fmt::Debug::fmt(&(*n as i64).mul(-1), f),
            Self::Latest => f.write_str(Self::LATEST),
            Self::Subscribe => f.write_str(Self::NONE),
        }
    }
}
