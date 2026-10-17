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
    pub use axum::extract::{Path, Query, State};
    pub use axum::response::{Html, IntoResponse, Json, Response};
    pub use axum::Form;
    pub use crate::core::blade::BladeContext as Context;
    pub use crate::core::error::{AppError, OptionExt};
    pub use crate::core::request::{HttpResult, LuminaResult, Request};
    pub use crate::core::response::ApiResponse;
    pub use crate::core::validation::{ValidatedForm, ValidatedJson};
    pub use crate::database::connection::{DatabasePool, Db};
    pub use crate::database::model::Model;
    pub use crate::database::orm::{LengthAwarePaginator, OrmResult, QueryBuilder};
    pub use crate::{belongs_to, belongs_to_many, has_many, has_one};
    pub use lumina_macros::{lumina_form, LuminaModel};
    // middleware helpers
    pub use axum::middleware::from_fn;
    pub use crate::http::middleware;
}

pub use lumina_macros::{lumina_form, LuminaModel};
pub use validator;
pub use tower_sessions;

