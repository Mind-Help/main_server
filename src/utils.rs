use std::{fmt::Display, str::FromStr};

use serde::{de, Deserialize, Deserializer};

pub fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
	S: FromStr,      // Required for S::from_str...
	S::Err: Display, // Required for .map_err(de::Error::custom)
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	S::from_str(&s).map_err(de::Error::custom)
}
