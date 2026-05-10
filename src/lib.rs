pub mod core;
pub mod http;
pub mod app;
pub mod database;
pub mod config;
pub mod support;
pub mod cli;

// Include route files sebagai bagian dari crate.
// Teknik #[path] membuat routes/web.rs bisa menggunakan `crate::` paths
// seolah-olah berada di dalam src/.
#[path = "../routes/web.rs"]
pub mod web_routes;

#[path = "../routes/api.rs"]
pub mod api_routes;

#[path = "../database/seeders/mod.rs"]
pub mod seeders;

#[path = "../database/factories/mod.rs"]
pub mod factories;

pub mod prelude {
    pub use crate::core::application::Application;
    pub use crate::core::router::Router;
    pub use crate::http::kernel::HttpKernel;
}
