use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use wasmi::{Caller, Engine, Instance, Linker, Memory, Module, Store};

pub mod is_runner;
pub use is_runner::{IsFunction, IsScriptRunner};


#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("Gagal mengompilasi modul WASM: {0}")]
    CompilationError(String),

    #[error("Gagal instansiasi modul WASM: {0}")]
    InstantiationError(String),

    #[error("Fungsi '{0}' tidak ditemukan dalam modul WASM")]
    FunctionNotFound(String),

    #[error("Memory export '{0}' tidak ditemukan")]
    MemoryNotFound(String),

    #[error("Gagal mengakses memori WASM: {0}")]
    MemoryAccessError(String),

    #[error("WASM Execution Trap/Panic: {0}")]
    Trap(String),

    #[error("Gagal serialisasi/deserialisasi JSON: {0}")]
    SerializationError(String),

    #[error("Plugin '{0}' tidak terdaftar")]
    PluginNotFound(String),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Host state yang dapat diakses oleh fungsi host di dalam sandbox WASM.
#[derive(Default, Clone, Debug)]
pub struct HostState {
    pub logs: Vec<(String, String)>,
    pub metadata: HashMap<String, String>,
}

/// Representasi plugin WebAssembly yang telah dikompilasi dan siap di-instansiasi.
#[derive(Clone)]
pub struct WasmPlugin {
    pub name: String,
    pub engine: Engine,
    pub module: Module,
}

impl WasmPlugin {
    /// Buat plugin dari WebAssembly Text format (WAT)
    pub fn from_wat(name: &str, wat: &str) -> Result<Self, SandboxError> {
        let engine = Engine::default();
        let module = Module::new(&engine, wat)
            .map_err(|e| SandboxError::CompilationError(e.to_string()))?;
        Ok(Self {
            name: name.to_string(),
            engine,
            module,
        })
    }

    /// Buat plugin dari biner WebAssembly (.wasm bytes)
    pub fn from_bytes(name: &str, bytes: &[u8]) -> Result<Self, SandboxError> {
        let engine = Engine::default();
        let module = Module::new(&engine, bytes)
            .map_err(|e| SandboxError::CompilationError(e.to_string()))?;
        Ok(Self {
            name: name.to_string(),
            engine,
            module,
        })
    }

    /// Buat plugin dari file .wasm di disk
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, SandboxError> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed_plugin");
        Self::from_bytes(name, &bytes)
    }

    /// Buat Linker standar Lumina dengan fungsi host bawaan
    fn create_linker(engine: &Engine) -> Result<Linker<HostState>, SandboxError> {
        let mut linker = Linker::<HostState>::new(engine);

        // Host Function: lumina_log(level_code, code)
        linker
            .func_wrap(
                "lumina_host",
                "lumina_log",
                |mut caller: Caller<'_, HostState>, level: i32, code: i32| {
                    let level_str = match level {
                        1 => "DEBUG",
                        2 => "INFO",
                        3 => "WARN",
                        4 => "ERROR",
                        _ => "LOG",
                    };

                    caller.data_mut().logs.push((level_str.to_string(), format!("Event code: {}", code)));
                },
            )
            .map_err(|e| SandboxError::InstantiationError(e.to_string()))?;



        // Host Function: lumina_now() -> i64
        linker
            .func_wrap("lumina_host", "lumina_now", |_caller: Caller<'_, HostState>| -> i64 {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as i64
            })
            .map_err(|e| SandboxError::InstantiationError(e.to_string()))?;

        Ok(linker)
    }

    /// Instansiasi plugin ke dalam Store dan environment eksekusi baru
    pub fn instantiate(&self) -> Result<WasmInstance, SandboxError> {
        let mut store = Store::new(&self.engine, HostState::default());
        let linker = Self::create_linker(&self.engine)?;

        let instance = linker
            .instantiate_and_start(&mut store, &self.module)
            .map_err(|e| SandboxError::InstantiationError(e.to_string()))?;

        Ok(WasmInstance { store, instance })
    }

    /// Eksekusi fungsi numerik sederhana (misal f(i32) -> i32)
    pub fn call_i32(&self, func_name: &str, arg: i32) -> Result<i32, SandboxError> {
        let mut instance = self.instantiate()?;
        instance.call_i32(func_name, arg)
    }

    /// Eksekusi transformasi string (misal: filter formatting, uppercase, format mata uang)
    pub fn call_string_transform(&self, func_name: &str, input: &str) -> Result<String, SandboxError> {
        let mut instance = self.instantiate()?;
        instance.call_string_transform(func_name, input)
    }
}

