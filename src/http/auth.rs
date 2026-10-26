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

/// Validasi dan decode JWT token dengan pengecekan blacklist token revocation.
pub async fn validate_token_with_blacklist(token: &str) -> Result<AuthUser, String> {
    if crate::core::auth::TokenBlacklist::is_revoked(token).await {
        return Err("Token telah dibatalkan (revoked).".to_string());
    }
    validate_token(token).map_err(|e| e.to_string())
}
