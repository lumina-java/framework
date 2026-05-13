use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use crate::core::application::AppState;

#[derive(Clone, Debug)]
pub struct Locale(pub String);

pub async fn localization_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let mut locale = state.config.get_or("APP_LOCALE", "en");

    // 1. Cek Query Parameter (?lang=id)
    if let Some(query) = req.uri().query() {
        if let Some(lang) = query.split('&').find(|p| p.starts_with("lang=")).map(|p| &p[5..]) {
            locale = lang.to_string();
        }
    }
    
    // 2. Cek Accept-Language Header
    else if let Some(accept_lang) = req.headers().get("accept-language").and_then(|h| h.to_str().ok()) {
        let primary = accept_lang.split(',').next().unwrap_or("en").split('-').next().unwrap_or("en");
        locale = primary.to_string();
    }

    // Simpan locale ke dalam request extension agar bisa diakses oleh ViewEngine atau Handler
    req.extensions_mut().insert(Locale(locale));

    next.run(req).await
}
