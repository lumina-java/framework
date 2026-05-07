use lumina_macros::LuminaModel;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, LuminaModel, Clone)]
#[table("categories")]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}
