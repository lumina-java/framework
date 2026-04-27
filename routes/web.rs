use crate::core::router::Router;
use crate::app::controllers::home_controller::HomeController;
use crate::app::controllers::user_controller::UserController;
use crate::app::controllers::auth_controller::AuthController;
use crate::app::controllers::dashboard_controller::DashboardController;
use crate::core::application::AppState;
use crate::http::middleware::web_auth_required;
use std::sync::Arc;
use axum::middleware::from_fn;

/// Registrasi semua Web Routes (HTML responses).
pub fn register() -> Router<Arc<AppState>> {
    // ─── Public Routes ───
    let public = Router::new()
        .get("/",          HomeController::index)
        .get("/about",     HomeController::about)
        .get("/users",     UserController::index)
        .get("/users/:id", UserController::show)
        
        // Authentication
        .get("/auth/login",     AuthController::show_login)
        .get("/auth/register",  AuthController::show_register)
        .post("/auth/login",    AuthController::login)
        .post("/auth/register", AuthController::register)
        
        // Debug
        .get("/debug/panic",    HomeController::debug_panic);

    // ─── Protected Routes (Hanya untuk User Login) ───
    let protected = Router::new()
        .get("/dashboard",   DashboardController::index)
        .get("/auth/logout",  AuthController::logout)
        .layer(from_fn(web_auth_required));

    // Gabungkan public dan protected
    public.merge(protected)
}
