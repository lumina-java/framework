use axum::{Json, extract::Json as JsonBody};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email:    String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name:     String,
    pub email:    String,
    pub password: String,
}

pub struct AuthController;

impl AuthController {
    /// POST /api/auth/login
    ///
    /// Verifikasi kredensial dan kembalikan JWT token jika valid.
    /// Kredensial demo: admin@lumina.rs / secret
    pub async fn login(JsonBody(body): JsonBody<LoginRequest>) -> Json<Value> {
        // Skeleton: hardcode credentials — akan diganti query DB + bcrypt verify
        if body.email == "admin@lumina.rs" && body.password == "secret" {
            let claims = crate::core::auth::Claims::new(
                1,
                body.email,
                "admin".to_string(),
                24,  // 24 jam
            );
            match crate::http::auth::generate_token(&claims) {
                Ok(token) => Json(json!({
                    "success":    true,
                    "message":    "Login berhasil",
                    "token":      token,
                    "token_type": "Bearer",
                    "expires_in": 86400
                })),
                Err(_) => Json(json!({
                    "success": false,
                    "message": "Internal error: gagal generate token"
                })),
            }
        } else {
            Json(json!({
                "success": false,
                "message": "Email atau password salah"
            }))
        }
    }

    /// POST /api/auth/register
    ///
    /// Buat akun baru. Skeleton — akan diintegrasikan ke DB pada tahap berikutnya.
    pub async fn register(JsonBody(body): JsonBody<RegisterRequest>) -> Json<Value> {
        // TODO: hash password bcrypt, insert ke DB via User model
        Json(json!({
            "success": true,
            "message": "Registrasi berhasil",
            "data": {
                "id":    99,
                "name":  body.name,
                "email": body.email,
                "role":  "user"
            }
        }))
    }

    /// GET /api/auth/me — Protected route (butuh JWT token)
    ///
    /// Kembalikan profil user yang sedang login.
    /// TODO: ekstrak Claims dari request extension setelah middleware inject.
    pub async fn me() -> Json<Value> {
        Json(json!({
            "success": true,
            "message": "User profile",
            "data": {
                "id":    1,
                "name":  "Slamet Sugandi",
                "email": "admin@lumina.rs",
                "role":  "admin"
            }
        }))
    }
}
