use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use crate::core::application::AppState;
use crate::app::models::user::User;
use crate::database::model::Model;
use crate::core::auth::Auth;
use serde::Deserialize;

pub struct SocialiteController;

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
}

impl SocialiteController {
    /// Redirect ke provider OAuth.
    pub async fn redirect(
        Path(provider): Path<String>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        if let Some(driver) = state.socialite.driver(&provider) {
            let url = driver.get_redirect_url();
            return Redirect::to(&url).into_response();
        }

        (axum::http::StatusCode::NOT_FOUND, "Provider not found").into_response()
    }

    /// Menangani callback dari provider OAuth.
    pub async fn callback(
        Path(provider): Path<String>,
        Query(query): Query<CallbackQuery>,
        req: crate::core::request::Request,
    ) -> impl IntoResponse {
        let state = &req.state;
        
        if let Some(driver) = state.socialite.driver(&provider) {
            match driver.get_user_by_code(query.code).await {
                Ok(social_user) => {
                    // 1. Cari atau buat user berdasarkan email
                    let pool = state.db.as_ref().unwrap();
                    let user = match User::find_by_email(pool, &social_user.email).await {
                        Ok(Some(existing_user)) => existing_user,
                        _ => {
                            // Buat user baru jika belum ada
                            let mut new_user = User {
                                name: social_user.name,
                                email: social_user.email,
                                password: Auth::make_hash(&crate::support::str::random(16)),
                                role: "user".to_string(),
                                ..User::default()
                            };
                            
                            match new_user.save(pool).await {
                                Ok(id) => {
                                    new_user.id = id;
                                    new_user
                                },
                                Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Gagal membuat user: {}", e)).into_response(),
                            }
                        }
                    };

                    // 2. Login user ke session
                    Auth::login(&req, user).await;

                    // 3. Redirect ke dashboard
                    return Redirect::to("/dashboard").into_response();
                }
                Err(e) => {
                    return (axum::http::StatusCode::BAD_REQUEST, format!("OAuth Error: {}", e)).into_response();
                }
            }
        }

        (axum::http::StatusCode::NOT_FOUND, "Provider not found").into_response()
    }
}
