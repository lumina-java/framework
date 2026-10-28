pub mod component;
pub mod directives;
pub mod extension;
pub mod prelude;
pub mod registry;
pub mod snapshot;
pub mod transport;

pub use component::{LiveComponent, parse_blade_args_to_json};
pub use extension::LiveApplicationExt;
pub use registry::{ComponentHandler, LiveComponentHandler, LiveRegistry};
pub use snapshot::{LiveSnapshot, SnapshotManager};
pub use transport::{call_action_handler, live_ws_handler, serve_live_js, LiveCallRequest, LiveCallResponse};

/// Inisialisasi otomatis direktif Lumina Live ke dalam Blade Engine.
pub fn init() {
    directives::register_live_directives();
}

/// Menghasilkan Axum router berisi endpoint Lumina Live:
/// - `GET  /_lumina/live/live.js`
/// - `POST /_lumina/live/call`
/// - `GET  /_lumina/live/ws`
pub fn routes() -> axum::Router<lumina::prelude::AppState> {
    transport::live_routes()
}
