use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Bound {
    Exact(u64),
    #[default]
    Latest,
    None,
}

impl serde::Serialize for Bound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Exact(n) => serializer.serialize_u64(*n),
            Self::Latest => serializer.serialize_str("latest"),
            Self::None => serializer.serialize_str("none"),
        }
    }
}

/// Many queries can take a block range. To see how to use this as a parameter,
/// see some examples.
#[derive(Clone, Debug, Default, Serialize)]
#[non_exhaustive]
pub struct QueryOptions {
    pub start: Bound,
    pub end: Bound,
}

impl QueryOptions {
    pub fn start(self, value: Bound) -> Self {
        Self {
            start: value,
            ..self
        }
    }
    pub fn end(self, value: Bound) -> Self {
        Self { end: value, ..self }
    }
}
