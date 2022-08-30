use std::str::FromStr;

use async_graphql::{InputValueResult, Scalar, ScalarType, Value};
use uuid::Uuid;

pub struct MyUuid(pub Uuid);

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
