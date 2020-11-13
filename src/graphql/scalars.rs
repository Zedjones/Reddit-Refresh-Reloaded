use async_graphql::{InputValueError, InputValueResult, Number, Scalar, ScalarType, Value};
use chrono::NaiveDateTime;
use std::time::Duration;

pub(crate) struct TimestampDateTime(pub NaiveDateTime);

#[Scalar(name = "DateTime")]
impl ScalarType for TimestampDateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::Number(val) = value {
            Ok(TimestampDateTime(NaiveDateTime::from_timestamp(
                val.as_i64().unwrap(),
                0,
            )))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }
    fn to_value(&self) -> Value {
        Value::Number(Number::from(self.0.timestamp() as i32))
    }
}

impl From<NaiveDateTime> for TimestampDateTime {
    fn from(item: NaiveDateTime) -> Self {
        TimestampDateTime(item)
    }
}

pub(crate) struct DurationString(pub Duration);

#[Scalar(name = "Duration")]
impl ScalarType for DurationString {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(val) = value {
            Ok(DurationString(humantime::parse_duration(&val)?))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }
    fn to_value(&self) -> Value {
        Value::String(self.0.as_secs().to_string() + "s")
    }
}

impl From<Duration> for DurationString {
    fn from(item: Duration) -> Self {
        DurationString(item)
    }
}
