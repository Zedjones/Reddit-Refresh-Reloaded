use chrono::NaiveTime;
use sqlx::PgPool;
use std::time::Duration;

pub(crate) struct User {
    pub username: String,
    pub password: String,
    pub refresh_time: Duration,
    pub token: Option<String>,
}

pub(crate) struct NewUser {
    pub username: String,
    pub password: String,
    pub refresh_time: Duration,
}

impl User {
    pub async fn insert(user: NewUser, pool: &PgPool) -> anyhow::Result<Self> {
        let midnight = NaiveTime::from_num_seconds_from_midnight(0, 0);
        let mut conn = pool.begin().await?;
        let hashed = bcrypt::hash(user.password, bcrypt::DEFAULT_COST)?;
        let seconds = user.refresh_time.as_secs();
        let user = sqlx::query!(
            "INSERT INTO users (username, password, refresh_time) \
             VALUES ($1, $2, $3) RETURNING token, username, password, refresh_time",
            user.username,
            hashed,
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
    pub async fn get_user(username: &str, pool: &PgPool) -> anyhow::Result<Self> {
        let midnight = NaiveTime::from_num_seconds_from_midnight(0, 0);
        let mut conn = pool.begin().await?;
        let user = sqlx::query!(
            "SELECT token, username, password, refresh_time FROM users \
             WHERE username = $1",
            username
        )
        .fetch_one(&mut conn)
        .await?;
        Ok(User {
            username: user.username,
            password: user.password,
            refresh_time: (user.refresh_time.unwrap() - midnight).to_std().unwrap(),
            token: user.token,
        })
    }
    pub async fn verify_login(username: &str, password: &str, pool: &PgPool) -> anyhow::Result<bool> {
        let user = User::get_user(username, pool).await?;
        Ok(bcrypt::verify(password, &user.password)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::db::{
        timeout_connect,
        user::{NewUser, User},
    };
    #[tokio::test]
    /**
    This test isn't really a proper test, it's just a quick and easy way to test
    that we can properly insert a value into an empty database. This will currently
    fail if run twice in a row without clearing the DB.
    */
    async fn insert_user() {
        std::env::set_var(
            "DATABASE_URL",
            "postgresql://zedjones:changeMe@localhost/postgres",
        );
        let pool = timeout_connect().await.unwrap();
        let user = NewUser {
            password: "a_password".to_string(),
            username: "a_user".to_string(),
            refresh_time: std::time::Duration::from_secs(5),
        };
        let user = User::insert(user, &pool).await.unwrap();
        assert_eq!(std::time::Duration::from_secs(5), user.refresh_time);
    }
}
