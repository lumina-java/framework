use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Response, Redirect},
};
use serde::Deserialize;
use crate::core::request::Request;
use crate::core::auth::socialite::Socialite;
use crate::core::auth::Auth;
use crate::app::models::user::User;
use crate::database::model::Model;

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code:  String,
    pub state: String,
}

pub struct SocialAuthController;

impl SocialAuthController {
    /// Redirect ke provider OAuth2.
    pub async fn redirect(req: Request, Path(provider): Path<String>) -> Response {
        if let Some((url, csrf_token)) = Socialite::get_redirect_url(&provider) {
            // Simpan CSRF token ke session untuk divalidasi nanti
            let _ = req.session.insert("oauth_state", csrf_token).await;
            return Redirect::to(&url).into_response();
        }
        
        Redirect::to("/auth/login").into_response()
    }

    /// Callback dari provider OAuth2.
    pub async fn callback(
        req: Request,
        Path(provider): Path<String>,
        Query(query): Query<CallbackQuery>,
    ) -> Response {
        // Validasi CSRF state
        let saved_state: Option<String> = req.session.get("oauth_state").await.unwrap_or_default();
        if saved_state.is_none() || saved_state.unwrap() != query.state {
            return Redirect::to("/auth/login").into_response();
        }

        // Ambil data user dari provider
        match Socialite::get_user(&provider, query.code).await {
            Ok(social_user) => {
                let pool = req.db().clone();
                
                // Cari user berdasarkan email
                let user_result = User::find_by_email(&pool, &social_user.email).await;
                
                let user = match user_result {
                    Ok(Some(u)) => u,
                    _ => {
                        // Jika belum ada, buat user baru (Auto Register)
                        let mut new_user = User::default();
                        new_user.name = social_user.name;
                        new_user.email = social_user.email;
                        new_user.password = Auth::make_hash(&crate::support::str::random(16)); // Random password
                        new_user.role = "user".to_string();
                        
                        let id = new_user.save(&pool).await.unwrap_or(0);
                        new_user.id = id;
                        new_user
                    }
                };

                // Login user
                Auth::login(&req, user).await;
                
                Redirect::to("/dashboard").into_response()
            }
            Err(_) => Redirect::to("/auth/login").into_response(),
        }
    }
}
