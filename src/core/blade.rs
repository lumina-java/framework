use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, LazyLock};


mod html_escape {
    pub fn encode_safe(input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
}

/// BladeContext — Menyimpan data dan variabel untuk dirender ke dalam template Blade.
/// Mendukung nested properties (dot-notation), array indexing, dan serialisasi data dinamis.
#[derive(Clone, Debug, Default)]
pub struct BladeContext {
    data: HashMap<String, Value>,
}

impl BladeContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Masukkan data serializable ke dalam context.
    pub fn insert<T: serde::Serialize>(&mut self, key: &str, value: T) {
        let clean_key = key.trim().trim_start_matches('$');
        if let Ok(val) = serde_json::to_value(value) {
            self.data.insert(clean_key.to_string(), val);
        }
    }

    /// Masukkan Value serde_json langsung.
    pub fn insert_val(&mut self, key: &str, value: Value) {
        let clean_key = key.trim().trim_start_matches('$');
        self.data.insert(clean_key.to_string(), value);
    }

    /// Ambil nilai variabel. Mendukung dot-notation (misal: "user.profile.name" atau "items.0.title").
    pub fn get(&self, key: &str) -> Option<&Value> {
        let key = key.trim().trim_start_matches('$');
        if !key.contains('.') {
            return self.data.get(key);
        }

        let parts: Vec<&str> = key.split('.').collect();
        let mut curr = self.data.get(parts[0])?;
        for part in &parts[1..] {
            if let Some(obj) = curr.as_object() {
                curr = obj.get(*part)?;
            } else if let Ok(idx) = part.parse::<usize>() {
                if let Some(arr) = curr.as_array() {
                    curr = arr.get(idx)?;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        Some(curr)
    }

    /// Cek apakah sebuah key ada di dalam context.
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Cek apakah ada pesan error validasi untuk kolom tertentu.
    /// Cek apakah user memiliki izin tertentu (RBAC support di Blade view)
    pub fn check_user_can(&self, permission: &str) -> bool {
        if let Some(user_val) = self.get("user") {
            if let Some(role) = user_val.get("role").and_then(|r| r.as_str()) {
                if role == "superadmin" || role == "admin" {
                    return true;
                }
            }
            if let Some(perms) = user_val.get("permissions").and_then(|p| p.as_array()) {
                if perms.iter().any(|p| p.as_str() == Some(permission) || p.as_str() == Some("*")) {
                    return true;
                }
            }
        }
        if let Some(perms_val) = self.get("permissions") {
            if let Some(perms) = perms_val.as_array() {
                if perms.iter().any(|p| p.as_str() == Some(permission) || p.as_str() == Some("*")) {
                    return true;
                }
            }
        }
        false
    }

    /// Cek apakah user memiliki peran tertentu (RBAC support di Blade view)
    pub fn check_user_has_role(&self, role: &str) -> bool {
        if let Some(user_val) = self.get("user") {
            if let Some(r) = user_val.get("role").and_then(|r| r.as_str()) {
                return r == role || r == "superadmin";
            }
        }
        false
    }

    pub fn has_error(&self, field: &str) -> bool {
        self.get_error(field).is_some()
    }

    /// Ambil pesan error validasi untuk kolom tertentu jika ada.
    pub fn get_error(&self, field: &str) -> Option<String> {
        let clean_field = field.trim().trim_matches('\'').trim_matches('"');
        let errs = self.get("errors")?;
        if let Some(obj) = errs.as_object() {
            if let Some(val) = obj.get(clean_field) {
                match val {
                    Value::String(s) if !s.trim().is_empty() => return Some(s.clone()),
                    Value::Array(arr) if !arr.is_empty() => {
                        if let Some(first) = arr.first() {
                            if let Some(s) = first.as_str() {
                                return Some(s.to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        None
    }

    /// Cek apakah suatu nilai bertipe truthy menurut aturan Blade/Laravel.
    pub fn is_truthy(val: Option<&Value>) -> bool {
        match val {
            None => false,
            Some(Value::Null) => false,
            Some(Value::Bool(b)) => *b,
            Some(Value::Number(n)) => n.as_f64().map(|f| f != 0.0).unwrap_or(false),
            Some(Value::String(s)) => {
                let trimmed = s.trim();
                !trimmed.is_empty() && trimmed != "0" && trimmed != "false"
            }
            Some(Value::Array(arr)) => !arr.is_empty(),
            Some(Value::Object(obj)) => !obj.is_empty(),
        }
    }

    /// Evaluasi nilai ekspresi variabel/literal menjadi String.
    /// Mendukung null coalescing: `$name ?? 'Default'`
    pub fn eval_val(&self, expr: &str) -> String {
        let expr = expr.trim();
        if expr.is_empty() {
            return String::new();
        }

        // Null coalescing: expr1 ?? expr2
        if let Some(idx) = expr.find("??") {
            let left = expr[..idx].trim();
            let right = expr[idx + 2..].trim();
            let val = self.eval_val(left);
            if !val.is_empty() {
                return val;
            } else {
                return self.eval_val(right);
            }
        }

        // String literal: '...' atau "..."
        if (expr.starts_with('\'') && expr.ends_with('\'') && expr.len() >= 2)
            || (expr.starts_with('"') && expr.ends_with('"') && expr.len() >= 2)
        {
            return expr[1..expr.len() - 1].to_string();
        }

        // Boolean literal
        if expr == "true" {
            return "true".to_string();
        }
        if expr == "false" {
            return "false".to_string();
        }

        // Number literal
        if expr.parse::<f64>().is_ok() {
            return expr.to_string();
        }

        // Helper Laravel: old('field', 'default')
        if expr.starts_with("old(") && expr.ends_with(')') {
            let inner = expr[4..expr.len() - 1].trim();
            let parts: Vec<&str> = inner.splitn(2, ',').collect();
            let field = parts[0].trim().trim_matches('\'').trim_matches('"');
            let default_val = if parts.len() > 1 {
                self.eval_val(parts[1].trim())
            } else {
                String::new()
            };

            if let Some(old_val) = self.get("old") {
                if let Some(obj) = old_val.as_object() {
                    if let Some(val) = obj.get(field) {
                        match val {
                            Value::String(s) => return s.clone(),
                            Value::Number(n) => return n.to_string(),
                            Value::Bool(b) => return b.to_string(),
                            Value::Null => {},
                            other => return other.to_string(),
                        }
                    }
                }
            }
            return default_val;
        }

        // Helper Laravel: asset('path')
        if expr.starts_with("asset(") && expr.ends_with(')') {
            let raw = expr[6..expr.len() - 1].trim().trim_matches('\'').trim_matches('"');
            if raw.starts_with("http://") || raw.starts_with("https://") || raw.starts_with("//") {
                return raw.to_string();
            }
            let clean = raw.trim_start_matches('/');
            return format!("/{}", clean);
        }

        // Helper Laravel: url('path') / route('path')
        if (expr.starts_with("url(") && expr.ends_with(')')) || (expr.starts_with("route(") && expr.ends_with(')')) {
            let open_paren = expr.find('(').unwrap();
            let raw = expr[open_paren + 1..expr.len() - 1].trim().trim_matches('\'').trim_matches('"');
            if raw.starts_with("http://") || raw.starts_with("https://") {
                return raw.to_string();
            }
            let clean = raw.trim_start_matches('/');
            return format!("/{}", clean);
        }

        // Variable lookup
        if let Some(val) = self.get(expr) {
            match val {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => String::new(),
                v => v.to_string(),
            }
        } else {
            String::new()
        }
    }

    /// Evaluasi kondisi boolean (truthy) untuk direktif @if, @unless, @checked, dll.
    pub fn eval_truthy(&self, expr: &str) -> bool {
        let expr = expr.trim();
        if expr.is_empty() {
            return false;
        }

        // Hapus tanda kurung terluar jika ada: ( expr )
        if expr.starts_with('(') && expr.ends_with(')') {
            let inner = &expr[1..expr.len() - 1];
            return self.eval_truthy(inner);
        }

        // Logical OR: ||
        if let Some(idx) = expr.find("||") {
            let left = &expr[..idx];
            let right = &expr[idx + 2..];
            return self.eval_truthy(left) || self.eval_truthy(right);
        }

        // Logical AND: &&
        if let Some(idx) = expr.find("&&") {
            let left = &expr[..idx];
            let right = &expr[idx + 2..];
            return self.eval_truthy(left) && self.eval_truthy(right);
        }

        // Unary NOT: !expr
        if expr.starts_with('!') {
            return !self.eval_truthy(&expr[1..]);
        }

        // Special helpers:
        // auth() / auth
        if expr == "auth" || expr == "auth()" {
            let uid = self.get("user_id").or_else(|| self.get("user")).or_else(|| self.get("auth"));
            return Self::is_truthy(uid);
        }
        // guest() / guest
        if expr == "guest" || expr == "guest()" {
            let uid = self.get("user_id").or_else(|| self.get("user")).or_else(|| self.get("auth"));
            return !Self::is_truthy(uid);
        }
        // can('permission')
        if expr.starts_with("can(") && expr.ends_with(')') {
            let perm = expr[4..expr.len() - 1].trim().trim_matches('\'').trim_matches('"');
            return self.check_user_can(perm);
        }
        // cannot('permission')
        if expr.starts_with("cannot(") && expr.ends_with(')') {
            let perm = expr[7..expr.len() - 1].trim().trim_matches('\'').trim_matches('"');
            return !self.check_user_can(perm);
        }
        // role('role')
        if expr.starts_with("role(") && expr.ends_with(')') {
            let role_name = expr[5..expr.len() - 1].trim().trim_matches('\'').trim_matches('"');
            return self.check_user_has_role(role_name);
        }
        // isset($var)
        if expr.starts_with("isset(") && expr.ends_with(')') {
            let var = &expr[6..expr.len() - 1].trim();
            return self.get(var).is_some() && !self.get(var).unwrap().is_null();
        }
        // empty($var)
        if expr.starts_with("empty(") && expr.ends_with(')') {
            let var = &expr[6..expr.len() - 1].trim();
            return !Self::is_truthy(self.get(var));
        }
        // production / production()
        if expr == "production" || expr == "production()" {
            let env = self.get("env").and_then(|v| v.as_str()).unwrap_or("");
            return env == "production" || env == "prod" || std::env::var("APP_ENV").unwrap_or_default() == "production";
        }
        // env('local', 'staging')
        if expr.starts_with("env(") && expr.ends_with(')') {
            let args = &expr[4..expr.len() - 1];
            let current_env = self.get("env").and_then(|v| v.as_str())
                .unwrap_or_else(|| "local");
            for arg in args.split(',') {
                let clean_arg = arg.trim().trim_matches('\'').trim_matches('"');
                if clean_arg == current_env {
                    return true;
                }
            }
            return false;
        }

        // error('field') / hasError('field')
        if (expr.starts_with("error(") && expr.ends_with(')')) || (expr.starts_with("hasError(") && expr.ends_with(')')) {
            let open = expr.find('(').unwrap();
            let field = expr[open + 1..expr.len() - 1].trim();
            return self.has_error(field);
        }

        // anyErrors / errors.any / $errors->any() / errors
        if expr == "errors" || expr == "errors()" || expr == "$errors->any()" || expr == "errors.any" || expr == "anyErrors" {
            if let Some(val) = self.get("errors") {
                if let Some(obj) = val.as_object() {
                    return !obj.is_empty();
                }
            }
            return false;
        }

        // Perbandingan: ===, ==, !=, >=, <=, >, <
        for op in &["===", "==", "!=", ">=", "<=", ">", "<"] {
            if let Some(idx) = expr.find(op) {
                let left = self.eval_val(&expr[..idx]);
                let right = self.eval_val(&expr[idx + op.len()..]);

                // Perbandingan numerik jika kedua sisi angka
                if let (Ok(n1), Ok(n2)) = (left.parse::<f64>(), right.parse::<f64>()) {
                    return match *op {
                        "===" | "==" => (n1 - n2).abs() < f64::EPSILON,
                        "!=" => (n1 - n2).abs() >= f64::EPSILON,
                        ">=" => n1 >= n2,
                        "<=" => n1 <= n2,
                        ">" => n1 > n2,
                        "<" => n1 < n2,
                        _ => false,
                    };
                }

                // Perbandingan string
                return match *op {
                    "===" | "==" => left == right,
                    "!=" => left != right,
                    ">=" => left >= right,
                    "<=" => left <= right,
                    ">" => left > right,
                    "<" => left < right,
                    _ => false,
                };
            }
        }

        // Boolean literal
        if expr == "true" {
            return true;
        }
        if expr == "false" {
            return false;
        }

        Self::is_truthy(self.get(expr))
    }
}

/// BladeEngine — Pure Native Laravel Blade Templating Engine for Rust.
/// Sangat cepat, tanpa alokasi berat, mendukung layout inheritance, loop variables,
/// conditional directives, integrasi Indonesian Script, serta Filter & Ekstensi WASM Sandbox.
pub struct BladeEngine {
    pub templates: HashMap<String, String>,
    pub plugin_manager: Arc<crate::core::sandbox::LuminaPluginManager>,
    pub wasm_filters: HashMap<String, (String, String)>,
}

impl Default for BladeEngine {
    fn default() -> Self {
        Self::empty()
    }
}



pub type CustomDirectiveHandler = std::sync::Arc<dyn Fn(&str, &BladeEngine, &BladeContext) -> String + Send + Sync>;

#[derive(Clone)]
pub struct RegisteredDirective {
    pub name: String,
    pub re_args: Regex,
    pub re_no_args: Regex,
    pub handler: CustomDirectiveHandler,
}

static RE_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)\{\{--.*?--\}\}").unwrap());
static RE_METHOD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@method\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_WASM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@wasm\s*\(\s*['"]([^'"]+)['"]\s*,\s*['"]([^'"]+)['"]\s*(?:,\s*(.*?))?\s*\)"#).unwrap());
static RE_EVAL_IS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@eval_is\s*\(\s*['"]([^'"]+)['"]\s*,\s*['"]([^'"]+)['"]\s*(?:,\s*(.*?))?\s*\)"#).unwrap());
static RE_RAW: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{!!\s*(.*?)\s*!!\}").unwrap());
static RE_VAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{\{\s*(.*?)\s*\}\}").unwrap());
static RE_UNLESS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@unless\s*\((.*?)\)").unwrap());
static RE_ISSET: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@isset\s*\((.*?)\)").unwrap());
static RE_EMPTY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@empty\s*\((.*?)\)").unwrap());
static RE_AUTH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@auth\b").unwrap());
static RE_GUEST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@guest\b").unwrap());
static RE_CAN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@can\s*\((.*?)\)").unwrap());
static RE_CANNOT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@cannot\s*\((.*?)\)").unwrap());
static RE_ROLE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@role\s*\((.*?)\)").unwrap());
static RE_PROD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@production\b").unwrap());
static RE_ENV: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@env\s*\((.*?)\)").unwrap());
static RE_EXTENDS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@extends\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_SHORT_SECTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@section\s*\(\s*['"]([^'"]+)['"]\s*,\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_SECTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)@section\s*\(\s*['"]([^'"]+)['"]\s*\)(.*?)@(?:endsection|stop|show)"#).unwrap());
static RE_YIELD_DEFAULT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@yield\s*\(\s*['"]([^'"]+)['"]\s*,\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_YIELD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@yield\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_HAS_SEC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@hasSection\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_INC_WHEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@includeWhen\s*\((.*?)\s*,\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_INC_UNLESS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@includeUnless\s*\((.*?)\s*,\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_INC_IF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@includeIf\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_INC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@include\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_FOREACH_OPEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@foreach\s*\((.*?)\)").unwrap());
static RE_FORELSE_OPEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@forelse\s*\((.*?)\)").unwrap());
static RE_FOR_OPEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@for\s*\((.*?)\)").unwrap());
static RE_INIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$?(\w+)\s*=\s*(\d+)").unwrap());
static RE_COND: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<\s*(\d+)").unwrap());
static RE_SWITCH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)@switch\s*\((.*?)\)(.*?)@endswitch"#).unwrap());
static RE_CASES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)@case\s*\(\s*['"]?([^'"]+)['"]?\s*\)(.*?)(?:@break|@case|@default|$)"#).unwrap());
static RE_DEFAULT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)@default(.*?)(?:@break|$)"#).unwrap());
static RE_ERROR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)@error\s*\(\s*['"]([^'"]+)['"]\s*\)(.*?)@enderror"#).unwrap());
static RE_CHECKED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@checked\s*\((.*?)\)").unwrap());
static RE_SELECTED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@selected\s*\((.*?)\)").unwrap());
static RE_DISABLED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@disabled\s*\((.*?)\)").unwrap());
static RE_READONLY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@readonly\s*\((.*?)\)").unwrap());
static RE_REQUIRED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@required\s*\((.*?)\)").unwrap());
static RE_CLASS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@class\s*\(\s*\[(.*?)\]\s*\)").unwrap());
static RE_STYLE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@style\s*\(\s*\[(.*?)\]\s*\)").unwrap());
static RE_JSON: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@json\s*\((.*?)\)").unwrap());
static RE_DUMP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@(?:dump|dd)\s*\((.*?)\)").unwrap());
static RE_IS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?:@is|@script)\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap());

static GLOBAL_DIRECTIVES: std::sync::OnceLock<std::sync::RwLock<std::collections::HashMap<String, RegisteredDirective>>> = std::sync::OnceLock::new();

/// Daftarkan custom directive ke Blade engine secara global.
/// Contoh: `@directiveName` atau `@directiveName(argumen)`
pub fn register_global_directive<F>(name: &str, handler: F)
where
    F: Fn(&str, &BladeEngine, &BladeContext) -> String + Send + Sync + 'static,
{
    let lock = GLOBAL_DIRECTIVES.get_or_init(|| std::sync::RwLock::new(std::collections::HashMap::new()));
    let pattern_args = format!(r#"@{}\s*\((.*?)\)"#, regex::escape(name));
    let pattern_no_args = format!(r#"@{}\b"#, regex::escape(name));
    let re_args = Regex::new(&pattern_args).expect("Valid regex for custom directive with args");
    let re_no_args = Regex::new(&pattern_no_args).expect("Valid regex for custom directive without args");

    let mut map = lock.write().unwrap();
    map.insert(
        name.to_string(),
        RegisteredDirective {
            name: name.to_string(),
            re_args,
            re_no_args,
            handler: std::sync::Arc::new(handler),
        },
    );
}

impl BladeEngine {
    /// Membuat BladeEngine kosong (tanpa membaca views path, cocok untuk test/memory engine)
    pub fn empty() -> Self {
        Self {
            templates: HashMap::new(),
            plugin_manager: Arc::new(crate::core::sandbox::LuminaPluginManager::new()),
            wasm_filters: HashMap::new(),
        }
    }

    pub fn new() -> Self {
        let mut engine = Self::empty();
        engine.reload_templates("resources/views");
        let _ = engine.plugin_manager.load_directory("plugins");
        let _ = engine.plugin_manager.load_directory("resources/plugins");
        engine
    }

    /// Pasang plugin_manager kustom/global (misal dari AppState)
    pub fn with_plugin_manager(mut self, manager: Arc<crate::core::sandbox::LuminaPluginManager>) -> Self {
        self.plugin_manager = manager;
        self
    }

    /// Daftarkan plugin WASM ke dalam sandbox engine
    pub fn register_plugin(&self, plugin: crate::core::sandbox::WasmPlugin) {
        self.plugin_manager.register(plugin);
    }

    /// Daftarkan alias filter Blade kustom yang mengarah ke plugin WASM tertentu
    pub fn register_wasm_filter(&mut self, alias: &str, plugin_name: &str, func_name: &str) {
        self.wasm_filters.insert(alias.to_string(), (plugin_name.to_string(), func_name.to_string()));
    }

    /// Daftarkan skrip Indonesian Script (.is) ke dalam runner engine
    pub fn register_is_script(&self, name: &str, runner: crate::core::sandbox::IsScriptRunner) {
        self.plugin_manager.register_is_script(name, runner);
    }



    pub fn reload_templates(&mut self, views_path: &str) {
        self.templates.clear();
        if Path::new(views_path).is_dir() {
            Self::collect_templates(views_path, "", &mut self.templates);
        }
    }

    fn collect_templates(base_path: &str, prefix: &str, out: &mut HashMap<String, String>) {
        let path = if prefix.is_empty() {
            base_path.to_string()
        } else {
            format!("{}/{}", base_path, prefix)
        };

        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                let file_name = entry.file_name().into_string().unwrap_or_default();

                if file_path.is_dir() {
                    let new_prefix = if prefix.is_empty() {
                        file_name
                    } else {
                        format!("{}/{}", prefix, file_name)
                    };
                    Self::collect_templates(base_path, &new_prefix, out);
                } else if file_name.ends_with(".blade.rs") || file_name.ends_with(".html") {
                    match fs::read_to_string(&file_path) {
                        Ok(content) => {
                            let template_name = if prefix.is_empty() {
                                file_name.clone()
                            } else {
                                format!("{}/{}", prefix, file_name)
                            };
                            let normalized = template_name.replace('\\', "/");
                            out.insert(normalized.clone(), content.clone());

                            if normalized.ends_with(".blade.rs") {
                                let without_ext = normalized.strip_suffix(".blade.rs").unwrap();
                                out.insert(without_ext.to_string(), content.clone());
                                out.insert(without_ext.replace('/', "."), content.clone());
                            } else if normalized.ends_with(".html") {
                                let without_ext = normalized.strip_suffix(".html").unwrap();
                                out.insert(without_ext.to_string(), content.clone());
                                out.insert(without_ext.replace('/', "."), content.clone());
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read template {:?}: {}", file_path, e);
                        }
                    }
                }
            }
        }
    }

    /// Render template menggunakan nama file atau format dot-notation (misal: "auth.login" atau "layouts/app.blade.rs")
    pub fn render(&self, template_name: &str, context: &BladeContext) -> String {
        let clean = template_name.replace('\\', "/");
        let raw = self.templates.get(&clean)
            .or_else(|| {
                if !clean.ends_with(".blade.rs") && !clean.ends_with(".html") {
                    let with_blade = format!("{}.blade.rs", clean.replace('.', "/"));
                    self.templates.get(&with_blade)
                } else {
                    None
                }
            })
            .or_else(|| {
                let dotted = clean.replace('/', ".");
                self.templates.get(&dotted)
            });

        let raw = match raw {
            Some(content) => content.clone(),
            None => {
                let keys = self.templates.keys().cloned().collect::<Vec<_>>();
                return format!("Template error: Template '{}' not found. Available templates: {:?}", template_name, keys);
            }
        };

        self.process_template(&raw, context, 0)
    }

    /// Render string template secara langsung tanpa membaca file
    pub fn render_string(&self, content: &str, context: &BladeContext) -> String {
        self.process_template(content, context, 0)
    }

    /// Pipeline utama pemrosesan template Blade
    fn process_template(&self, content: &str, context: &BladeContext, depth: usize) -> String {
        if depth > 25 {
            return "<!-- Template recursion limit exceeded -->".to_string();
        }

        let mut s = content.to_string();

        // 0. {{-- Comment --}}
        s = RE_COMMENT.replace_all(&s, "").to_string();

        // 1. Template Inheritance: @extends, @section, @yield, @hasSection
        s = self.process_inheritance(&s, context, depth);

        // 2. Includes: @include, @includeIf, @includeWhen, @includeUnless
        s = self.process_includes(&s, context, depth);

        // 3. Normalisasi direktif varian kondisional ke @if ... @endif standar
        s = self.normalize_conditionals(&s);

        // 4. Perulangan: @foreach, @forelse, @for
        s = self.process_loops(&s, context, depth);

        // 5. Kondisional bercabang: @if, @elseif, @else, @endif (mendukung nesting)
        s = self.process_if_blocks(&s, context, depth);

        // 6. Switch-Case: @switch, @case, @break, @default, @endswitch
        s = self.process_switch(&s, context, depth);

        // 7. Form & Security Helpers: @csrf, @method
        s = s.replace("@csrf", &format!("<input type=\"hidden\" name=\"csrf_token\" value=\"{}\">", context.get("csrf_token").and_then(|v| v.as_str()).unwrap_or("")));
        s = RE_METHOD.replace_all(&s, "<input type=\"hidden\" name=\"_method\" value=\"$1\">").to_string();

        // 7b. Form Error Directives: @error('field') ... @enderror
        s = self.process_errors(&s, context, depth);

        // 8. Attribute Helpers: @checked, @selected, @disabled, @readonly, @required, @class, @style
        s = self.process_attributes(&s, context);

        // 9. Debug & Output Helpers: @json, @dump, @dd
        s = self.process_debug_helpers(&s, context);

        // 10. Indonesian Script: @is('script') / @script('script')
        s = self.process_indonesian_script(&s);

        // 11. Direktif WASM: @wasm('plugin_name', 'function_name', $arg)
        s = RE_WASM.replace_all(&s, |caps: &regex::Captures| {
            let plugin_name = &caps[1];
            let func_name = &caps[2];
            let arg = caps.get(3).map(|m| context.eval_val(m.as_str())).unwrap_or_default();
            match self.plugin_manager.call_filter(plugin_name, func_name, &arg) {
                Ok(out) => out,
                Err(e) => format!("<!-- WASM error: {} -->", e),
            }
        }).to_string();

        // 12. Direktif Evaluasi Backend Indonesian Script: @eval_is('file.is', 'func', $arg1, $arg2, ...)
        s = RE_EVAL_IS.replace_all(&s, |caps: &regex::Captures| {
            let script_path = &caps[1];
            let func_name = &caps[2];
            let mut args_vec = Vec::new();
            if let Some(args_raw) = caps.get(3) {
                for arg in args_raw.as_str().split(',') {
                    let clean_arg = arg.trim();
                    if !clean_arg.is_empty() {
                        let evaled = context.eval_val(clean_arg);
                        if let Ok(num) = evaled.parse::<f64>() {
                            args_vec.push(json!(num));
                        } else if evaled == "true" {
                            args_vec.push(json!(true));
                        } else if evaled == "false" {
                            args_vec.push(json!(false));
                        } else {
                            args_vec.push(json!(evaled));
                        }
                    }
                }
            }

            match self.plugin_manager.eval_is_script(script_path, func_name, &args_vec) {
                Ok(val) => match val {
                    Value::String(s) => s,
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            i.to_string()
                        } else if let Some(f) = n.as_f64() {
                            if f.fract() == 0.0 {
                                (f as i64).to_string()
                            } else {
                                f.to_string()
                            }
                        } else {
                            n.to_string()
                        }
                    },
                    Value::Bool(b) => b.to_string(),
                    Value::Null => String::new(),
                    other => other.to_string(),
                },
                Err(e) => format!("<!-- Indonesian Script error: {} -->", e),
            }
        }).to_string();

        // 13. Ekstensi Custom Directives (didukung oleh lumina-live atau plugin pihak ketiga - zero regex compilation)
        if s.contains('@') {
            if let Some(lock) = GLOBAL_DIRECTIVES.get() {
                let directives = lock.read().unwrap();
                for dir in directives.values() {
                    if !s.contains(&dir.name) {
                        continue;
                    }
                    s = dir.re_args.replace_all(&s, |caps: &regex::Captures| {
                        (dir.handler)(&caps[1], self, context)
                    }).to_string();

                    s = dir.re_no_args.replace_all(&s, |_caps: &regex::Captures| {
                        (dir.handler)("", self, context)
                    }).to_string();
                }
            }
        }
        // 15. Interpolasi Variabel & Filter: {!! raw !!} dan {{ escaped }}

        s = RE_RAW.replace_all(&s, |caps: &regex::Captures| {
            self.eval_expr_with_filters(&caps[1], context)
        }).to_string();

        s = RE_VAR.replace_all(&s, |caps: &regex::Captures| {
            html_escape::encode_safe(&self.eval_expr_with_filters(&caps[1], context))
        }).to_string();

        s
    }

    /// Evaluasi ekspresi variabel atau literal yang mungkin memiliki filter berantai:
    /// Contoh: `$nominal | rupiah`
    /// Contoh: `$title | slug | uppercase`
    /// Contoh: `$content | limit: 100`
    /// Contoh: `$comment | sensor_kata` (WASM Sandbox)
    pub fn eval_expr_with_filters(&self, expr: &str, context: &BladeContext) -> String {
        let expr = expr.trim();
        if !expr.contains('|') {
            return context.eval_val(expr);
        }

        // Simpan operator '||' jika ada agar tidak terpotong pipa filter
        let safe_expr = expr.replace("||", "__LOGICAL_OR__");
        let parts: Vec<&str> = safe_expr.split('|').collect();

        let base_raw = parts[0].replace("__LOGICAL_OR__", "||");
        let mut current_val = context.eval_val(&base_raw);

        for segment in &parts[1..] {
            let seg = segment.trim();
            if seg.is_empty() {
                continue;
            }

            // Parsing filter name dan parameter opsional: filter_name: arg1, arg2
            let (filter_name, args) = if let Some(colon_idx) = seg.find(':') {
                let name = seg[..colon_idx].trim();
                let raw_args = &seg[colon_idx + 1..];
                let args_vec: Vec<&str> = raw_args.split(',').map(|a| a.trim()).collect();
                (name, args_vec)
            } else {
                (seg, Vec::new())
            };

            current_val = self.apply_filter(filter_name, &args, &current_val);
        }

        current_val
    }

    /// Terapkan filter pada nilai string (Native Rust Filters + WASM Sandbox Fallback)
    fn apply_filter(&self, filter_name: &str, args: &[&str], input: &str) -> String {
        match filter_name {
            // Native Filter: Format Mata Uang Rupiah
            "rupiah" | "formatRupiah" => Self::format_rupiah(input),

            // Native Filter: URL Slug Generator
            "slug" => Self::slug(input),

            // Native Filter: Huruf Besar (Uppercase)
            "upper" | "uppercase" => input.to_uppercase(),

            // Native Filter: Huruf Kecil (Lowercase)
            "lower" | "lowercase" => input.to_lowercase(),

            // Native Filter: Kapitalisasi Kata
            "capitalize" | "title" => {
                let mut c = input.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            }

            // Native Filter: Pangkas Spasi
            "trim" => input.trim().to_string(),

            // Native Filter: Pemotongan Teks (limit: 50 atau potong: 50)
            "limit" | "potong" => {
                let max_len = args.first().and_then(|a| a.parse::<usize>().ok()).unwrap_or(100);
                if input.chars().count() > max_len {
                    let truncated: String = input.chars().take(max_len).collect();
                    format!("{}...", truncated)
                } else {
                    input.to_string()
                }
            }

            // Native Filter: Balik Karakter
            "reverse" | "balik" => input.chars().rev().collect(),

            // WASM Sandbox Fallback: Delegasikan ke WebAssembly Plugin
            custom_filter => {
                // 1. Cek pemetaan alias filter terdaftar
                if let Some((plugin_name, func_name)) = self.wasm_filters.get(custom_filter) {
                    if let Ok(res) = self.plugin_manager.call_filter(plugin_name, func_name, input) {
                        return res;
                    }
                }

                // 2. Cek apakah ada modul plugin dengan nama filter itu sendiri
                if let Ok(res) = self.plugin_manager.call_filter(custom_filter, "transform", input) {
                    return res;
                }
                if let Ok(res) = self.plugin_manager.call_filter(custom_filter, custom_filter, input) {
                    return res;
                }

                // Jika filter tidak ditemukan, kembalikan teks asli dengan aman tanpa panic
                input.to_string()
            }
        }
    }

    /// Helper Native: Format angka ke format mata uang Rupiah
    pub fn format_rupiah(input: &str) -> String {
        let clean = input.trim().replace(',', "");
        if let Ok(num) = clean.parse::<f64>() {
            let int_part = num.trunc() as i64;
            let s = int_part.abs().to_string();
            let mut formatted = String::new();
            let mut count = 0;
            for c in s.chars().rev() {
                if count > 0 && count % 3 == 0 {
                    formatted.push('.');
                }
                formatted.push(c);
                count += 1;
            }
            let reversed: String = formatted.chars().rev().collect();
            if int_part < 0 {
                format!("-Rp {}", reversed)
            } else {
                format!("Rp {}", reversed)
            }
        } else {
            format!("Rp {}", input)
        }
    }

    /// Helper Native: Mengubah teks menjadi slug URL (contoh: "Halo Dunia!" -> "halo-dunia")
    pub fn slug(input: &str) -> String {
        let mut result = String::new();
        let mut last_was_dash = false;
        for c in input.chars() {
            if c.is_alphanumeric() {
                result.push(c.to_ascii_lowercase());
                last_was_dash = false;
            } else if !last_was_dash && !result.is_empty() {
                result.push('-');
                last_was_dash = true;
            }
        }
        if result.ends_with('-') {
            result.pop();
        }
        result
    }


    /// Normalisasi sintaks kondisional Laravel menjadi format `@if` standar
    fn normalize_conditionals(&self, input: &str) -> String {
        let mut s = input.to_string();

        // @unless(expr) -> @if(!(expr)) & @endunless -> @endif
        s = RE_UNLESS.replace_all(&s, "@if(!($1))").to_string();
        s = s.replace("@endunless", "@endif");

        // @isset(var) -> @if(isset(var)) & @endisset -> @endif
        s = RE_ISSET.replace_all(&s, "@if(isset($1))").to_string();
        s = s.replace("@endisset", "@endif");

        // @empty(var) -> @if(empty(var)) & @endempty -> @endif
        s = RE_EMPTY.replace_all(&s, "@if(empty($1))").to_string();
        s = s.replace("@endempty", "@endif");

        // @auth -> @if(auth) & @endauth -> @endif
        s = RE_AUTH.replace_all(&s, "@if(auth)").to_string();
        s = s.replace("@endauth", "@endif");

        // @guest -> @if(guest) & @endguest -> @endif
        s = RE_GUEST.replace_all(&s, "@if(guest)").to_string();
        s = s.replace("@endguest", "@endif");

        // @can('permission') -> @if(can('permission')) & @endcan -> @endif
        s = RE_CAN.replace_all(&s, "@if(can($1))").to_string();
        s = s.replace("@endcan", "@endif");

        // @cannot('permission') -> @if(cannot($1)) & @endcannot -> @endif
        s = RE_CANNOT.replace_all(&s, "@if(cannot($1))").to_string();
        s = s.replace("@endcannot", "@endif");

        // @role('role') -> @if(role($1)) & @endrole -> @endif
        s = RE_ROLE.replace_all(&s, "@if(role($1))").to_string();
        s = s.replace("@endrole", "@endif");

        // @production -> @if(production) & @endproduction -> @endif
        s = RE_PROD.replace_all(&s, "@if(production)").to_string();
        s = s.replace("@endproduction", "@endif");

        // @env(...) -> @if(env(...)) & @endenv -> @endif
        s = RE_ENV.replace_all(&s, "@if(env($1))").to_string();
        s = s.replace("@endenv", "@endif");

        s
    }

    /// Penanganan pewarisan template (@extends, @section, @yield)
    fn process_inheritance(&self, content: &str, context: &BladeContext, depth: usize) -> String {
        let mut s = content.to_string();

        
        if let Some(caps) = RE_EXTENDS.captures(&s) {
            let layout_raw = caps[1].trim().replace('\\', "/");
            let layout_content = self.templates.get(&layout_raw)
                .or_else(|| {
                    let normalized = layout_raw.replace('.', "/");
                    let file = if normalized.ends_with(".blade.rs") || normalized.ends_with(".html") {
                        normalized
                    } else {
                        format!("{}.blade.rs", normalized)
                    };
                    self.templates.get(&file)
                });

            let mut sections: HashMap<String, String> = HashMap::new();

            // Short inline syntax: @section('title', 'Page Title')
            
            for sec_caps in RE_SHORT_SECTION.captures_iter(&s) {
                sections.insert(sec_caps[1].to_string(), sec_caps[2].to_string());
            }

            // Block syntax: @section('name') ... @endsection / @stop / @show
            
            for sec_caps in RE_SECTION.captures_iter(&s) {
                let sec_name = sec_caps[1].to_string();
                let sec_content = sec_caps[2].to_string();
                sections.insert(sec_name, sec_content);
            }

            // Load parent layout
            if let Some(parent_content) = layout_content {
                let mut layout_processed = parent_content.clone();

                // 1. @yield('name', 'default fallback')
                layout_processed = RE_YIELD_DEFAULT.replace_all(&layout_processed, |y_caps: &regex::Captures| {
                    let y_name = &y_caps[1];
                    let y_default = &y_caps[2];
                    sections.get(y_name).cloned().unwrap_or_else(|| y_default.to_string())
                }).to_string();

                // 2. @yield('name')
                layout_processed = RE_YIELD.replace_all(&layout_processed, |y_caps: &regex::Captures| {
                    let y_name = &y_caps[1];
                    sections.get(y_name).cloned().unwrap_or_default()
                }).to_string();

                // 3. @hasSection('name')
                layout_processed = RE_HAS_SEC.replace_all(&layout_processed, |h_caps: &regex::Captures| {
                    if sections.contains_key(&h_caps[1]) {
                        "@if(true)".to_string()
                    } else {
                        "@if(false)".to_string()
                    }
                }).to_string();

                s = self.process_template(&layout_processed, context, depth + 1);
            }
        }

        s
    }

    /// Penanganan @include, @includeIf, @includeWhen, @includeUnless
    fn process_includes(&self, content: &str, context: &BladeContext, depth: usize) -> String {
        let mut s = content.to_string();

        // @includeWhen(cond, 'path')
        s = RE_INC_WHEN.replace_all(&s, |caps: &regex::Captures| {
            let cond = &caps[1];
            let path = &caps[2];
            if context.eval_truthy(cond) {
                self.resolve_include(path, context, depth, false)
            } else {
                String::new()
            }
        }).to_string();

        // @includeUnless(cond, 'path')
        s = RE_INC_UNLESS.replace_all(&s, |caps: &regex::Captures| {
            let cond = &caps[1];
            let path = &caps[2];
            if !context.eval_truthy(cond) {
                self.resolve_include(path, context, depth, false)
            } else {
                String::new()
            }
        }).to_string();

        // @includeIf('path')
        s = RE_INC_IF.replace_all(&s, |caps: &regex::Captures| {
            self.resolve_include(&caps[1], context, depth, true)
        }).to_string();

        // @include('path')
        s = RE_INC.replace_all(&s, |caps: &regex::Captures| {
            self.resolve_include(&caps[1], context, depth, false)
        }).to_string();

        s
    }

    fn resolve_include(&self, path_raw: &str, context: &BladeContext, depth: usize, silent: bool) -> String {
        let clean = path_raw.trim().replace('\\', "/");
        let inc_content = self.templates.get(&clean)
            .or_else(|| {
                let normalized = clean.replace('.', "/");
                let file = if normalized.ends_with(".blade.rs") || normalized.ends_with(".html") {
                    normalized
                } else {
                    format!("{}.blade.rs", normalized)
                };
                self.templates.get(&file)
            });

        if let Some(content) = inc_content {
            self.process_template(content, context, depth + 1)
        } else if silent {
            String::new()
        } else {
            format!("<!-- Include '{}' not found -->", path_raw)
        }
    }

    /// Penanganan perulangan (@foreach, @forelse, @for)
    fn process_loops(&self, content: &str, context: &BladeContext, depth: usize) -> String {
        let mut s = content.to_string();

        // @foreach ($collection as $item) / @foreach ($collection as $key => $item)
        
        while let Some(m) = RE_FOREACH_OPEN.find(&s) {
            let start = m.start();
            let header = RE_FOREACH_OPEN.captures(&s[start..]).unwrap()[1].to_string();
            let body_start = m.end();

            // Cari matching @endforeach
            let mut nesting = 1;
            let mut idx = body_start;
            let mut block_end = None;

            while idx < s.len() {
                let slice = &s[idx..];
                if slice.starts_with("@foreach") {
                    nesting += 1;
                    idx += 8;
                } else if slice.starts_with("@endforeach") {
                    nesting -= 1;
                    if nesting == 0 {
                        block_end = Some(idx);
                        break;
                    }
                    idx += 11;
                } else {
                    idx += 1;
                }
            }

            if let Some(end) = block_end {
                let body = &s[body_start..end];
                let rendered = self.render_foreach_loop(&header, body, "", context, depth);
                s.replace_range(start..end + "@endforeach".len(), &rendered);
            } else {
                break;
            }
        }

        // @forelse ($collection as $item) ... @empty ... @endforelse
        
        while let Some(m) = RE_FORELSE_OPEN.find(&s) {
            let start = m.start();
            let header = RE_FORELSE_OPEN.captures(&s[start..]).unwrap()[1].to_string();
            let body_start = m.end();

            let mut nesting = 1;
            let mut idx = body_start;
            let mut empty_pos = None;
            let mut block_end = None;

            while idx < s.len() {
                let slice = &s[idx..];
                if slice.starts_with("@forelse") {
                    nesting += 1;
                    idx += 8;
                } else if slice.starts_with("@empty") && nesting == 1 && (slice[6..].starts_with(|c: char| c.is_whitespace()) || slice.len() == 6) {
                    empty_pos = Some(idx);
                    idx += 6;
                } else if slice.starts_with("@endforelse") {
                    nesting -= 1;
                    if nesting == 0 {
                        block_end = Some(idx);
                        break;
                    }
                    idx += 11;
                } else {
                    idx += 1;
                }
            }

            if let Some(end) = block_end {
                let (body, empty_body) = if let Some(e_idx) = empty_pos {
                    (&s[body_start..e_idx], &s[e_idx + 6..end])
                } else {
                    (&s[body_start..end], "")
                };

                let rendered = self.render_foreach_loop(&header, body, empty_body, context, depth);
                s.replace_range(start..end + "@endforelse".len(), &rendered);
            } else {
                break;
            }
        }

        // @for ($i = 0; $i < N; $i++) atau @for ($i in 0..N)
        
        while let Some(m) = RE_FOR_OPEN.find(&s) {
            let start = m.start();
            let header = RE_FOR_OPEN.captures(&s[start..]).unwrap()[1].to_string();
            let body_start = m.end();

            let mut nesting = 1;
            let mut idx = body_start;
            let mut block_end = None;

            while idx < s.len() {
                let slice = &s[idx..];
                if slice.starts_with("@for") && !slice.starts_with("@foreach") && !slice.starts_with("@forelse") {
                    nesting += 1;
                    idx += 4;
                } else if slice.starts_with("@endfor") && !slice.starts_with("@endforeach") && !slice.starts_with("@endforelse") {
                    nesting -= 1;
                    if nesting == 0 {
                        block_end = Some(idx);
                        break;
                    }
                    idx += 7;
                } else {
                    idx += 1;
                }
            }

            if let Some(end) = block_end {
                let body = &s[body_start..end];
                let rendered = self.render_for_loop(&header, body, context, depth);
                s.replace_range(start..end + "@endfor".len(), &rendered);
            } else {
                break;
            }
        }

        s
    }

    fn render_foreach_loop(&self, header: &str, body: &str, empty_body: &str, context: &BladeContext, depth: usize) -> String {
        // Syntax: $collection as $item ATAU $collection as $key => $item
        let parts: Vec<&str> = header.split(" as ").collect();
        if parts.len() != 2 {
            return String::new();
        }

        let coll_expr = parts[0].trim();
        let target = parts[1].trim();

        let (key_var, item_var) = if let Some(arrow_idx) = target.find("=>") {
            (
                Some(target[..arrow_idx].trim().trim_start_matches('$').to_string()),
                target[arrow_idx + 2..].trim().trim_start_matches('$').to_string(),
            )
        } else {
            (None, target.trim_start_matches('$').to_string())
        };

        let collection_val = context.get(coll_expr);

        match collection_val {
            Some(Value::Array(items)) if !items.is_empty() => {
                let total = items.len();
                let mut out = String::new();
                for (idx, item) in items.iter().enumerate() {
                    let mut iter_ctx = context.clone();
                    iter_ctx.insert_val(&item_var, item.clone());
                    if let Some(ref k) = key_var {
                        iter_ctx.insert_val(k, json!(idx));
                    }

                    let loop_obj = json!({
                        "index": idx,
                        "iteration": idx + 1,
                        "remaining": total - (idx + 1),
                        "count": total,
                        "first": idx == 0,
                        "last": idx == total - 1,
                        "even": (idx + 1) % 2 == 0,
                        "odd": (idx + 1) % 2 != 0,
                        "depth": depth + 1,
                    });
                    iter_ctx.insert_val("loop", loop_obj);
                    out.push_str(&self.process_template(body, &iter_ctx, depth + 1));
                }
                out
            }
            Some(Value::Object(map)) if !map.is_empty() => {
                let total = map.len();
                let mut out = String::new();
                for (idx, (k, v)) in map.iter().enumerate() {
                    let mut iter_ctx = context.clone();
                    iter_ctx.insert_val(&item_var, v.clone());
                    if let Some(ref kv) = key_var {
                        iter_ctx.insert_val(kv, json!(k));
                    }

                    let loop_obj = json!({
                        "index": idx,
                        "iteration": idx + 1,
                        "remaining": total - (idx + 1),
                        "count": total,
                        "first": idx == 0,
                        "last": idx == total - 1,
                        "even": (idx + 1) % 2 == 0,
                        "odd": (idx + 1) % 2 != 0,
                        "depth": depth + 1,
                    });
                    iter_ctx.insert_val("loop", loop_obj);
                    out.push_str(&self.process_template(body, &iter_ctx, depth + 1));
                }
                out
            }
            _ => {
                if !empty_body.is_empty() {
                    self.process_template(empty_body, context, depth + 1)
                } else {
                    String::new()
                }
            }
        }
    }

    fn render_for_loop(&self, header: &str, body: &str, context: &BladeContext, depth: usize) -> String {
        // Support: $i in 0..5
        if header.contains(" in ") {
            let parts: Vec<&str> = header.split(" in ").collect();
            if parts.len() == 2 {
                let var = parts[0].trim().trim_start_matches('$');
                let range_parts: Vec<&str> = parts[1].split("..").collect();
                if range_parts.len() == 2 {
                    let start: i64 = range_parts[0].trim().parse().unwrap_or(0);
                    let end: i64 = range_parts[1].trim().parse().unwrap_or(0);
                    let mut out = String::new();
                    for i in start..end {
                        let mut iter_ctx = context.clone();
                        iter_ctx.insert_val(var, json!(i));
                        out.push_str(&self.process_template(body, &iter_ctx, depth + 1));
                    }
                    return out;
                }
            }
        }

        // Support standard C/PHP style: $i = 0; $i < 5; $i++
        let parts: Vec<&str> = header.split(';').collect();
        if parts.len() == 3 {
            
            
            if let (Some(init_caps), Some(cond_caps)) = (RE_INIT.captures(parts[0]), RE_COND.captures(parts[1])) {
                let var_name = &init_caps[1];
                let start: i64 = init_caps[2].parse().unwrap_or(0);
                let end: i64 = cond_caps[1].parse().unwrap_or(0);

                let mut out = String::new();
                for i in start..end {
                    let mut iter_ctx = context.clone();
                    iter_ctx.insert_val(var_name, json!(i));
                    out.push_str(&self.process_template(body, &iter_ctx, depth + 1));
                }
                return out;
            }
        }

        String::new()
    }

    fn extract_balanced_parens<'a>(s: &'a str, open_idx: usize) -> Option<(&'a str, usize)> {
        let mut depth = 0;
        let bytes = s.as_bytes();
        for i in open_idx..s.len() {
            if bytes[i] == b'(' {
                depth += 1;
            } else if bytes[i] == b')' {
                depth -= 1;
                if depth == 0 {
                    return Some((&s[open_idx + 1..i], i + 1));
                }
            }
        }
        None
    }

    /// Penanganan percabangan @if ... @elseif ... @else ... @endif (mendukung nesting)
    fn process_if_blocks(&self, input: &str, context: &BladeContext, depth: usize) -> String {
        let mut result = input.to_string();

        while let Some(if_idx) = result.find("@if") {
            let after_if = &result[if_idx + 3..];
            let paren_rel = after_if.find('(');
            if paren_rel.is_none() || !after_if[..paren_rel.unwrap()].chars().all(|c| c.is_whitespace()) {
                break;
            }
            let open_paren = if_idx + 3 + paren_rel.unwrap();
            let (cond, after_if_end) = match Self::extract_balanced_parens(&result, open_paren) {
                Some(res) => (res.0.trim().to_string(), res.1),
                None => break,
            };

            let start_pos = if_idx;
            let mut branches: Vec<(Option<String>, usize, usize)> = Vec::new();
            let mut curr_cond = Some(cond);
            let mut curr_branch_start = after_if_end;
            let mut nesting = 1;
            let mut block_end = None;

            let mut idx = after_if_end;

            while idx < result.len() {
                if result.as_bytes()[idx] == b'@' {
                    let slice = &result[idx..];
                    if slice.starts_with("@if") && slice[3..].starts_with(|c: char| c.is_whitespace() || c == '(') {
                        nesting += 1;
                        idx += 3;
                        continue;
                    } else if slice.starts_with("@endif") {
                        nesting -= 1;
                        if nesting == 0 {
                            branches.push((curr_cond, curr_branch_start, idx));
                            block_end = Some(idx + "@endif".len());
                            break;
                        }
                        idx += 6;
                        continue;
                    } else if nesting == 1 {
                        if slice.starts_with("@elseif") {
                            let after_elseif = &slice[7..];
                            if let Some(p) = after_elseif.find('(') {
                                if after_elseif[..p].chars().all(|c| c.is_whitespace()) {
                                    if let Some((elseif_cond, after_paren)) = Self::extract_balanced_parens(slice, 7 + p) {
                                        branches.push((curr_cond, curr_branch_start, idx));
                                        curr_cond = Some(elseif_cond.trim().to_string());
                                        idx += after_paren;
                                        curr_branch_start = idx;
                                        continue;
                                    }
                                }
                            }
                        } else if slice.starts_with("@else") && (slice[5..].starts_with(|c: char| c.is_whitespace()) || slice[5..].starts_with('<') || slice.len() == 5) {
                            branches.push((curr_cond, curr_branch_start, idx));
                            curr_cond = None;
                            idx += 5;
                            curr_branch_start = idx;
                            continue;
                        }
                    }
                }
                idx += 1;
            }

            if let Some(end_pos) = block_end {
                let mut chosen_content = String::new();
                for (branch_cond, b_start, b_end) in branches {
                    let is_match = match branch_cond {
                        Some(ref c) => context.eval_truthy(c),
                        None => true,
                    };
                    if is_match {
                        chosen_content = result[b_start..b_end].to_string();
                        break;
                    }
                }

                let rendered_branch = self.process_template(&chosen_content, context, depth + 1);
                result.replace_range(start_pos..end_pos, &rendered_branch);
            } else {
                break;
            }
        }

        result
    }

    /// Penanganan @switch ... @case ... @default ... @endswitch
    fn process_switch(&self, content: &str, context: &BladeContext, depth: usize) -> String {
        let mut s = content.to_string();
        s = RE_SWITCH.replace_all(&s, |caps: &regex::Captures| {
            let target_val = context.eval_val(&caps[1]);
            let body = &caps[2];

            
            let mut matched = false;
            let mut output = String::new();

            for case_caps in RE_CASES.captures_iter(body) {
                let case_val = case_caps[1].trim();
                let case_body = case_caps[2].trim();
                if case_val == target_val {
                    matched = true;
                    output = self.process_template(case_body, context, depth + 1);
                    break;
                }
            }

            if !matched {
                
                if let Some(def_caps) = RE_DEFAULT.captures(body) {
                    output = self.process_template(def_caps[1].trim(), context, depth + 1);
                }
            }

            output
        }).to_string();

        s
    }

    /// Penanganan direktif error validasi form: @error('field') ... @enderror
    fn process_errors(&self, content: &str, context: &BladeContext, depth: usize) -> String {
        RE_ERROR.replace_all(content, |caps: &regex::Captures| {
            let field = &caps[1];
            let inner = &caps[2];

            if let Some(msg) = context.get_error(field) {
                let mut sub_ctx = context.clone();
                sub_ctx.insert("message", &msg);
                sub_ctx.insert("$message", &msg);
                let replaced = inner
                    .replace("{{ $message }}", &html_escape::encode_safe(&msg))
                    .replace("{{$message}}", &html_escape::encode_safe(&msg))
                    .replace("{{ $message}}", &html_escape::encode_safe(&msg))
                    .replace("{{$message }}", &html_escape::encode_safe(&msg))
                    .replace("{{ message }}", &html_escape::encode_safe(&msg))
                    .replace("{{message}}", &html_escape::encode_safe(&msg))
                    .replace("{!! $message !!}", &msg)
                    .replace("{!!$message!!}", &msg);
                self.process_template(&replaced, &sub_ctx, depth + 1)
            } else {
                String::new()
            }
        }).to_string()
    }

    /// Penanganan direktif atribut HTML: @checked, @selected, @disabled, @readonly, @required, @class, @style
    fn process_attributes(&self, content: &str, context: &BladeContext) -> String {
        let mut s = content.to_string();

        // @checked(cond)
        s = RE_CHECKED.replace_all(&s, |caps: &regex::Captures| {
            if context.eval_truthy(&caps[1]) { "checked" } else { "" }
        }).to_string();

        // @selected(cond)
        s = RE_SELECTED.replace_all(&s, |caps: &regex::Captures| {
            if context.eval_truthy(&caps[1]) { "selected" } else { "" }
        }).to_string();

        // @disabled(cond)
        s = RE_DISABLED.replace_all(&s, |caps: &regex::Captures| {
            if context.eval_truthy(&caps[1]) { "disabled" } else { "" }
        }).to_string();

        // @readonly(cond)
        s = RE_READONLY.replace_all(&s, |caps: &regex::Captures| {
            if context.eval_truthy(&caps[1]) { "readonly" } else { "" }
        }).to_string();

        // @required(cond)
        s = RE_REQUIRED.replace_all(&s, |caps: &regex::Captures| {
            if context.eval_truthy(&caps[1]) { "required" } else { "" }
        }).to_string();

        // @class(['p-4', 'font-bold' => $active, 'hidden' => !$show])
        s = RE_CLASS.replace_all(&s, |caps: &regex::Captures| {
            let inner = &caps[1];
            let mut active_classes: Vec<String> = Vec::new();
            for item in inner.split(',') {
                let item = item.trim();
                if item.is_empty() {
                    continue;
                }
                if let Some(arrow_idx) = item.find("=>") {
                    let class_name = item[..arrow_idx].trim().trim_matches('\'').trim_matches('"');
                    let cond = item[arrow_idx + 2..].trim();
                    if context.eval_truthy(cond) {
                        active_classes.push(class_name.to_string());
                    }
                } else {
                    let class_name = item.trim_matches('\'').trim_matches('"');
                    if !class_name.is_empty() {
                        active_classes.push(class_name.to_string());
                    }
                }
            }
            if !active_classes.is_empty() {
                format!("class=\"{}\"", active_classes.join(" "))
            } else {
                String::new()
            }
        }).to_string();

        // @style(['background-color: red' => $hasError])
        s = RE_STYLE.replace_all(&s, |caps: &regex::Captures| {
            let inner = &caps[1];
            let mut active_styles: Vec<String> = Vec::new();
            for item in inner.split(',') {
                let item = item.trim();
                if item.is_empty() {
                    continue;
                }
                if let Some(arrow_idx) = item.find("=>") {
                    let style_rule = item[..arrow_idx].trim().trim_matches('\'').trim_matches('"');
                    let cond = item[arrow_idx + 2..].trim();
                    if context.eval_truthy(cond) {
                        active_styles.push(style_rule.to_string());
                    }
                } else {
                    let style_rule = item.trim_matches('\'').trim_matches('"');
                    if !style_rule.is_empty() {
                        active_styles.push(style_rule.to_string());
                    }
                }
            }
            if !active_styles.is_empty() {
                format!("style=\"{}\"", active_styles.join("; "))
            } else {
                String::new()
            }
        }).to_string();

        s
    }

    /// Penanganan @json, @dump, @dd
    fn process_debug_helpers(&self, content: &str, context: &BladeContext) -> String {
        let mut s = content.to_string();

        // @json($variable)
        s = RE_JSON.replace_all(&s, |caps: &regex::Captures| {
            let var = caps[1].trim();
            if let Some(val) = context.get(var) {
                serde_json::to_string(val).unwrap_or_else(|_| "null".to_string())
            } else {
                "null".to_string()
            }
        }).to_string();

        // @dump($variable) / @dd($variable)
        s = RE_DUMP.replace_all(&s, |caps: &regex::Captures| {
            let var = caps[1].trim();
            let json_str = if let Some(val) = context.get(var) {
                serde_json::to_string_pretty(val).unwrap_or_default()
            } else {
                format!("Variable '{}' not found", var)
            };
            format!(
                "<pre style=\"background:#181825;color:#a6e3a1;padding:12px;border-radius:8px;font-family:monospace;font-size:12px;overflow-x:auto;\">{}</pre>",
                html_escape::encode_safe(&json_str)
            )
        }).to_string();

        s
    }

    /// Penanganan direktif Indonesian Script (@is, @script)
    fn process_indonesian_script(&self, content: &str) -> String {
        RE_IS.replace_all(content, |caps: &regex::Captures| {
            let script_name = caps[1].trim();
            let mut file_path = format!("resources/scripts/{}", script_name);
            if !file_path.ends_with(".is") {
                file_path.push_str(".is");
            }

            let source = if let Ok(content) = fs::read_to_string(&file_path) {
                content
            } else {
                let alt_path = format!("resources/js/{}", script_name);
                let alt_path_is = if alt_path.ends_with(".is") { alt_path.clone() } else { format!("{}.is", alt_path) };
                fs::read_to_string(&alt_path_is).unwrap_or_else(|_| format!("// File script '.is' tidak ditemukan: {}", file_path))
            };

            let compiled_js = indonesian_script::compile_is(&source);
            format!("<script>\n{}\n</script>", compiled_js)
        }).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blade_extends_and_yield() {
        let mut engine = BladeEngine::empty();
        engine.templates.insert("layouts/app.blade.rs".to_string(), "<html><title>@yield('title', 'Default Title')</title><body>@yield('content')</body></html>".to_string());
        engine.templates.insert("home.blade.rs".to_string(), "@extends('layouts.app')\n@section('title', 'Halaman Utama')\n@section('content')<h1>Halo {{ $name }}</h1>@endsection".to_string());

        let mut ctx = BladeContext::new();
        ctx.insert("name", "Lumina Pure Blade");

        let html = engine.render("home", &ctx);
        assert!(html.contains("<title>Halaman Utama</title>"));
        assert!(html.contains("<h1>Halo Lumina Pure Blade</h1>"));
    }

    #[test]
    fn test_blade_conditionals_nested() {
        let engine = BladeEngine::empty();

        let tmpl = r#"
        @if ($isAdmin)
            <h1>Halo Admin</h1>
            @if ($role == 'superadmin')
                <span>Full Access</span>
            @else
                <span>Normal Admin</span>
            @endif
        @else
            <h1>Halo Tamu</h1>
        @endif
        "#;

        let mut ctx = BladeContext::new();
        ctx.insert("isAdmin", true);
        ctx.insert("role", "superadmin");

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("Halo Admin"));
        assert!(html.contains("Full Access"));
        assert!(!html.contains("Halo Tamu"));
    }

    #[test]
    fn test_blade_foreach_and_loop_variable() {
        let engine = BladeEngine::empty();

        let tmpl = r#"
        <ul>
        @foreach ($users as $user)
            <li>{{ $loop.iteration }}. {{ $user.name }} (first: {{ $loop.first }})</li>
        @endforeach
        </ul>
        "#;

        let mut ctx = BladeContext::new();
        ctx.insert("users", json!([
            {"name": "Alice"},
            {"name": "Bob"}
        ]));

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("1. Alice (first: true)"));
        assert!(html.contains("2. Bob (first: false)"));
    }

    #[test]
    fn test_blade_forelse_empty() {
        let engine = BladeEngine::empty();

        let tmpl = r#"
        @forelse ($items as $item)
            <p>{{ $item }}</p>
        @empty
            <p class="empty">Tidak ada item</p>
        @endforelse
        "#;

        let mut ctx = BladeContext::new();
        ctx.insert("items", json!([]));

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("Tidak ada item"));
    }

    #[test]
    fn test_blade_auth_guest() {
        let engine = BladeEngine::empty();

        let tmpl = r#"
        @auth
            <p>Welcome back, {{ $email }}</p>
        @else
            <a href="/login">Login</a>
        @endauth
        "#;

        let mut ctx = BladeContext::new();
        ctx.insert("user_id", 1);
        ctx.insert("email", "admin@lumina.rs");

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("Welcome back, admin@lumina.rs"));
        assert!(!html.contains("Login"));
    }

    #[test]
    fn test_blade_attribute_helpers() {
        let engine = BladeEngine::empty();

        let tmpl = r#"<input type="checkbox" @checked($isActive)> <button @class(['btn', 'btn-primary' => $isPrimary, 'hidden' => $isHidden])>Kirim</button>"#;

        let mut ctx = BladeContext::new();
        ctx.insert("isActive", true);
        ctx.insert("isPrimary", true);
        ctx.insert("isHidden", false);

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("checked"));
        assert!(html.contains("class=\"btn btn-primary\""));
    }

