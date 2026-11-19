pub mod evaluator;
pub mod formatter;

pub use evaluator::{TinkerEvaluator, TinkerResult};
pub use formatter::TinkerFormatter;

use crate::database::connection::DatabasePool;
use std::sync::Arc;

/// Facade utama untuk Lumina Tinker REPL Engine
pub struct Tinker;

impl Tinker {
    /// Inisialisasi evaluator Tinker dengan database pool opsional
    pub fn evaluator(pool: Option<Arc<DatabasePool>>) -> TinkerEvaluator {
        TinkerEvaluator::new(pool)
    }

    /// Evaluasi satu baris perintah ekspresi secara instan
    pub async fn eval(input: &str, pool: Option<Arc<DatabasePool>>) -> TinkerResult {
        Self::evaluator(pool).eval(input).await
    }
}