/// Instance aktif modul WASM dengan memori dan store terisolasi.
pub struct WasmInstance {
    pub store: Store<HostState>,
    pub instance: Instance,
}

impl WasmInstance {
    /// Eksekusi fungsi numerik (i32) -> i32
    pub fn call_i32(&mut self, func_name: &str, arg: i32) -> Result<i32, SandboxError> {
        let func = self
            .instance
            .get_typed_func::<i32, i32>(&self.store, func_name)
            .map_err(|_| SandboxError::FunctionNotFound(func_name.to_string()))?;

        func.call(&mut self.store, arg)
            .map_err(|e| SandboxError::Trap(e.to_string()))
    }

    /// Mendapatkan export memori dari guest WASM
    pub fn get_memory(&self) -> Result<Memory, SandboxError> {
        self.instance
            .get_memory(&self.store, "memory")
            .ok_or_else(|| SandboxError::MemoryNotFound("memory".to_string()))
    }

    /// Eksekusi transformasi string menggunakan shared memory buffer guest
    pub fn call_string_transform(&mut self, func_name: &str, input: &str) -> Result<String, SandboxError> {
        let mem = self.get_memory()?;
        let input_bytes = input.as_bytes();
        let input_len = input_bytes.len() as i32;

        // Cek apakah ada fungsi alloc
        let alloc_ptr = if let Ok(alloc_fn) = self.instance.get_typed_func::<i32, i32>(&self.store, "alloc") {
            alloc_fn
                .call(&mut self.store, input_len)
                .map_err(|e| SandboxError::Trap(format!("Alloc trap: {}", e)))?
        } else {
            0
        };

        // Tulis bytes input ke memory guest via mem.write
        mem.write(&mut self.store, alloc_ptr as usize, input_bytes)
            .map_err(|e| SandboxError::MemoryAccessError(e.to_string()))?;

        // Coba panggil dengan return i64 (gabungan pointer dan length)
        if let Ok(func) = self.instance.get_typed_func::<(i32, i32), i64>(&self.store, func_name) {
            let res = func
                .call(&mut self.store, (alloc_ptr, input_len))
                .map_err(|e| SandboxError::Trap(e.to_string()))?;

            let out_ptr = (res >> 32) as usize;
            let out_len = (res & 0xFFFF_FFFF) as usize;

            let mut out_buf = vec![0u8; out_len];
            mem.read(&self.store, out_ptr, &mut out_buf)
                .map_err(|e| SandboxError::MemoryAccessError(e.to_string()))?;

            let out_str = std::str::from_utf8(&out_buf)
                .map_err(|e| SandboxError::SerializationError(e.to_string()))?;

            return Ok(out_str.to_string());
        }

        // Coba panggil dengan return i32 (out_len di offset alloc_ptr)
        if let Ok(func) = self.instance.get_typed_func::<(i32, i32), i32>(&self.store, func_name) {
            let out_len = func
                .call(&mut self.store, (alloc_ptr, input_len))
                .map_err(|e| SandboxError::Trap(e.to_string()))? as usize;

            let mut out_buf = vec![0u8; out_len];
            mem.read(&self.store, alloc_ptr as usize, &mut out_buf)
                .map_err(|e| SandboxError::MemoryAccessError(e.to_string()))?;

            let out_str = std::str::from_utf8(&out_buf)
                .map_err(|e| SandboxError::SerializationError(e.to_string()))?;

            return Ok(out_str.to_string());
        }

        Err(SandboxError::FunctionNotFound(format!(
            "Fungsi '{}' tidak memiliki signature (i32, i32) -> i64 atau (i32, i32) -> i32",
            func_name
        )))
    }

}

