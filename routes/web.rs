use crate::core::router::Router;
use crate::app::controllers::home_controller::HomeController;
use crate::app::controllers::user_controller::UserController;
use crate::core::application::AppState;
use std::sync::Arc;

/// Registrasi semua Web Routes (HTML responses).
pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .get("/",          HomeController::index)
        .get("/about",     HomeController::about)
        .get("/users",     UserController::index)
        .get("/users/:id", UserController::show)
        // Authentication
        .get("/auth/login",    crate::app::controllers::auth_controller::AuthController::show_login)
        .get("/auth/register", crate::app::controllers::auth_controller::AuthController::show_register)
        .post("/auth/login",   crate::app::controllers::auth_controller::AuthController::login)
        .post("/auth/register", crate::app::controllers::auth_controller::AuthController::register)
}
