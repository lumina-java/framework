use axum::{extract::{State, Path}, Json};
use std::sync::Arc;
use serde_json::{json, Value};
use crate::core::application::AppState;
use crate::database::model::Model;
use crate::app::models::product::Product;

pub struct ProductController;

impl ProductController {
    /// GET /api/products — Daftar semua produk
    pub async fn index(State(state): State<Arc<AppState>>) -> Json<Value> {
        let db = state.db.as_ref().expect("db_guard seharusnya mencegah ini");
        match Product::all(db).await {
            Ok(products) => Json(json!({
                "status": "success",
                "data": products
            })),
            Err(e) => Json(json!({
                "status": "error",
                "message": e.to_string()
            })),
        }
    }

    /// GET /api/products/:id — Detail produk berdasarkan ID
    pub async fn show(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i64>
    ) -> Json<Value> {
        let db = state.db.as_ref().expect("db_guard seharusnya mencegah ini");
        match Product::find(db, id).await {
            Ok(product) => Json(json!({
                "status": "success",
                "data": product
            })),
            Err(_) => Json(json!({
                "status": "error",
                "message": "Product not found"
            })),
        }
    }
}
