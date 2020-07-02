use async_graphql::{InputValueError, InputValueResult, ScalarType, Value};
use chrono::NaiveDateTime;

struct TimestampDateTime(NaiveDateTime);

impl ScalarType for TimestampDateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::Int(val) = value {
            Ok(TimestampDateTime(NaiveDateTime::from_timestamp(
                val as i64, 0,
            )))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }
    fn to_value(&self) -> Value {
        Value::Int(self.0.timestamp() as i32)
    }
}
