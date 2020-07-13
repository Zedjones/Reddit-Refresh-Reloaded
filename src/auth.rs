use jsonwebtoken::{
    decode as jwt_decode, encode as jwt_encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone)]
pub struct Encoder {
    pub secret: String,
    pub expiration_time: Duration,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    pub exp: i64,
    pub sub: String,
}

impl Encoder {
    pub(crate) fn encode(&self, claims: &Claims) -> anyhow::Result<String> {
        Ok(jwt_encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?)
    }
    pub(crate) fn decode(&self, token: &str) -> anyhow::Result<Claims> {
        Ok(jwt_decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|token_data| token_data.claims)?)
    }
}
