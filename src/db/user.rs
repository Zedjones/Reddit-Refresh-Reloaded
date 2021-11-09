use async_graphql::SimpleObject;
use sqlx::{postgres::types::PgInterval, PgPool};
use std::convert::{TryFrom, TryInto};
use std::time::Duration;

use super::notifiers::apprise::AppriseConfig;

pub(crate) struct User {
    pub username: String,
    pub password: String,
    pub refresh_time: Duration,
    pub notifiers: Vec<AppriseConfig>,
}

impl User {
    pub fn convert_to_duration(interval: PgInterval) -> Duration {
        Duration::from_micros(interval.microseconds.try_into().unwrap())
    }
    pub async fn insert(user: User, pool: &PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let hashed = bcrypt::hash(user.password, bcrypt::DEFAULT_COST)?;
        let user = sqlx::query!(
            "INSERT INTO users (username, password, refresh_time) \
             VALUES ($1, $2, $3) RETURNING username, password, refresh_time",
            user.username,
            hashed,
            PgInterval::try_from(user.refresh_time).unwrap(),
        )
        .fetch_one(&mut conn)
        .await?;
        conn.commit().await?;
        Ok(User {
            username: user.username,
            password: user.password,
            refresh_time: Self::convert_to_duration(user.refresh_time.unwrap()),
            notifiers: Vec::new(),
        })
    }
    pub async fn get_user(username: &str, pool: &PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let user = sqlx::query!(
            "SELECT username, password, refresh_time FROM users \
             WHERE username = $1",
            username
        )
        .fetch_one(&mut conn)
        .await?;
        Ok(User {
            username: user.username.clone(),
            password: user.password,
            refresh_time: Duration::from_micros(
                user.refresh_time.unwrap().microseconds.try_into().unwrap(),
            ),
            notifiers: AppriseConfig::get_configs_for_user(&user.username, &pool).await?, //TODO: update this for actual settings
        })
    }
    pub async fn verify_login(
        username: &str,
        password: &str,
        pool: &PgPool,
    ) -> anyhow::Result<bool> {
        let user = User::get_user(username, pool).await?;
        Ok(bcrypt::verify(password, &user.password)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::db::{timeout_connect, user::User};
    #[tokio::test]
    /**
    This test isn't really a proper test, it's just a quick and easy way to test
    that we can properly insert a value into an empty database. This will currently
    fail if run twice in a row without clearing the DB.
    */
    async fn insert_user() {
        let pool = timeout_connect("postgresql://zedjones:changeMe@localhost/postgres")
            .await
            .unwrap();
        let user = User {
            password: "a_password".to_string(),
            username: "a_user".to_string(),
            refresh_time: std::time::Duration::from_secs(5),
            notifiers: Vec::new(),
        };
        let user = User::insert(user, &pool).await.unwrap();
        assert_eq!(std::time::Duration::from_secs(5), user.refresh_time);
    }
}
