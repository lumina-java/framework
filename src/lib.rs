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
    // axum re-exports — available in every controller via `use lumina::prelude::*`
    pub use axum::extract::State;
    pub use axum::response::Html;
    pub use axum::Form;
    pub use tera::Context;
    pub use crate::core::request::Request;
    pub use axum::response::IntoResponse;
    pub use crate::database::model::Model;
    pub use lumina_macros::LuminaModel;
}


pub use lumina_macros::lumina_form;
pub use validator;
