use crate::core::request::Request;
use crate::core::view::View;
use axum::response::IntoResponse;
use crate::app::models::user::User;
use crate::database::model::Model;

pub struct DashboardController;

impl DashboardController {
    /// GET /dashboard
    /// Menggunakan Request bundle untuk akses cepat ke State, Session, dan User.
    pub async fn index(req: Request) -> impl IntoResponse {
        // Karena route ini dilindungi middleware, req.user pasti Some
        let user_session = req.user.expect("Unauthorized access to dashboard");

        // 🔥 EAGER LOADING IN ACTION (N+1 Solution)
        // Kita mengambil semua user dan semua post mereka dalam 2 query saja.
        let users = User::query(&req.state.db)
            .with("posts")
            .get()
            .await
            .unwrap_or_default();

        View::make("dashboard.index")
            .with("title", "Dashboard — Lumina")
            .with("user_id", &user_session.sub)
            .with("email", &user_session.email)
            .with("role", &user_session.role)
            .with("users", &users)
            .render(&req.state, &req.session)
            .await
    }
}
