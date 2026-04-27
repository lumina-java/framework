pub mod core;
pub mod http;
pub mod app;
pub mod database;
pub mod config;
pub mod support;

pub mod prelude {
    pub use crate::core::application::Application;
    pub use crate::http::kernel::HttpKernel;
}
