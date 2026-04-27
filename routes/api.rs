use crate::core::router::Router;
use crate::app::controllers::api_controller::ApiController;

/// Registrasi semua API Routes (JSON responses).
///
/// Semua route di sini akan diprefix /api oleh Application::build_router().
/// Contoh: GET /users/:id di sini → diakses sebagai GET /api/users/:id
pub fn register() -> Router {
    Router::new()
        .get("/users",      ApiController::index)
        .get("/users/:id",  ApiController::show)   // ← Dynamic route
        .post("/users",     ApiController::store)
}
