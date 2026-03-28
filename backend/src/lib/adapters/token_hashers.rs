use crate::domain::user::models::{Token, UserError};
use crate::domain::user::ports::TokenHasher;
use anyhow::Context;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// Struct holding JWT claims.
#[derive(Serialize, Deserialize, Debug)]
struct Claims<'a> {
    id: Uuid,
    pub kind: String,
    pub exp: i64,
    pub iat: i64,
    pub iss: &'a str,
    pub aud: &'a str,
    pub user_id: Uuid,
}

impl<'a> Claims<'a> {
    pub fn new(token: Token, iss: &'a str, aud: &'a str) -> Self {
        Self {
            id: token.id,
            kind: token.kind.as_ref().to_string(),
            exp: token.expiry.unix_timestamp(),
            iat: OffsetDateTime::now_utc().unix_timestamp(),
            iss,
            aud,
            user_id: token.user_id,
        }
    }
}

/// Implementation of [`TokenHasher`] using JWT.
pub struct JWTTokenHasher {
    secret: EncodingKey,
    issuer: String,
    audience: String,
}

impl JWTTokenHasher {
    pub fn new(secret: String, issuer: String, audience: String) -> Self {
        JWTTokenHasher {
            secret: EncodingKey::from_secret(secret.as_bytes()),
            issuer,
            audience,
        }
    }
}

impl TokenHasher for JWTTokenHasher {
    fn hash(&self, token: Token) -> Result<String, UserError> {
        let claims = Claims::new(token, &self.issuer, &self.audience);
        let hash =
            encode(&Header::default(), &claims, &self.secret).context("Error encoding token")?;
        Ok(hash)
    }
}
