use axum::{
    extract::{State, Query},
    response::Html,
};
use crate::core::application::AppState;
use crate::core::telescope::entry::EntryType;
use tera::Context;
use serde::Deserialize;

pub struct TelescopeController;

#[derive(Deserialize)]
pub struct TelescopeQuery {
    pub tag: Option<String>,
}

impl TelescopeController {
    /// GET /lumina/telescope
    pub async fn index(
        State(state): State<AppState>,
        Query(query): Query<TelescopeQuery>,
    ) -> Html<String> {
        let mut context = Context::new();
        
        let tag = query.tag.unwrap_or_else(|| "requests".to_string());
        context.insert("active_tag", &tag);

        let entries = match tag.as_str() {
            "requests" => state.telescope.get_by_type(EntryType::Request),
            "events" => state.telescope.get_by_type(EntryType::Event),
            "jobs" => state.telescope.get_by_type(EntryType::Job),
            "logs" => state.telescope.get_by_type(EntryType::Log),
            _ => state.telescope.all(),
        };

        context.insert("entries", &entries);
        
        let rendered = state.view.render("telescope/index.blade.rs", &context);
        Html(rendered)
    }

    /// GET /lumina/telescope/entries (Partial for HTMX)
    pub async fn entries(
        State(state): State<AppState>,
        Query(query): Query<TelescopeQuery>,
    ) -> Html<String> {
        let mut context = Context::new();
        let tag = query.tag.unwrap_or_else(|| "requests".to_string());
        
        let entries = match tag.as_str() {
            "requests" => state.telescope.get_by_type(EntryType::Request),
            "events" => state.telescope.get_by_type(EntryType::Event),
            "jobs" => state.telescope.get_by_type(EntryType::Job),
            "logs" => state.telescope.get_by_type(EntryType::Log),
            _ => state.telescope.all(),
        };

        context.insert("entries", &entries);
        context.insert("active_tag", &tag);
        
        let rendered = state.view.render("telescope/partials/entries.blade.rs", &context);
        Html(rendered)
    }
}
