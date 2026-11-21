extern crate self as pakem;
extern crate self as lumina;

pub mod cli;
pub mod config;
pub mod core;
pub mod database;
pub mod http;
pub mod support;

pub mod testing {
    pub use crate::core::testing::*;
}

pub mod openapi {
    pub use crate::core::openapi::*;
}

pub mod tinker {
    pub use crate::core::tinker::*;
}

pub mod hardware {
    pub use crate::core::hardware::*;
}

pub mod ai {
    pub use crate::core::ai::*;
}

pub mod prelude {
    pub use crate::core::application::{Application, AppState};
    pub use crate::core::router::Router;
    pub use crate::core::openapi::{OpenApiSpec, RouteDoc};
    pub use crate::core::testing::{TestApp, TestResponse, TestRequestBuilder};
    pub use crate::http::kernel::HttpKernel;
    // axum re-exports — available in every controller via `use lumina::prelude::*`
    pub use axum::extract::{Path, Query, State};
    pub use axum::response::{Html, IntoResponse, Json, Response};
    pub use axum::Form;
    pub use crate::core::blade::BladeContext as Context;
    pub use crate::core::error::{AppError, OptionExt};
    pub use crate::core::request::{HttpResult, LuminaResult, PakemResult, Request};
    pub use crate::core::response::ApiResponse;
    pub use crate::core::validation::{ValidatedForm, ValidatedJson};
    pub use crate::core::view::View;
    pub use crate::core::storage::{Storage, StorageDriver, S3Config};
    pub use crate::core::cache::{Cache, CacheManager, CacheDriver};
    pub use crate::database::connection::{DatabasePool, Db};
    pub use crate::database::facade::DB;
    pub use crate::database::transaction::{Transaction, TableQueryBuilder};
    pub use crate::database::model::Model;
    pub use crate::database::orm::{LengthAwarePaginator, OrmResult, QueryBuilder};
    pub use crate::database::factory::{Factory, FactoryBuilder, HasFactory};
    pub use crate::database::faker::Faker;
    pub use crate::database::audit::{Audit, AuditLog, AuditRecordBuilder};
    pub use crate::database::observer::{AutoSlug, AutoHash, ModelLifecycle, Observer};
    pub use crate::database::seeder::{Seeder, DatabaseSeeder, SeederRegistry, call_seeder};
    pub use crate::{belongs_to, belongs_to_many, has_many, has_one};
    pub use pakem_macros::{lumina_form, pakem_form, LuminaModel, PakemModel};
    // financial & enterprise types
    pub use uuid::Uuid;
    pub use rust_decimal::Decimal;
    pub use rust_decimal_macros::dec;
    // middleware helpers
    pub use axum::middleware::{from_fn, from_fn_with_state, Next};
    pub use crate::http::middleware;
    // background queue jobs
    pub use crate::core::queue::{Job, Dispatchable, QueueManager, JobRegistry};
    pub use crate::core::auth::{Auth, AuthUser, HasRolesAndPermissions};
    pub use crate::core::security::BruteForceGuard;
    pub use crate::core::sandbox::{LuminaPluginManager, WasmPlugin, PluginHook, PluginStats, PluginMetricsTracker};
    pub use crate::core::tinker::{Tinker, TinkerEvaluator, TinkerResult, TinkerFormatter};
    pub use crate::core::hardware::{EscPosBuilder, BarcodeScanEvent, FingerprintTemplate, CameraConfig, DeviceMessage, DeviceResponse, device_bridge_script};
    pub use crate::core::ai::Ai;
}

pub use pakem_macros::{lumina_form, pakem_form, LuminaModel, PakemModel};
pub use validator;
pub use tower_sessions;
