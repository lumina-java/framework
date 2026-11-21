use pakem::prelude::*;
use std::fs;
use uuid::Uuid;

#[test]
fn test_wasm_lifecycle_hooks_and_metrics() {
    let manager = LuminaPluginManager::new();

    // Plugin 1: Middleware Auth & Header Validator
    let wat_auth = r#"
    (module
        (memory (export "memory") 1)
        (func (export "alloc") (param i32) (result i32)
            i32.const 0
        )
        (func (export "lumina_on_request") (param i32 i32) (result i32)
            ;; Selalu kembalikan length output 10
            i32.const 10
        )
        (func (export "lumina_before_save") (param i32 i32) (result i32)
            i32.const 8
        )
    )
    "#;

    let plugin = WasmPlugin::from_wat("auth_guard", wat_auth).expect("Compile wat_auth gagal");
    manager.register(plugin);

    // 1. Uji Pemanggilan Hook OnRequest
    let results = manager.call_hook(PluginHook::OnRequest, "test_payload");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, "auth_guard");
    assert!(results[0].1.is_ok());

    // 2. Uji Pemanggilan Hook BeforeSave
    let results = manager.call_hook(PluginHook::BeforeSave, "user_data");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, "auth_guard");
    assert!(results[0].1.is_ok());

    // 3. Uji Pemanggilan Hook AfterResponse (tidak diekspor oleh auth_guard -> diabaikan)
    let results = manager.call_hook(PluginHook::AfterResponse, "response_payload");
    assert_eq!(results.len(), 0);

    // 4. Verifikasi Metrik Pelacakan
    let stats = manager.plugin_stats();
    assert_eq!(stats.len(), 1);
    let auth_stat = &stats[0];
    assert_eq!(auth_stat.name, "auth_guard");
    assert_eq!(auth_stat.calls_count, 2); // 2 sukses hook (OnRequest & BeforeSave)
    assert_eq!(auth_stat.errors_count, 0);
    assert!(auth_stat.last_executed_at.is_some());
}

#[test]
fn test_wasm_fault_isolation_and_error_metric() {
    let manager = LuminaPluginManager::new();

    // Plugin Chaos yang memicu trap/unreachable
    let wat_chaos = r#"
    (module
        (memory (export "memory") 1)
        (func (export "alloc") (param i32) (result i32)
            i32.const 0
        )
        (func (export "crash_me") (param i32 i32) (result i32)
            unreachable
        )
    )
    "#;

    let plugin = WasmPlugin::from_wat("chaos_engine", wat_chaos).expect("Compile wat_chaos gagal");
    manager.register(plugin);

    // Eksekusi fungsi yang memicu trap
    let res = manager.call_filter("chaos_engine", "crash_me", "boom");
    assert!(res.is_err(), "Seharusnya mengembalikan Err Trap dan host tidak panic");

    // Verifikasi error tercatat di metrik
    let stats = manager.plugin_stats();
    let chaos_stat = stats.iter().find(|s| s.name == "chaos_engine").unwrap();
    assert_eq!(chaos_stat.calls_count, 1);
    assert_eq!(chaos_stat.errors_count, 1);
    assert!(chaos_stat.last_error.is_some());
}

#[test]
fn test_wasm_directory_hot_reloading() {
    let temp_dir = std::env::temp_dir().join(format!("lumina_wasm_test_{}", Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();

    let plugin_file = temp_dir.join("greeter.wasm");

    // 1. Versi 1 dari Plugin Greeter
    let wat_v1 = r#"
    (module
        (memory (export "memory") 1)
        (data (i32.const 0) "Halo v1")
        (func (export "alloc") (param i32) (result i32)
            i32.const 64
        )
        (func (export "greet") (param i32 i32) (result i64)
            ;; offset 0 (di 32 bit atas), length 7 (di 32 bit bawah) -> (0 << 32) | 7 = 7
            i64.const 7
        )
    )
    "#;
    // Tulis binary/text hasil compile ke disk
    fs::write(&plugin_file, wat_v1.as_bytes()).unwrap();

    let manager = LuminaPluginManager::new();
    let count = manager.load_directory(&temp_dir).unwrap();
    assert_eq!(count, 1);

    let res_v1 = manager.call_filter("greeter", "greet", "test").unwrap();
    assert_eq!(res_v1, "Halo v1");

    // 2. Timpa dengan Versi 2
    let wat_v2 = r#"
    (module
        (memory (export "memory") 1)
        (data (i32.const 0) "Halo v2")
        (func (export "alloc") (param i32) (result i32)
            i32.const 64
        )
        (func (export "greet") (param i32 i32) (result i64)
            ;; offset 0 (di 32 bit atas), length 7 (di 32 bit bawah) -> (0 << 32) | 7 = 7
            i64.const 7
        )
    )
    "#;
    fs::write(&plugin_file, wat_v2.as_bytes()).unwrap();

    // 3. Reload manual on-demand
    let reload_count = manager.reload(&temp_dir).unwrap();
    assert_eq!(reload_count, 1);

    // 4. Verifikasi instan: Hasil otomatis terbarui menjadi v2!
    let res_v2 = manager.call_filter("greeter", "greet", "test").unwrap();
    assert_eq!(res_v2, "Halo v2");

    // Bersihkan folder temp
    let _ = fs::remove_dir_all(&temp_dir);
}
