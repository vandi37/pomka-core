use std::{ fmt::Debug, ops::Add};

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use signed_token::Signed;
use uuid::Uuid;

const ISS: &'static str = "pomka-core";
use crate::config::TokenConfig;

#[derive(Clone)]
pub struct TokensState {
    pub admins: JwtComplex,
    pub bots: JwtComplex,
    pub userbots: Signed,
    pub user_tokens: Signed,
}

#[derive(Clone)]
pub struct JwtComplex {
    pub refresh: String,
    pub access: String,
}
#[derive(Clone, Debug)]
pub struct InvalidSecretLength;
impl TryFrom<TokenConfig> for TokensState {
    type Error = InvalidSecretLength;
    fn try_from(value: TokenConfig) -> Result<Self, Self::Error> {
        Ok(TokensState {
            admins: JwtComplex {
                refresh: value.admins_refresh,
                access: value.admins_access,
            },
            bots: JwtComplex {
                refresh: value.bots_refresh,
                access: value.bots_access,
            },
            userbots: Signed::new(value.userbots).ok_or(InvalidSecretLength)?,
            user_tokens: Signed::new(value.user_tokens).ok_or(InvalidSecretLength)?,
        })
    }
}

pub fn create_jwt<T: Serialize>(user_id: i64, data: T, secret: &[u8], add: Duration) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .add(add)
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration,
        iat: Utc::now().timestamp(),
        iss: ISS.to_string(),
        jti: Uuid::new_v4(),
        data,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}
pub fn validate_jwt<T: DeserializeOwned>(token: &str, secret: &[u8]) -> Result<Claims<T>, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::default());
    validation.validate_exp = true;
    validation.set_issuer(&[ISS.to_string()]);
    Ok(decode::<Claims<T>>(token, &DecodingKey::from_secret(secret), &validation)?.claims)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims<T> {
    pub sub: i64,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub jti: uuid::Uuid,
    #[serde(flatten)]
    pub data: T,
}