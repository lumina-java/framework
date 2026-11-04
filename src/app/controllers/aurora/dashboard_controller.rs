use lumina::prelude::*;
use serde::Serialize;
use sqlx::Row;

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
                if let Ok(Some(row)) = sqlx::query("SELECT name FROM users WHERE id = ? LIMIT 1")
                    .bind(user_id)
                    .fetch_optional(&req.db().pool)
                    .await
                {
                    if let Ok(db_name) = row.try_get::<String, _>("name") {
                        user.name = db_name;
                    }
                }
            }
        }
        if user.name.is_empty() {
            user.name = user.email.split('@').next().unwrap_or("Pengguna").to_string();
        }

        let total_users: i64 = match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE deleted_at IS NULL")
            .fetch_one(&req.db().pool)
            .await
        {
            Ok(count) => count,
            Err(_) => sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
                .fetch_one(&req.db().pool)
                .await
                .unwrap_or(1),
        };

        // Ambil daftar pengguna untuk tabel manajemen hak akses (Role Management)
        let mut users_list: Vec<AuroraUserItem> = Vec::new();
        let query_soft = "SELECT id, name, email, role, CAST(created_at AS TEXT) as created_at FROM users WHERE deleted_at IS NULL ORDER BY id ASC LIMIT 50";
        let query_all = "SELECT id, name, email, role, CAST(created_at AS TEXT) as created_at FROM users ORDER BY id ASC LIMIT 50";
        let rows_res = match sqlx::query(query_soft).fetch_all(&req.db().pool).await {
            Ok(r) => Ok(r),
            Err(_) => sqlx::query(query_all).fetch_all(&req.db().pool).await,
        };
        if let Ok(rows) = rows_res {
            for r in rows {
                let id: i64 = r.try_get::<i64, _>("id").or_else(|_| r.try_get::<i32, _>("id").map(|v| v as i64)).unwrap_or_default();
                let name: String = r.try_get("name").unwrap_or_default();
                let email: String = r.try_get("email").unwrap_or_default();
                let role: String = r.try_get("role").unwrap_or_else(|_| "member".to_string());
                let created_at: Option<String> = r.try_get("created_at").ok();
                users_list.push(AuroraUserItem {
                    id,
                    name,
                    email,
                    role,
                    created_at,
                });
            }
        }

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
