use std::time::Duration;

pub(crate) struct User {
    token: String,
    email: String,
    password: String,
    refresh_time: Duration,
}

impl User {}
