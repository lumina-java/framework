use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::core::request::Request;
use crate::core::view::View;

pub struct AuroraDashboardController;

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct AuroraUserItem {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: Option<String>,
}

impl AuroraDashboardController {
    /// GET /dashboard — Tampilan dashboard utama Aurora
    pub async fn index(req: Request) -> Response {
        let mut user = match &req.user {
            Some(u) => u.clone(),
            None => {
                return req
                    .redirect("/login")
                    .with_error("Silakan masuk terlebih dahulu.")
                    .go(&req)
                    .await;
            }
        };

        // Jika nama di sesi masih kosong, otomatis muat nama asli dari tabel users
        if user.name.is_empty() {
            if let Ok(user_id) = user.sub.parse::<i64>() {
                if let Ok(Some((db_name,))) = sqlx::query_as::<_, (String,)>("SELECT name FROM users WHERE id = ? LIMIT 1")
                    .bind(user_id)
                    .fetch_optional(&req.db().pool)
                    .await
                {
                    user.name = db_name;
                }
            }
        }
        if user.name.is_empty() {
            user.name = user.email.split('@').next().unwrap_or("Pengguna").to_string();
        }

        let total_users: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&req.db().pool)
            .await
            .unwrap_or(1);

        // Ambil daftar pengguna untuk tabel manajemen hak akses (Role Management)
        let users_list: Vec<AuroraUserItem> = sqlx::query_as::<_, AuroraUserItem>(
            "SELECT id, name, email, role, created_at FROM users ORDER BY id ASC LIMIT 50"
        )
        .fetch_all(&req.db().pool)
        .await
        .unwrap_or_default();

        View::make("aurora.dashboard.index")
            .with("title", "Dashboard — Lumina Aurora")
            .with("user", user)
            .with("total_users", total_users)
            .with("users_list", users_list)
            .render(&req)
            .await
            .into_response()
    }
}
