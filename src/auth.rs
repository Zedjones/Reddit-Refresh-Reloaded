use actix_http::error::Error as HttpError;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{
    decode as jwt_decode, encode as jwt_encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub async fn validate_token(
    req: ServiceRequest,
    bearer: BearerAuth,
) -> Result<ServiceRequest, HttpError> {
    let token = bearer.token();
    let encoder = req.app_data::<Encoder>().unwrap();
    if let Err(err) = encoder.decode(token) {
        log::info!("{}", err);
        Err(actix_web::error::ErrorUnauthorized(err))
    } else {
        Ok(req)
    }
}

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
