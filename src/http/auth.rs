use crate::core::auth::AuthUser;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

/// Generate JWT token dari AuthUser.
pub fn generate_token(user: &AuthUser) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("APP_KEY").unwrap_or_else(|_| "secret".to_string());
    encode(
        &Header::default(),
        user,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

/// Validasi dan decode JWT token.
/// Mengembalikan AuthUser jika valid, error jika expired atau invalid signature.
pub fn validate_token(token: &str) -> Result<AuthUser, jsonwebtoken::errors::Error> {
    let secret = std::env::var("APP_KEY").unwrap_or_else(|_| "secret".to_string());
    let data = decode::<AuthUser>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data.claims)
}
