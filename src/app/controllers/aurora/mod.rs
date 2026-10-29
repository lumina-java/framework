pub mod auth_controller;
pub mod dashboard_controller;
pub mod profile_controller;

pub use auth_controller::AuroraAuthController;
pub use dashboard_controller::AuroraDashboardController;
pub use profile_controller::AuroraProfileController;

use crate::core::application::AppState;
use crate::core::router::Router;

/// Menghasilkan Router lengkap untuk seluruh alur Lumina Aurora:
/// - Auth: Login, Register, Logout
/// - Dashboard: Ringkasan sistem dan profil
/// - Profile: Edit nama/email dan ganti kata sandi
/// Lengkap dengan Zero-Panic Safety Boundary dan OWASP Security Headers.
pub fn routes() -> Router<AppState> {
    Router::new()
        .get("/login", AuroraAuthController::show_login)
        .post("/login", AuroraAuthController::login)
        .get("/register", AuroraAuthController::show_register)
        .post("/register", AuroraAuthController::register)
        .post("/logout", AuroraAuthController::logout)
        .get("/dashboard", AuroraDashboardController::index)
        .get("/profile", AuroraProfileController::edit)
        .post("/profile", AuroraProfileController::update)
        .post("/profile/password", AuroraProfileController::change_password)
        .security_headers()
        .panic_boundary()
}
