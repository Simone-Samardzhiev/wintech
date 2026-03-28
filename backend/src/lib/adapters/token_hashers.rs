use crate::domain::user::models::{Token, UserError};
use crate::domain::user::ports::TokenHasher;
use anyhow::Context;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// Struct holding JWT claims.
#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    id: Uuid,
    pub kind: String,
    pub expiry: OffsetDateTime,
    pub user_id: Uuid,
}

impl From<Token> for Claims {
    fn from(token: Token) -> Self {
        Self {
            id: token.id,
            kind: token.kind.as_ref().into(),
            expiry: token.expiry,
            user_id: token.user_id,
        }
    }
}

/// Implementation of [`TokenHasher`] using JWT.
pub struct JWTTokenHasher {
    secret: EncodingKey,
}

impl JWTTokenHasher {
    pub fn new(secret: String) -> Self {
        JWTTokenHasher {
            secret: EncodingKey::from_secret(secret.as_bytes()),
        }
    }
}

impl TokenHasher for JWTTokenHasher {
    fn hash(&self, token: Token) -> Result<String, UserError> {
        let hash = encode(&Header::default(), &Claims::from(token), &self.secret)
            .context("Error encoding token")?;
        Ok(hash)
    }
}