/// Pengelola Plugin dan Ekstensi Lumina Sandbox (Thread-Safe & Hot-Reloadable)
#[derive(Clone, Default)]
pub struct LuminaPluginManager {
    plugins: Arc<RwLock<HashMap<String, Arc<WasmPlugin>>>>,
    is_scripts: Arc<RwLock<HashMap<String, Arc<IsScriptRunner>>>>,
}

impl LuminaPluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            is_scripts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Daftarkan plugin WASM ke manager
    pub fn register(&self, plugin: WasmPlugin) {
        let mut map = self.plugins.write().unwrap();
        map.insert(plugin.name.clone(), Arc::new(plugin));
    }

    /// Ambil plugin WASM berdasarkan nama
    pub fn get(&self, name: &str) -> Option<Arc<WasmPlugin>> {
        let map = self.plugins.read().unwrap();
        map.get(name).cloned()
    }

    /// Daftarkan skrip Indonesian Script (.is) yang telah diparsing ke cache
    pub fn register_is_script(&self, name: &str, runner: IsScriptRunner) {
        let mut map = self.is_scripts.write().unwrap();
        map.insert(name.to_string(), Arc::new(runner));
    }

    /// Ambil runner Indonesian Script dari cache
    pub fn get_is_script(&self, name: &str) -> Option<Arc<IsScriptRunner>> {
        let map = self.is_scripts.read().unwrap();
        map.get(name).cloned()
    }

    /// Evaluasi fungsi dari skrip Indonesian Script (.is) dengan input arguments JSON
    pub fn eval_is_script(&self, script_path: &str, func_name: &str, args: &[serde_json::Value]) -> Result<serde_json::Value, SandboxError> {
        let runner = if let Some(cached) = self.get_is_script(script_path) {
            cached
        } else {
            let candidates = [
                script_path.to_string(),
                format!("resources/scripts/{}", script_path),
                format!("resources/scripts/{}.is", script_path),
                format!("app/scripts/{}", script_path),
                format!("app/scripts/{}.is", script_path),
            ];

            let mut found_content = None;
            for candidate in &candidates {
                if let Ok(content) = std::fs::read_to_string(candidate) {
                    found_content = Some(content);
                    break;
                }
            }

            let content = found_content.ok_or_else(|| {
                SandboxError::IoError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("File skrip Indonesian Script '{}' tidak ditemukan", script_path),
                ))
            })?;

            let parsed = IsScriptRunner::from_source(&content)?;
            let arc_runner = Arc::new(parsed);
            let mut map = self.is_scripts.write().unwrap();
            map.insert(script_path.to_string(), arc_runner.clone());
            arc_runner
        };

        runner.call(func_name, args)
    }

    /// Panggil custom filter Blade yang terdaftar di WASM
    pub fn call_filter(&self, plugin_name: &str, func_name: &str, input: &str) -> Result<String, SandboxError> {
        let plugin = self.get(plugin_name).ok_or_else(|| SandboxError::PluginNotFound(plugin_name.to_string()))?;
        plugin.call_string_transform(func_name, input)
    }

    /// Pindai dan muat semua file .wasm dari sebuah direktori (misal `plugins/`)
    pub fn load_directory(&self, dir: impl AsRef<Path>) -> Result<usize, SandboxError> {
        let path = dir.as_ref();
        if !path.exists() {
            return Ok(0);
        }

        let mut loaded = 0;
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() && entry_path.extension().and_then(|s| s.to_str()) == Some("wasm") {
                if let Ok(plugin) = WasmPlugin::from_file(&entry_path) {
                    self.register(plugin);
                    loaded += 1;
                }
            }
        }

        Ok(loaded)
    }

    /// Mengambil daftar nama plugin WASM yang sedang aktif di-load
    pub fn list_wasm_plugins(&self) -> Vec<String> {
        let map = self.plugins.read().unwrap();
        map.keys().cloned().collect()
    }

    /// Mengambil daftar nama skrip Indonesian Script (.is) yang sedang aktif di cache
    pub fn list_is_scripts(&self) -> Vec<String> {
        let map = self.is_scripts.read().unwrap();
        map.keys().cloned().collect()
    }

    /// Mendapatkan ringkasan metrik sandbox untuk Telescope / Debug Monitor
    pub fn get_metrics(&self) -> SandboxMetrics {
        let wasm = self.list_wasm_plugins();
        let is = self.list_is_scripts();
        SandboxMetrics {
            wasm_plugins_count: wasm.len(),
            is_scripts_count: is.len(),
            wasm_plugins: wasm,
            is_scripts: is,
        }
    }
}

