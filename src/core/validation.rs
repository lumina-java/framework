use std::collections::HashMap;
use validator::{Validate, ValidationErrors};

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
                        "length" => {
                            if let Some(min) = error.params.get("min") {
                                format!("Minimal {} karakter.", min).into()
                            } else if let Some(max) = error.params.get("max") {
                                format!("Maksimal {} karakter.", max).into()
                            } else {
                                "Panjang karakter tidak sesuai.".into()
                            }
                        }
                        "required" => "Field ini wajib diisi.".into(),
                        "unique" => "Data ini sudah terdaftar.".into(),
                        "must_match" => "Konfirmasi tidak cocok.".into(),
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
    extract::{FromRequest, FromRequestParts, Request},
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

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
        let value = Json::<T>::from_request(req, state)
            .await
            .map_err(|_e| {
                crate::core::error::AppError::BadRequest("Format JSON tidak valid".to_string())
            })?
            .0;

        value
            .validate()
            .map_err(|e| crate::core::error::AppError::ValidationError(format!("{}", e)))?;

        Ok(ValidatedJson(value))
    }
}

/// Trait opsional untuk form yang membutuhkan validasi CSRF otomatis.
pub trait CsrfValidatable {
    fn get_csrf_token(&self) -> &str;
}

pub struct ValidatedForm<T>(pub T);

pub struct FormValidationRejection(pub axum::response::Response);

impl IntoResponse for FormValidationRejection {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Serialize + Validate + Send + Sync + 'static,
    S: Send + Sync,
    axum::Form<T>: FromRequest<S>,
{
    type Rejection = FormValidationRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Ambil Session dan Referer sebelum request dikonsumsi
        let session = req.extensions().get::<tower_sessions::Session>().cloned();
        let referer = req
            .headers()
            .get(axum::http::header::REFERER)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("/")
            .to_string();

        // Ekstrak CsrfToken untuk memastikan sinkronisasi saat redirect
        let (mut parts, body) = req.into_parts();
        let token = axum_csrf::CsrfToken::from_request_parts(&mut parts, state)
            .await
            .ok();
        let req = Request::from_parts(parts, body);

        let form_res = axum::Form::<T>::from_request(req, state).await;
        let value = form_res.ok().map(|axum::Form(v)| v);

        if let Some(value) = value {
            if let Err(e) = value.validate() {
                if let Some(session) = session {
                    let _ = session.insert("_errors", e.to_map()).await;
                    let _ = session
                        .insert("_old", serde_json::to_value(&value).unwrap_or_default())
                        .await;

                    let flash = crate::core::session::FlashManager::new(&session);
                    flash
                        .error("Validasi gagal. Mohon periksa kembali form Anda.")
                        .await;
                }

                let redirect = axum::response::Redirect::to(&referer);
                let res = if let Some(t) = token {
                    (t, redirect).into_response()
                } else {
                    redirect.into_response()
                };

                return Err(FormValidationRejection(res));
            }
            Ok(ValidatedForm(value))
        } else {
            if let Some(session) = session {
                let flash = crate::core::session::FlashManager::new(&session);
                flash.error("Format data tidak valid.").await;
            }

            let redirect = axum::response::Redirect::to(&referer);
            let res = if let Some(t) = token {
                (t, redirect).into_response()
            } else {
                redirect.into_response()
            };

            Err(FormValidationRejection(res))
        }
    }
}
