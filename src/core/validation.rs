use axum::{
    async_trait,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use validator::Validate;
use std::collections::HashMap;

/// Wrapper untuk Axum Json extractor yang melakukan validasi otomatis.
///
/// Penggunaan:
/// ```rust
/// pub async fn store(ValidatedJson(payload): ValidatedJson<MyRequest>) -> impl IntoResponse { ... }
/// ```
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: Validate + serde::de::DeserializeOwned + 'static,
{
    type Rejection = Response;

    async fn from_request(req: Request<axum::body::Body>, state: &S) -> Result<Self, Self::Rejection> {
        // 1. Ekstrak sebagai Json terlebih dahulu
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| rejection.into_response())?;

        // 2. Lakukan validasi
        if let Err(errors) = value.validate() {
            return Err(ValidationErrorResponse::from(errors).into_response());
        }

        Ok(ValidatedJson(value))
    }
}

/// Wrapper untuk Axum Form extractor yang melakukan validasi otomatis.
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedForm<T>
where
    S: Send + Sync,
    T: Validate + serde::de::DeserializeOwned + 'static,
{
    type Rejection = Response;

    async fn from_request(req: Request<axum::body::Body>, state: &S) -> Result<Self, Self::Rejection> {
        let axum::Form(value) = axum::Form::<T>::from_request(req, state)
            .await
            .map_err(|rejection| rejection.into_response())?;

        if let Err(errors) = value.validate() {
            return Err(ValidationErrorResponse::from(errors).into_response());
        }

        Ok(ValidatedForm(value))
    }
}

/// Struktur response error validasi (422 Unprocessable Entity)
#[derive(Serialize)]
pub struct ValidationErrorResponse {
    pub message: String,
    pub errors: HashMap<String, Vec<String>>,
}

impl From<validator::ValidationErrors> for ValidationErrorResponse {
    fn from(errors: validator::ValidationErrors) -> Self {
        let mut error_map = HashMap::new();

        for (field, field_errors) in errors.field_errors() {
            let messages: Vec<String> = field_errors
                .iter()
                .map(|e| {
                    e.message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Invalid value for field: {}", field))
                })
                .collect();
            
            error_map.insert(field.to_string(), messages);
        }

        Self {
            message: "The given data was invalid.".to_string(),
            errors: error_map,
        }
    }
}

impl IntoResponse for ValidationErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::UNPROCESSABLE_ENTITY, Json(self)).into_response()
    }
}
