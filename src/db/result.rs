use chrono::{DateTime, Utc};

pub(crate) struct Result {
    id: i32,
    inserted: DateTime<Utc>,
    search_id: i32,
    title: String,
}

impl Result {}
