use validator::{Validate, ValidationErrors};
use std::collections::HashMap;

/// Trait untuk mempermudah ekstraksi error dari validator ke format yang ramah UI/API.
pub trait Validatable {
    fn to_map(&self) -> HashMap<String, String>;
}

impl Validatable for ValidationErrors {
    fn to_map(&self) -> HashMap<String, String> {
        let mut errors = HashMap::new();
        for (field, field_errors) in self.field_errors() {
            if let Some(error) = field_errors.first() {
                let message = error.message.clone().unwrap_or_else(|| {
                    // Default messages berdasarkan code validator
                    match error.code.as_ref() {
                        "email" => "Format email tidak valid.".into(),
                        "length" => "Panjang karakter tidak sesuai.".into(),
                        "required" => "Field ini wajib diisi.".into(),
                        _ => format!("Field {} tidak valid.", field).into(),
                    }
                });
                errors.insert(field.to_string(), message.to_string());
            }
        }
        errors
    }
}

/// Helper untuk menyimpan old input ke session (agar form tidak kosong saat error).
pub fn flash_errors(_session: &tower_sessions::Session, _errors: HashMap<String, String>) {
    // Implementasi flash logic di sini jika diperlukan selain via render
}

use axum::{
    async_trait,
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S>,
{
    type Rejection = crate::core::error::AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let value = Json::<T>::from_request(req, state).await.map_err(|_e| {
             crate::core::error::AppError::BadRequest("Format JSON tidak valid".to_string())
        })?.0;
        
        value.validate().map_err(|e| {
             crate::core::error::AppError::ValidationError(format!("{}", e))
        })?;
        
        Ok(ValidatedJson(value))
    }
}
