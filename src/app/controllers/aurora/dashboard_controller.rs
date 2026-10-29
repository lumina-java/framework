use axum::response::{IntoResponse, Response};
use crate::app::models::user::User;
use crate::core::request::Request;
use crate::core::view::View;
use crate::database::model::Model;

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

        let total_users = User::all(req.db()).await.map(|u| u.len()).unwrap_or(1);

        View::make("aurora.dashboard.index")
            .with("title", "Dashboard — Lumina Aurora")
            .with("user", user)
            .with("total_users", total_users)
            .render(&req)
            .await
            .into_response()
    }
}

