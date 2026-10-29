use axum::response::{IntoResponse, Response};
use crate::core::request::Request;
use crate::core::view::View;

pub struct AuroraDashboardController;

impl AuroraDashboardController {
    /// GET /dashboard — Tampilan dashboard utama Aurora
    pub async fn index(req: Request) -> Response {
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

        let total_users: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&req.db().pool)
            .await
            .unwrap_or(1);

        View::make("aurora.dashboard.index")
            .with("title", "Dashboard — Lumina Aurora")
            .with("user", user)
            .with("total_users", total_users)
            .render(&req)
            .await
            .into_response()
    }
}
