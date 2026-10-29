use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use validator::Validate;
use crate::app::models::user::User;
use crate::core::auth::Auth;
use crate::core::request::Request;
use crate::core::validation::ValidatedForm;
use crate::core::view::View;
use crate::database::model::Model;

pub struct AuroraProfileController;

#[derive(Deserialize, Validate)]
pub struct UpdateProfileForm {
    #[validate(length(min = 2, message = "Nama minimal 2 karakter"))]
    pub name: String,
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct ChangePasswordForm {
    pub current_password: String,
    #[validate(length(min = 8, message = "Password baru minimal 8 karakter"))]
    pub new_password: String,
    pub new_password_confirmation: String,
}

impl AuroraProfileController {
    /// GET /profile — Menampilkan halaman pengaturan profil
    pub async fn edit(req: Request) -> Response {
        let user = match &req.user {
            Some(u) => u.clone(),
            None => {
                return req
                    .redirect("/login")
                    .with_error("Silakan masuk terlebih dahulu.")
                    .go(&req)
                    .await;
            }
        };

        View::make("aurora.profile.edit")
            .with("title", "Pengaturan Profil — Lumina Aurora")
            .with("user", user)
            .render(&req)
            .await
            .into_response()
    }

    /// POST /profile — Memperbarui data nama dan email
    pub async fn update(req: Request, form: ValidatedForm<UpdateProfileForm>) -> Response {
        let auth_user = match &req.user {
            Some(u) => u.clone(),
            None => return req.redirect("/login").go(&req).await,
        };

        let f = &form.0;
        let user_id: i64 = auth_user.sub.parse().unwrap_or(0);

        if let Ok(mut user) = User::find(req.db(), user_id).await {
            user.name = f.name.clone();
            user.email = f.email.clone();
            let _ = user.update(req.db()).await;

            req.audit_log(
                "UPDATE_PROFILE",
                "users",
                Some(user_id.to_string()),
                None,
                "SUCCESS",
            );

            return req
                .back()
                .with_success("Profil Anda berhasil diperbarui.")
                .go(&req)
                .await;
        }

        req.back()
            .with_error("Gagal memperbarui profil.")
            .go(&req)
            .await
    }

    /// POST /profile/password — Mengganti kata sandi dengan verifikasi password lama
    pub async fn change_password(req: Request, form: ValidatedForm<ChangePasswordForm>) -> Response {
        let auth_user = match &req.user {
            Some(u) => u.clone(),
            None => return req.redirect("/login").go(&req).await,
        };

        let f = &form.0;

        if f.new_password != f.new_password_confirmation {
            return req
                .back()
                .with_error("Konfirmasi password baru tidak cocok.")
                .go(&req)
                .await;
        }

        let user_id: i64 = auth_user.sub.parse().unwrap_or(0);

        if let Ok(mut user) = User::find(req.db(), user_id).await {
            if !Auth::verify(&f.current_password, &user.password) {
                return req
                    .back()
                    .with_error("Kata sandi saat ini salah.")
                    .go(&req)
                    .await;
            }

            user.password = Auth::make_hash(&f.new_password);
            let _ = user.update(req.db()).await;

            req.audit_log(
                "CHANGE_PASSWORD",
                "users",
                Some(user_id.to_string()),
                None,
                "SUCCESS",
            );

            return req
                .back()
                .with_success("Kata sandi Anda berhasil diperbarui.")
                .go(&req)
                .await;
        }

        req.back()
            .with_error("Gagal mengubah kata sandi.")
            .go(&req)
            .await
    }
}

