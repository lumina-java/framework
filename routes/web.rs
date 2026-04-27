use crate::core::router::Router;
use crate::app::controllers::home_controller::HomeController;
use crate::app::controllers::user_controller::UserController;

/// Registrasi semua Web Routes (HTML responses).
///
/// Route di sini langsung diakses tanpa prefix.
/// Contoh: GET /users/:id → UserController::show
pub fn register() -> Router {
    Router::new()
        .get("/",          HomeController::index)
        .get("/about",     HomeController::about)
        .get("/users",     UserController::index)
        .get("/users/:id", UserController::show)   // ← Dynamic route
}
