pub mod cli;
pub mod config;
pub mod core;
pub mod database;
pub mod http;
pub mod support;

pub mod prelude {
    pub use crate::core::application::{Application, AppState};
    pub use crate::core::router::Router;
    pub use crate::http::kernel::HttpKernel;
    pub use axum::extract::State;
}

pub use lumina_macros::lumina_form;
pub use validator;
