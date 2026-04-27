use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::core::auth::Claims;

/// Generate JWT token dari Claims.
/// Secret diambil dari environment variable `JWT_SECRET`.
pub fn generate_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = crate::support::env("JWT_SECRET", "lumina-secret-change-in-production");
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validasi dan decode JWT token.
/// Mengembalikan Claims jika valid, error jika expired atau invalid signature.
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = crate::support::env("JWT_SECRET", "lumina-secret-change-in-production");
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
