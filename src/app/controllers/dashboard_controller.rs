use crate::core::request::Request;
use crate::core::view::View;
use axum::response::IntoResponse;

pub struct DashboardController;

impl DashboardController {
    /// GET /dashboard
    /// Menggunakan Request bundle untuk akses cepat ke State, Session, dan User.
    pub async fn index(req: Request) -> impl IntoResponse {
        // Karena route ini dilindungi middleware, req.user pasti Some
        let user = req.user.expect("Unauthorized access to dashboard");

        View::make("dashboard.index")
            .with("title", "Dashboard — Lumina")
            .with("tuan", "Tuang adalah lumina")
            .with("user_id", &user.sub)
            .with("email", &user.email)
            .with("role", &user.role)
            .render(&req.state, &req.session)
            .await
    }
}
