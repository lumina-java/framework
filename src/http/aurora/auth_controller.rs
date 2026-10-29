use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::core::auth::{Auth, AuthUser};
use crate::core::request::Request;
use crate::core::security::BruteForceGuard;
use crate::core::validation::ValidatedForm;
use crate::core::view::View;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct AuroraUserRecord {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

pub struct AuroraAuthController;

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: String,
    #[serde(default)]
    pub remember: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct RegisterForm {
    #[validate(length(min = 2, message = "Nama minimal 2 karakter"))]
    pub name: String,
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
    pub password_confirmation: String,
}

impl AuroraAuthController {
    /// GET /login — Menampilkan halaman login Aurora
    pub async fn show_login(req: Request) -> Response {
        if let Some(res) = req.redirect_if_authenticated("/dashboard").await {
            return res;
        }

        View::make("aurora.auth.login")
            .with("title", "Masuk — Lumina Aurora")
            .render(&req)
            .await
            .into_response()
    }

    /// POST /login — Memproses autentikasi pengguna dengan BruteForce & Audit Trail
    pub async fn login(req: Request, form: ValidatedForm<LoginForm>) -> Response {
        let f = &form.0;
        let ip = req
            .header("x-forwarded-for")
            .or_else(|| req.header("x-real-ip"))
            .unwrap_or("127.0.0.1");
        let brute_key = format!("{}:{}", ip, f.email);

        // 1. Cek proteksi Brute-Force
        if let Err(e) = BruteForceGuard::check(&brute_key).await {
            return req.back().with_error(&e.to_string()).go(&req).await;
        }

        // 2. Verifikasi Kredensial Pengguna via Database
        let user_res: Result<Option<AuroraUserRecord>, _> = sqlx::query_as(
            "SELECT id, name, email, password, role FROM users WHERE email = ? LIMIT 1"
        )
        .bind(&f.email)
        .fetch_optional(&req.db().pool)
        .await;

        if let Ok(Some(user)) = user_res {
            if Auth::verify(&f.password, &user.password) {
                // Reset counter brute-force jika login berhasil
                BruteForceGuard::reset(&brute_key).await;

                let auth_user = AuthUser::new(
                    user.id,
                    user.email.clone(),
                    user.role.clone(),
                    vec!["users.view".to_string()],
                    if f.remember.is_some() { 720 } else { 24 },
                );

                let _ = Auth::login(&req.session, auth_user).await;

                // Catat log audit login berhasil
                req.audit_log(
                    "LOGIN",
                    "auth",
                    Some(user.id.to_string()),
                    None,
                    "SUCCESS",
                );

                return req
                    .redirect("/dashboard")
                    .with_success("Selamat datang kembali di Lumina Aurora!")
                    .go(&req)
                    .await;
            }
        }

        // Catat percobaan gagal di BruteForceGuard
        let attempts = BruteForceGuard::hit(&brute_key).await;
        req.audit_log("LOGIN", "auth", None, None, "FAILED");

        let err_msg = if attempts >= 5 {
            "Terlalu banyak percobaan gagal. Akun/IP dikunci sementara 15 menit.".to_string()
        } else {
            format!("Email atau password salah. (Percobaan {}/5)", attempts)
        };

        req.back().with_error(&err_msg).go(&req).await
    }

    /// GET /register — Menampilkan halaman registrasi Aurora
    pub async fn show_register(req: Request) -> Response {
        if let Some(res) = req.redirect_if_authenticated("/dashboard").await {
            return res;
        }

        View::make("aurora.auth.register")
            .with("title", "Daftar Akun — Lumina Aurora")
            .render(&req)
            .await
            .into_response()
    }

    /// POST /register — Memproses pendaftaran pengguna baru
    pub async fn register(req: Request, form: ValidatedForm<RegisterForm>) -> Response {
        let f = &form.0;

        if f.password != f.password_confirmation {
            return req
                .back()
                .with_error("Konfirmasi password tidak cocok.")
                .go(&req)
                .await;
        }

        let existing: Result<Option<AuroraUserRecord>, _> = sqlx::query_as(
            "SELECT id, name, email, password, role FROM users WHERE email = ? LIMIT 1"
        )
        .bind(&f.email)
        .fetch_optional(&req.db().pool)
        .await;

        if let Ok(Some(_)) = existing {
            return req
                .back()
                .with_error("Email sudah terdaftar. Silakan gunakan email lain.")
                .go(&req)
                .await;
        }

        let hashed = Auth::make_hash(&f.password);
        let insert_res = sqlx::query(
            "INSERT INTO users (name, email, password, role) VALUES (?, ?, ?, 'member')"
        )
        .bind(&f.name)
        .bind(&f.email)
        .bind(&hashed)
        .execute(&req.db().pool)
        .await;

        if let Ok(res) = insert_res {
            let new_id = res.last_insert_id().unwrap_or(1);
            req.audit_log(
                "REGISTER",
                "users",
                Some(new_id.to_string()),
                None,
                "SUCCESS",
            );

            let auth_user = AuthUser::new(new_id, f.email.clone(), "member".to_string(), vec![], 24);
            let _ = Auth::login(&req.session, auth_user).await;

            return req
                .redirect("/dashboard")
                .with_success("Akun berhasil dibuat! Selamat datang di Lumina Aurora.")
                .go(&req)
                .await;
        }

        req.back()
            .with_error("Terjadi kesalahan saat menyimpan data akun.")
            .go(&req)
            .await
    }

    /// POST /logout — Mengakhiri sesi pengguna
    pub async fn logout(req: Request) -> Response {
        req.audit_log("LOGOUT", "auth", None, None, "SUCCESS");
        Auth::logout(&req.session).await;

        req.redirect("/login")
            .with_success("Anda telah berhasil keluar.")
            .go(&req)
            .await
    }
}