    #[test]
    fn test_blade_null_coalescing() {
        let engine = BladeEngine::empty();

        let tmpl = r#"<p>{{ $username ?? 'Pengguna Anonim' }}</p>"#;
        let ctx = BladeContext::new();

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("Pengguna Anonim"));
    }

    #[test]
    fn test_blade_switch_case() {
        let engine = BladeEngine::empty();

        let tmpl = r#"
        @switch ($status)
            @case ('pending')
                <span>Menunggu Konfirmasi</span>
                @break
            @case ('active')
                <span>Akun Aktif</span>
                @break
            @default
                <span>Status Tidak Dikenal</span>
        @endswitch
        "#;

        let mut ctx = BladeContext::new();
        ctx.insert("status", "active");

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("Akun Aktif"));
        assert!(!html.contains("Menunggu Konfirmasi"));
    }

    #[test]
    fn test_blade_for_loop() {
        let engine = BladeEngine::empty();

        let tmpl = r#"@for ($i in 1..4)<span>{{ $i }}</span>@endfor"#;
        let ctx = BladeContext::new();

        let html = engine.render_string(tmpl, &ctx);
        assert_eq!(html, "<span>1</span><span>2</span><span>3</span>");
    }

    #[test]
    fn test_blade_dot_notation() {
        let engine = BladeEngine::empty();

        let tmpl = r#"<p>{{ $user.profile.address.city }}</p>"#;
        let mut ctx = BladeContext::new();
        ctx.insert("user", json!({
            "profile": {
                "address": {
                    "city": "Bandung"
                }
            }
        }));

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("Bandung"));
    }

    #[test]
    fn test_blade_native_and_chained_filters() {
        let engine = BladeEngine::empty();

        let tmpl = r#"
        <p>Harga: {{ $harga | rupiah }}</p>
        <p>Slug: {{ $judul | slug }}</p>
        <p>Rantai: {{ $judul | slug | uppercase }}</p>
        <p>Potong: {{ $deskripsi | limit: 10 }}</p>
        "#;

        let mut ctx = BladeContext::new();
        ctx.insert("harga", 150000);
        ctx.insert("judul", "Lumina Web Framework Rust!");
        ctx.insert("deskripsi", "Ini adalah deskripsi yang sangat panjang sekali");

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("<p>Harga: Rp 150.000</p>"));
        assert!(html.contains("<p>Slug: lumina-web-framework-rust</p>"));
        assert!(html.contains("<p>Rantai: LUMINA-WEB-FRAMEWORK-RUST</p>"));
        assert!(html.contains("<p>Potong: Ini adalah...</p>"));
    }

    #[test]
    fn test_blade_wasm_filter_pipeline() {
        let mut engine = BladeEngine::empty();

        // WAT Plugin sederhana untuk sensor kata: mengubah semua 'x' menjadi '*'
        let wat = r#"
            (module
                (memory (export "memory") 1)
                (func (export "sensor") (param $ptr i32) (param $len i32) (result i32)
                    (local $i i32)
                    (local $curr i32)
                    (local.set $i (i32.const 0))
                    (block $break
                        (loop $loop
                            (br_if $break (i32.ge_u (local.get $i) (local.get $len)))
                            (local.set $curr (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
                            ;; jika huruf 'x' (ASCII 120), ganti jadi '*' (ASCII 42)
                            (if (i32.eq (local.get $curr) (i32.const 120))
                                (then
                                    (i32.store8
                                        (i32.add (local.get $ptr) (local.get $i))
                                        (i32.const 42)
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

        let plugin = crate::core::sandbox::WasmPlugin::from_wat("filter_sensor", wat).expect("Compile plugin gagal");
        engine.register_plugin(plugin);
        engine.register_wasm_filter("sensor", "filter_sensor", "sensor");

        let tmpl = r#"<p>Komentar: {{ $pesan | sensor }}</p>"#;
        let mut ctx = BladeContext::new();
        ctx.insert("pesan", "teks rahasia xxxx");

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("<p>Komentar: teks rahasia ****</p>"));
    }

    #[test]
    fn test_blade_wasm_directive() {
        let engine = BladeEngine::empty();

        // WAT Plugin untuk direktif: mengubah teks menjadi uppercase
        let wat = r#"
            (module
                (memory (export "memory") 1)
                (func (export "kapital") (param $ptr i32) (param $len i32) (result i32)
                    (local $i i32)
                    (local $curr i32)
                    (local.set $i (i32.const 0))
                    (block $break
                        (loop $loop
                            (br_if $break (i32.ge_u (local.get $i) (local.get $len)))
                            (local.set $curr (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
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

        let plugin = crate::core::sandbox::WasmPlugin::from_wat("text_tool", wat).expect("Compile plugin gagal");
        engine.register_plugin(plugin);

        let tmpl = r#"<div>Hasil: @wasm('text_tool', 'kapital', $nama)</div>"#;
        let mut ctx = BladeContext::new();
        ctx.insert("nama", "lumina hybrid");

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("<div>Hasil: LUMINA HYBRID</div>"));
    }

    #[test]
    fn test_blade_eval_is_directive() {
        let engine = BladeEngine::empty();

        let source = r#"
        fungsi hitung_promo(total, kupon) {
            jika kupon == "DISKON50" {
                kembalikan total * 0.50;
            }
            kembalikan 0;
        }
        "#;

        let runner = crate::core::sandbox::IsScriptRunner::from_source(source).unwrap();
        engine.register_is_script("promo.is", runner);

        let tmpl = r#"<span>Hemat: Rp @eval_is('promo.is', 'hitung_promo', $total, 'DISKON50')</span>"#;
        let mut ctx = BladeContext::new();
        ctx.insert("total", 100000);

        let html = engine.render_string(tmpl, &ctx);
        assert!(html.contains("<span>Hemat: Rp 50000</span>"));
    }

    #[test]
    fn test_blade_form_validation_and_helpers() {
        let engine = BladeEngine::empty();

        let tmpl = r#"
        <form method="POST" action="/register">
            @csrf
            <input type="email" name="email" value="{{ old('email', 'guest@example.com') }}">
            @error('email')
                <span class="error-msg">{{ $message }}</span>
            @enderror

            <input type="password" name="password">
            @error('password')
                <span class="error-msg">{{ $message }}</span>
            @enderror

            <input type="text" name="phone" value="{{ old('phone', '0812000000') }}">

            <link rel="stylesheet" href="{{ asset('css/custom.css') }}">

            @if(error('email'))
                <div class="alert alert-danger">Ada error pada input email!</div>
            @endif

            @if(anyErrors)
                <div class="summary">Formulir memiliki error validasi.</div>
            @endif
        </form>
        "#;

        let mut ctx = BladeContext::new();
        ctx.insert("csrf_token", "sec-token-123");

        // Suntikkan errors dan old input persis seperti yang dilakukan ViewEngine::render_with_session
        let errors_json = serde_json::json!({
            "email": "Email ini sudah digunakan oleh akun lain."
        });
        ctx.insert("errors", errors_json);

        let old_json = serde_json::json!({
            "email": "salim@example.com"
        });
        ctx.insert("old", old_json);

        let html = engine.render_string(tmpl, &ctx);

        // 1. Verifikasi old() mengambil nilai lama
        assert!(html.contains("value=\"salim@example.com\""));
        // 2. Verifikasi old() fallback ke default jika field tidak ada di old input
        assert!(html.contains("value=\"0812000000\""));
        // 3. Verifikasi @error('email') merender $message
        assert!(html.contains("<span class=\"error-msg\">Email ini sudah digunakan oleh akun lain.</span>"));
        // 4. Verifikasi @error('password') yang tidak ada error TIDAK dirender
        assert!(!html.contains("@error('password')"));
        assert!(!html.contains("@enderror"));
        // 5. Verifikasi asset() helper
        assert!(html.contains("href=\"/css/custom.css\""));
        // 6. Verifikasi error('email') dan anyErrors di @if
        assert!(html.contains("Ada error pada input email!"));
        assert!(html.contains("Formulir memiliki error validasi."));
        // 7. Verifikasi @csrf terisi
        assert!(html.contains("value=\"sec-token-123\""));
    }
}


