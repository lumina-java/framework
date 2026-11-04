use lumina::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub struct AuroraProfileController;

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateProfileForm {
    #[validate(length(min = 2, message = "Nama minimal 2 karakter"))]
    pub name: String,
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
}

#[derive(Deserialize, Serialize, Validate)]
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

        let update_res = if let Ok(tx) = req.begin().await {
            let res = tx.table("users")
                .where_eq("id", user_id)
                .update([("name", &f.name), ("email", &f.email)])
                .await;
            if res.is_ok() {
                let _ = tx.commit().await;
            }
            res
        } else {
            sqlx::query("UPDATE users SET name = ?, email = ? WHERE id = ?")
                .bind(&f.name)
                .bind(&f.email)
                .bind(user_id)
                .execute(&req.db().pool)
                .await
                .map(|r| r.rows_affected())
        };

        if update_res.is_ok() {
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

        let user_row: Result<Option<(String,)>, _> =
            sqlx::query_as("SELECT password FROM users WHERE id = ? LIMIT 1")
                .bind(user_id)
                .fetch_optional(&req.db().pool)
                .await;

        if let Ok(Some((current_hash,))) = user_row {
            if !Auth::verify(&f.current_password, &current_hash) {
                return req
                    .back()
                    .with_error("Kata sandi saat ini salah.")
                    .go(&req)
                    .await;
            }

            let new_hash = Auth::make_hash(&f.new_password);
            let _ = sqlx::query("UPDATE users SET password = ? WHERE id = ?")
                .bind(&new_hash)
                .bind(user_id)
                .execute(&req.db().pool)
                .await;

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

    /// POST /profile/delete — Menghapus/menonaktifkan akun dengan transaksi ACID & Soft Deletes
    pub async fn destroy(req: Request) -> Response {
        let auth_user = match &req.user {
            Some(u) => u.clone(),
            None => return req.redirect("/login").go(&req).await,
        };

        let user_id: i64 = auth_user.sub.parse().unwrap_or(0);

        if let Ok(tx) = req.begin().await {
            let del_res = tx.table("users")
                .soft_deletes()
                .where_eq("id", user_id)
                .delete()
                .await;

            if del_res.is_ok() {
                let _ = tx.commit().await;
                req.audit_log(
                    "DELETE_ACCOUNT",
                    "users",
                    Some(user_id.to_string()),
                    None,
                    "SUCCESS",
                );

                return req
                    .redirect("/login")
                    .with_success("Akun Anda telah dinonaktifkan.")
                    .go(&req)
                    .await;
            }
        }

        req.back()
            .with_error("Gagal menonaktifkan akun.")
            .go(&req)
            .await
    }
}
