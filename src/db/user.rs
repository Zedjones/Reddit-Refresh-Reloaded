use chrono::NaiveTime;
use sqlx::PgPool;
use std::time::Duration;

pub(crate) struct User {
    username: String,
    password: String,
    refresh_time: Duration,
    token: Option<String>,
}

pub(crate) struct NewUser {
    username: String,
    password: String,
    refresh_time: Duration,
}

impl User {
    pub async fn insert(user: NewUser, pool: PgPool) -> anyhow::Result<Self> {
        let midnight = NaiveTime::from_num_seconds_from_midnight(0, 0);
        let mut conn = pool.begin().await?;
        let seconds = user.refresh_time.as_secs();
        let user = sqlx::query!(
            "INSERT INTO users (username, password, refresh_time) \
             VALUES ($1, $2, $3) RETURNING token, username, password, refresh_time",
            user.username,
            user.password,
            NaiveTime::from_num_seconds_from_midnight(seconds as u32, 0)
        )
        .fetch_one(&mut conn)
        .await?;
        conn.commit().await?;
        Ok(User {
            username: user.username,
            password: user.password,
            refresh_time: (user.refresh_time.unwrap() - midnight).to_std().unwrap(),
            token: user.token,
        })
    }
}