/// Metrik ringkasan ekosistem sandbox untuk Telescope & Monitor
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SandboxMetrics {
    pub wasm_plugins_count: usize,
    pub is_scripts_count: usize,
    pub wasm_plugins: Vec<String>,
    pub is_scripts: Vec<String>,
}

/// Handle ergonomis untuk mengeksekusi file skrip Indonesian Script di controller
#[derive(Clone)]
pub struct ScriptHandle {
    pub plugin_manager: Arc<LuminaPluginManager>,
    pub script_path: String,
}

impl ScriptHandle {
    pub fn new(plugin_manager: Arc<LuminaPluginManager>, script_path: &str) -> Self {
        Self {
            plugin_manager,
            script_path: script_path.to_string(),
        }
    }

    /// Panggil fungsi di dalam file skrip dengan arguments JSON
    pub fn call(&self, func_name: &str, args: serde_json::Value) -> Result<serde_json::Value, SandboxError> {
        let args_vec = match args {
            serde_json::Value::Array(arr) => arr,
            single => vec![single],
        };
        self.plugin_manager.eval_is_script(&self.script_path, func_name, &args_vec)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_numeric_execution() {
        // WAT sederhana: fungsi kuadrat angka f(x) = x * x
        let wat = r#"
            (module
                (func (export "kuadrat") (param i32) (result i32)
                    local.get 0
                    local.get 0
                    i32.mul
                )
            )
        "#;

        let plugin = WasmPlugin::from_wat("matematika", wat).expect("Gagal memuat WAT");
        let hasil = plugin.call_i32("kuadrat", 7).expect("Gagal eksekusi kuadrat");
        assert_eq!(hasil, 49);
    }

    #[test]
    fn test_wasm_host_logger_call() {
        // WAT yang memanggil host logger lumina_host::lumina_log(level=2, code=200)
        let wat = r#"
            (module
                (import "lumina_host" "lumina_log" (func $log (param i32 i32)))
                (func (export "sapa")
                    i32.const 2
                    i32.const 200
                    call $log
                )
            )
        "#;

        let plugin = WasmPlugin::from_wat("logger_test", wat).expect("Gagal compile");
        let mut instance = plugin.instantiate().expect("Gagal instantiate");

        let sapa_fn = instance
            .instance
            .get_typed_func::<(), ()>(&instance.store, "sapa")
            .expect("Fungsi sapa tidak ditemukan");

        sapa_fn.call(&mut instance.store, ()).expect("Eksekusi sapa gagal");

        // Periksa log di host state
        assert_eq!(instance.store.data().logs.len(), 1);
        assert_eq!(instance.store.data().logs[0].0, "INFO");
        assert_eq!(instance.store.data().logs[0].1, "Event code: 200");
    }


    #[test]
    fn test_wasm_string_transformation_filter() {
        // WAT: Filter yang mengubah huruf kecil 'a'-'z' menjadi huruf besar 'A'-'Z'
        let wat = r#"
            (module
                (memory (export "memory") 1)
                (func (export "ke_huruf_besar") (param $ptr i32) (param $len i32) (result i32)
                    (local $i i32)
                    (local $curr i32)
                    (local.set $i (i32.const 0))
                    (block $break
                        (loop $loop
                            (br_if $break (i32.ge_u (local.get $i) (local.get $len)))
                            (local.set $curr (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
                            ;; jika $curr >= 97 ('a') dan $curr <= 122 ('z'), kurangi 32
                            (if (i32.and (i32.ge_u (local.get $curr) (i32.const 97)) (i32.le_u (local.get $curr) (i32.const 122)))
                                (then
                                    (i32.store8
                                        (i32.add (local.get $ptr) (local.get $i))
                                        (i32.sub (local.get $curr) (i32.const 32))
                                    )
                                )
                            )
                            (local.set $i (i32.add (local.get $i) (i32.const 1)))
                            (br $loop)
                        )
                    )
                    local.get $len
                )
            )
        "#;

        let plugin = WasmPlugin::from_wat("text_filter", wat).expect("Compile filter gagal");
        let hasil = plugin
            .call_string_transform("ke_huruf_besar", "lumina framework indonesia")
            .expect("Transform string gagal");

        assert_eq!(hasil, "LUMINA FRAMEWORK INDONESIA");
    }

    #[test]
    fn test_wasm_fault_isolation_trap_does_not_crash_host() {
        // WAT yang sengaja memanggil instruksi 'unreachable' (panic)
        let wat = r#"
            (module
                (func (export "meledak") (param i32) (result i32)
                    unreachable
                )
            )
        "#;

        let plugin = WasmPlugin::from_wat("chaos_plugin", wat).expect("Compile chaos gagal");
        let hasil = plugin.call_i32("meledak", 100);

        // Host tidak boleh crash! Harus ditangkap sebagai SandboxError::Trap
        assert!(hasil.is_err());
        match hasil.err().unwrap() {
            SandboxError::Trap(msg) => {
                assert!(msg.contains("unreachable"));
            }
            other => panic!("Harusnya SandboxError::Trap, tapi dapat: {:?}", other),
        }
    }

    #[test]
    fn test_plugin_manager_registration_and_lookup() {
        let manager = LuminaPluginManager::new();
        let wat = r#"
            (module
                (func (export "tambah_satu") (param i32) (result i32)
                    local.get 0
                    i32.const 1
                    i32.add
                )
            )
        "#;

        let plugin = WasmPlugin::from_wat("helper", wat).expect("Compile helper gagal");
        manager.register(plugin);

        let retrieved = manager.get("helper");
        assert!(retrieved.is_some());

        let hasil = retrieved.unwrap().call_i32("tambah_satu", 41).unwrap();
        assert_eq!(hasil, 42);
    }

    #[test]
    fn test_plugin_manager_is_script_evaluation() {
        let manager = Arc::new(LuminaPluginManager::new());

        let source = r#"
        fungsi hitung_ongkir(berat, asal, tujuan) {
            ongkir = 10000;
            jika tujuan == "luar_jawa" {
                ongkir = ongkir * 2.5;
            }
            kembalikan ongkir * berat;
        }
        "#;

        let runner = IsScriptRunner::from_source(source).unwrap();
        manager.register_is_script("ongkir.is", runner);

        let handle = ScriptHandle::new(manager.clone(), "ongkir.is");

        // Ongkir Jawa: 10000 * 2 = 20000
        let res1 = handle.call("hitung_ongkir", serde_json::json!([2, "jakarta", "bandung"])).unwrap();
        assert_eq!(res1, serde_json::json!(20000.0));

        // Ongkir Luar Jawa: 25000 * 3 = 75000
        let res2 = handle.call("hitung_ongkir", serde_json::json!([3, "jakarta", "luar_jawa"])).unwrap();
        assert_eq!(res2, serde_json::json!(75000.0));
    }

    #[test]
    fn test_plugin_manager_get_metrics() {
        let manager = LuminaPluginManager::new();
        let wat = r#"
        (module
            (func (export "dummy") (param i32) (result i32)
                local.get 0
            )
        )
        "#;
        let plugin = WasmPlugin::from_wat("metric_test", wat).unwrap();
        manager.register(plugin);

        let script_source = "fungsi hitung(x) { kembalikan x * 2; }";
        let runner = IsScriptRunner::from_source(script_source).unwrap();
        manager.register_is_script("test_script.is", runner);

        let metrics = manager.get_metrics();
        assert_eq!(metrics.wasm_plugins_count, 1);
        assert_eq!(metrics.is_scripts_count, 1);
        assert!(metrics.wasm_plugins.contains(&"metric_test".to_string()));
        assert!(metrics.is_scripts.contains(&"test_script.is".to_string()));
    }
}

