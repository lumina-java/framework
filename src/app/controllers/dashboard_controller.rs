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
        let user = req.user.clone().expect("Unauthorized access to dashboard");

        // 🔥 EAGER LOADING IN ACTION (N+1 Solution)
        // Mengambil semua user dan post mereka dalam 2 query SQL optimal
        let users = User::query(req.db())
            .with("posts")
            .get()
            .await
            .unwrap_or_default();

        // Cuantik, bersih, bebas tanda ampersand (&)
        View::make("dashboard.index")
            .with("title", "Dashboard — Lumina")
            .with("user_id", user.sub.clone())
            .with("email", user.email.clone())
            .with("role", user.role.clone())
            .with("user", user)
            .with("users", users)
            .render(req)
            .await
    }
}
