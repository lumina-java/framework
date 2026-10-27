use async_trait::async_trait;
use lumina::core::blade::{BladeContext, BladeEngine};
use lumina::core::error::AppError;
use lumina::core::live::component::LiveComponent;
use lumina::core::live::registry::LiveRegistry;
use lumina::core::live::snapshot::{LiveSnapshot, SnapshotManager};
use lumina::core::live::transport::LUMINA_LIVE_JS;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// Komponen Counter Uji Coba
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

    // 1. Seal snapshot
    let token = SnapshotManager::seal(&snapshot).expect("Gagal membuat snapshot");
    assert!(!token.is_empty());
    assert!(token.contains('.'));

    // 2. Unseal snapshot sah
    let unsealed = SnapshotManager::unseal(&token).expect("Gagal memverifikasi snapshot sah");
    assert_eq!(unsealed.id, "c-1234");
    assert_eq!(unsealed.name, "counter");
    assert_eq!(unsealed.data["count"], 10);
    assert_eq!(unsealed.data["step"], 2);

    // 3. Simulasi tampering di DevTools (ubah signature)
    let tampered_sig = format!("{}.deadbeef12345678", token.split('.').next().unwrap());
    let err = SnapshotManager::unseal(&tampered_sig);
    assert!(err.is_err(), "Snapshot dengan signature palsu harus ditolak!");

    // 4. Simulasi tampering di DevTools (ubah data tanpa ubah signature)
    let fake_data = hex::encode(r#"{"id":"c-1234","name":"counter","data":{"count":999999}}"#.as_bytes());
    let sig = token.split('.').nth(1).unwrap();
    let tampered_payload = format!("{}.{}", fake_data, sig);
    let err2 = SnapshotManager::unseal(&tampered_payload);
    assert!(err2.is_err(), "Snapshot dengan payload dimodifikasi harus ditolak!");
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

    // 1. Initial render
    let (initial_state, initial_html) = handler
        .render_initial(json!({ "count": 0, "step": 5 }), &engine)
        .expect("Gagal render awal");

    assert_eq!(initial_state["count"], 0);
    assert!(initial_html.contains(r#"<span class="count">0</span>"#));

    // 2. Dispatch aksi "tambah"
    let (updated_state, updated_html) = handler
        .call(initial_state, "tambah", vec![], &engine)
        .await
        .expect("Gagal eksekusi aksi tambah");

    assert_eq!(updated_state["count"], 5);
    assert!(updated_html.contains(r#"<span class="count">5</span>"#));

    // 3. Dispatch aksi "kurang"
    let (final_state, final_html) = handler
        .call(updated_state, "kurang", vec![], &engine)
        .await
        .expect("Gagal eksekusi aksi kurang");

    assert_eq!(final_state["count"], 0);
    assert!(final_html.contains(r#"<span class="count">0</span>"#));
}

#[test]
fn test_blade_engine_kilat_directives() {
    let mut engine = BladeEngine::new();
    engine.templates.insert(
        "live.counter".to_string(),
        r#"<h1>Nilai: {{ $count }}</h1>"#.to_string(),
    );

    // Register counter to global registry
    LiveRegistry::global().register::<TestCounter>("counter");

    // Test @kilatScripts injection
    let layout_template = "@kilatScripts";
    let rendered_scripts = engine.render_string(layout_template, &BladeContext::new());
    assert!(rendered_scripts.contains(r#"<script src="/_lumina/live/live.js" defer></script>"#));

    // Test @kilat('counter') rendering
    let page_template = "@kilat('counter')";
    let rendered_page = engine.render_string(page_template, &BladeContext::new());

    assert!(rendered_page.contains(r#"data-kilat-id="c-"#));
    assert!(rendered_page.contains(r#"data-kilat-name="counter""#));
    assert!(rendered_page.contains(r#"data-kilat-snapshot=""#));
    assert!(rendered_page.contains("<h1>Nilai: 0</h1>"));
}

#[test]
fn test_lumina_live_js_contains_tri_dialect_support() {
    // Verifikasi runtime JS memuat ketiga dialek: Indonesia (kilat), Universal (live), Legacy (wire)
    assert!(LUMINA_LIVE_JS.contains("kilat:"));
    assert!(LUMINA_LIVE_JS.contains("live:"));
    assert!(LUMINA_LIVE_JS.contains("wire:"));

    // Verifikasi 22 direktif kunci
    assert!(LUMINA_LIVE_JS.contains("'klik': 'click'"));
    assert!(LUMINA_LIVE_JS.contains("'kirim': 'submit'"));
    assert!(LUMINA_LIVE_JS.contains("'tunggu': 'loading'"));
    assert!(LUMINA_LIVE_JS.contains("'pantau': 'poll'"));
    assert!(LUMINA_LIVE_JS.contains("'yakin': 'confirm'"));
    assert!(LUMINA_LIVE_JS.contains("'navigasi': 'navigate'"));
    assert!(LUMINA_LIVE_JS.contains("'selubung': 'cloak'"));
    assert!(LUMINA_LIVE_JS.contains("'luring': 'offline'"));
    assert!(LUMINA_LIVE_JS.contains("'abaikan': 'ignore'"));
    assert!(LUMINA_LIVE_JS.contains("'alir': 'stream'"));
}
