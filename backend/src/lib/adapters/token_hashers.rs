use crate::domain::user::{
    models::{Token, TokenKind, UserError},
    ports::TokenCoder,
};
use anyhow::Context;
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
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

/// Implementation of [`TokenCoder`] using JWT.
pub struct JWTTokenCoder {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    issuer: String,
    audience: String,
}

impl JWTTokenCoder {
    pub fn new(secret: String, issuer: String, audience: String) -> Self {
        JWTTokenCoder {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            issuer,
            audience,
        }
    }
}

impl TokenCoder for JWTTokenCoder {
    fn encode(&self, token: &Token) -> Result<String, UserError> {
        use jsonwebtoken::encode;

        let claims = Claims::new(token, &self.issuer, &self.audience);
        let hash = encode(&Header::default(), &claims, &self.encoding_key)
            .context("Error encoding token")?;
        Ok(hash)
    }

    fn decode(&self, token: &str) -> Result<Token, UserError> {
        use jsonwebtoken::{Algorithm, Validation, decode};

        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&self.audience]);
        validation.set_issuer(&[&self.issuer]);

        let claims = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|_| UserError::InvalidToken)?
            .claims;

        let kind = TokenKind::try_from(claims.kind.as_str())?;
        let expiry =
            OffsetDateTime::from_unix_timestamp(claims.exp).map_err(|_| UserError::InvalidToken)?;

        Ok(Token::new(claims.id, kind, expiry, claims.user_id))
    }
}
