use crate::domain::user::{
    models::{Token, TokenKind, UserError},
    ports::TokenCoder,
};
use anyhow::Context;
use jsonwebtoken::{
    Algorithm::HS256, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

/// Struct holding JWT claims.
#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    id: Uuid,
    pub kind: String,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub aud: String,
    pub user_id: Uuid,
}

impl Claims {
    pub fn new(token: &Token, iss: &str, aud: &str) -> Self {
        Self {
            id: token.id,
            kind: token.kind.as_ref().to_string(),
            exp: token.expiry.unix_timestamp(),
            iat: OffsetDateTime::now_utc().unix_timestamp(),
            iss: iss.to_string(),
            aud: aud.to_string(),
            user_id: token.user_id,
        }
    }
}

/// Struct holding data needed to encode and decode tokens.
struct JWTCoderInner {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    issuer: String,
    audience: String,
    validation: Validation,
}

impl JWTCoderInner {
    pub fn new(secret: String, issuer: String, audience: String) -> Self {
        let mut validation = Validation::new(HS256);
        validation.set_issuer(&[&issuer]);
        validation.set_audience(&[&audience]);

        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            issuer,
            audience,
            validation,
        }
    }
}

/// Implementation of [`TokenCoder`] using JWT.
#[derive(Clone)]
pub struct JWTTokenCoder {
    inner: Arc<JWTCoderInner>,
}

impl JWTTokenCoder {
    pub fn new(secret: String, issuer: String, audience: String) -> Self {
        Self {
            inner: Arc::new(JWTCoderInner::new(secret, issuer, audience)),
        }
    }
}

impl TokenCoder for JWTTokenCoder {
    fn encode(&self, token: &Token) -> Result<String, UserError> {
        let claims = Claims::new(token, &self.inner.issuer, &self.inner.audience);
        let hash = encode(&Header::default(), &claims, &self.inner.encoding_key)
            .context("Error encoding token")?;
        Ok(hash)
    }

    fn decode(&self, token: &str) -> Result<Token, UserError> {
        let claims = decode::<Claims>(token, &self.inner.decoding_key, &self.inner.validation)
            .map_err(|_| UserError::InvalidToken)?
            .claims;

        let kind = TokenKind::try_from(claims.kind.as_str())?;
        let expiry =
            OffsetDateTime::from_unix_timestamp(claims.exp).map_err(|_| UserError::InvalidToken)?;

        Ok(Token::new(claims.id, kind, expiry, claims.user_id))
    }
}
