use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::{DateTime, Utc};

pub struct DateTimeUtc(pub DateTime<Utc>);

#[Scalar]
impl ScalarType for DateTimeUtc {
    fn parse(value: Value) -> InputValueResult<Self> {
        let value = match value.to_string().parse() {
            Ok(val) => val,
            Err(_) => {
                return Err(InputValueError::custom("failed to parse date"))
            }
        };
        Ok(DateTimeUtc(value))
    }
    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
