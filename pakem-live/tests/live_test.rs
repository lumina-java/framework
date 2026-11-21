use async_trait::async_trait;
use lumina::core::blade::{BladeContext, BladeEngine};
use lumina::core::error::AppError;
use lumina_live::component::LiveComponent;
use lumina_live::registry::LiveRegistry;
use lumina_live::snapshot::{LiveSnapshot, SnapshotManager};
use lumina_live::transport::LUMINA_LIVE_JS;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCounter {
    pub count: i32,
    pub step: i32,
}

impl Default for TestCounter {
    fn default() -> Self {
        Self { count: 0, step: 1 }
    }
}

#[async_trait]
impl LiveComponent for TestCounter {
    fn name(&self) -> &'static str {
        "counter"
    }

    fn view(&self) -> &'static str {
        "live.counter"
    }

    async fn call_action(&mut self, action: &str, _params: Vec<Value>) -> Result<(), AppError> {
        match action {
            "tambah" | "increment" => self.count += self.step,
            "kurang" | "decrement" => self.count -= self.step,
            _ => return Err(AppError::BadRequest(format!("Aksi tidak dikenal: {}", action))),
        }
        Ok(())
    }
}

#[test]
fn test_live_snapshot_seal_and_tamper_proofing() {
    let snapshot = LiveSnapshot::new("c-1234", "counter", json!({ "count": 10, "step": 2 }));

    let token = SnapshotManager::seal(&snapshot).expect("Gagal membuat snapshot");
    assert!(!token.is_empty());
    assert!(token.contains('.'));

    let unsealed = SnapshotManager::unseal(&token).expect("Gagal memverifikasi snapshot sah");
    assert_eq!(unsealed.id, "c-1234");
    assert_eq!(unsealed.name, "counter");
    assert_eq!(unsealed.data["count"], 10);
    assert_eq!(unsealed.data["step"], 2);

    let tampered_sig = format!("{}.deadbeef12345678", token.split('.').next().unwrap());
    let err = SnapshotManager::unseal(&tampered_sig);
    assert!(err.is_err(), "Snapshot dengan signature palsu harus ditolak!");
}

#[tokio::test]
async fn test_live_registry_and_component_action_dispatch() {
    let mut engine = BladeEngine::new();
    engine.templates.insert(
        "live.counter".to_string(),
        r#"<span class="count">{{ $count }}</span><button kilat:klik="tambah">+</button>"#.to_string(),
    );

    let registry = LiveRegistry::new();
    registry.register::<TestCounter>("counter");

    let handler = registry.get("counter").expect("Handler counter harus terdaftar");

    let (initial_state, initial_html) = handler
        .render_initial(json!({ "count": 0, "step": 5 }), &engine)
        .expect("Gagal render awal");

    assert_eq!(initial_state["count"], 0);
    assert!(initial_html.contains(r#"<span class="count">0</span>"#));

    let (updated_state, updated_html) = handler
        .call(initial_state, "tambah", vec![], &engine)
        .await
        .expect("Gagal eksekusi aksi tambah");

    assert_eq!(updated_state["count"], 5);
    assert!(updated_html.contains(r#"<span class="count">5</span>"#));
}

#[test]
fn test_blade_engine_kilat_directives_via_lumina_live() {
    lumina_live::init();

    let mut engine = BladeEngine::new();
    engine.templates.insert(
        "live.counter".to_string(),
        r#"<h1>Nilai: {{ $count }}</h1>"#.to_string(),
    );

    LiveRegistry::global().register::<TestCounter>("counter");

    let layout_template = "@kilatScripts";
    let rendered_scripts = engine.render_string(layout_template, &BladeContext::new());
    assert!(rendered_scripts.contains(r#"<script src="/_lumina/live/live.js" defer></script>"#));

    let page_template = "@kilat('counter')";
    let rendered_page = engine.render_string(page_template, &BladeContext::new());

    assert!(rendered_page.contains(r#"data-kilat-id="c-"#));
    assert!(rendered_page.contains(r#"data-kilat-name="counter""#));
    assert!(rendered_page.contains("<h1>Nilai: 0</h1>"));

    let page_with_args = "@kilat('counter', ['count' => 42, 'step' => 7])";
    let rendered_with_args = engine.render_string(page_with_args, &BladeContext::new());
    assert!(rendered_with_args.contains("<h1>Nilai: 42</h1>"));

    let page_with_json = "@live('counter', {\"count\": 99})";
    let rendered_with_json = engine.render_string(page_with_json, &BladeContext::new());
    assert!(rendered_with_json.contains("<h1>Nilai: 99</h1>"));
}

#[test]
fn test_lumina_live_js_contains_tri_dialect_support() {
    assert!(LUMINA_LIVE_JS.contains("kilat:"));
    assert!(LUMINA_LIVE_JS.contains("live:"));
    assert!(LUMINA_LIVE_JS.contains("wire:"));
    assert!(LUMINA_LIVE_JS.contains("'klik': 'click'"));
}
