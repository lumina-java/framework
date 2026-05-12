use crate::app::controllers::auth_controller::AuthController;
use crate::app::controllers::social_auth_controller::SocialAuthController;
use crate::app::controllers::dashboard_controller::DashboardController;
use crate::app::controllers::home_controller::HomeController;
use crate::app::controllers::user_controller::UserController;
use crate::core::application::AppState;
use crate::core::router::Router;
use crate::http::middleware::web_auth_required;
use axum::middleware::from_fn;

/// Registrasi semua Web Routes (HTML responses).
pub fn register(_config: &crate::core::config::ConfigManager) -> Router<AppState> {
    Router::<AppState>::new()
        // ─── Public Routes ───
        .get("/", HomeController::index)
        .get("/about", HomeController::about)
        .get("/health", |axum::extract::State(state): axum::extract::State<AppState>| async move {
            let db_status = if state.db.is_some() { "UP" } else { "DOWN" };
            axum::response::Json(serde_json::json!({
                "status": "UP",
                "database": db_status,
                "framework": "Lumina 1.0",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        })
        .get("/users", UserController::index)
        .get("/users/:id", UserController::show)
        .get("/test-orm", UserController::test_orm)
        
        // Group Authentication
        .group("/auth", |r| {
            r.get("/login", AuthController::show_login)
             .get("/register", AuthController::show_register)
             .post("/login", AuthController::login)
             .post("/register", AuthController::register)
             .get("/logout", AuthController::logout)
             
             // Social Auth
             .get("/:provider/redirect", SocialAuthController::redirect)
             .get("/:provider/callback", SocialAuthController::callback)
        })

        // ─── Protected Routes (Hanya untuk User Login) ───
        .group("/dashboard", |r| {
            r.middleware(from_fn(web_auth_required))
             .get("/", DashboardController::index)
        })

        .get("/debug/panic", HomeController::debug_panic)
        .get("/debug/dd", HomeController::debug_dd)
}
