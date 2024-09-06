use std::collections::HashSet;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use ethers_core::types::Address;

pub fn serialize_comma_separated<S, T, I>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: IntoIterator<Item = I>,
    I: Serialize,
{
    let mut buffer = String::new();
    for (i, item) in value.into_iter().enumerate() {
        if i > 0 {
            buffer.push(',');
        }
        buffer.push_str(
            serde_json::to_string(&item)
                .unwrap()
                .trim_start_matches('"')
                .trim_end_matches('"'),
        );
    }
    if !buffer.is_empty() {
        return serializer.serialize_str(&buffer);
    }

    serializer.serialize_none()
}

pub fn deserialize_addresses<'de, D>(deserializer: D) -> Result<HashSet<Address>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let mut addresses = HashSet::new();
    for address in s.split(";;") {
        addresses.insert(address.parse().unwrap());
    }
    Ok(addresses)
}
