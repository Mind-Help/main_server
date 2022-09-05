use std::str::FromStr;

use async_graphql::{InputValueResult, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::deserialize_from_str;

#[derive(Serialize, Deserialize)]
pub struct MyUuid(#[serde(deserialize_with = "deserialize_from_str")] pub Uuid);

#[Scalar]
impl ScalarType for MyUuid {
	fn parse(value: Value) -> InputValueResult<Self> {
		Ok(MyUuid(Uuid::from_str(&value.to_string()).unwrap()))
	}
	fn to_value(&self) -> Value {
		Value::String(self.0.to_string())
	}
}

/* impl From<Uuid> for MyUuid {
	fn from(val: Uuid) -> Self {
		Self(val)
	}
}

impl From<MyUuid> for Uuid {
	fn from(val: MyUuid) -> Self {
		val.0
	}
} */
