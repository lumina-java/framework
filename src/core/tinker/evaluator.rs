use super::formatter::TinkerFormatter;
use crate::database::connection::DatabasePool;
use sqlx::{Column, Row};
use std::sync::Arc;
use std::time::Instant;

/// Representasi hasil evaluasi satu ekspresi di Lumina Tinker
#[derive(Debug, Clone)]
pub enum TinkerResult {
    Table {
        columns: Vec<String>,
        rows: Vec<Vec<String>>,
        duration_ms: f64,
    },
    Scalar {
        value: String,
        duration_ms: f64,
    },
    Json {
        value: serde_json::Value,
        duration_ms: f64,
    },
    Info {
        message: String,
    },
    Error {
        message: String,
    },
    Exit,
}

impl TinkerResult {
    /// Render hasil ke dalam format string visual siap tampil di terminal
    pub fn render(&self) -> String {
        match self {
            TinkerResult::Table { columns, rows, duration_ms } => {
                let table_str = TinkerFormatter::render_table(columns, rows);
                let row_count = rows.len();
                format!("{}\n⚡ Executed in {:.2}ms ({} row{})\n", table_str, duration_ms, row_count, if row_count == 1 { "" } else { "s" })
            }
            TinkerResult::Scalar { value, duration_ms } => {
                TinkerFormatter::render_scalar(value, *duration_ms)
            }
            TinkerResult::Json { value, duration_ms } => {
                TinkerFormatter::render_json(value, *duration_ms)
            }
            TinkerResult::Info { message } => {
                format!("{}\n", message)
            }
            TinkerResult::Error { message } => {
                format!("❌ Error: {}\n", message)
            }
            TinkerResult::Exit => {
                "👋 Goodbye! Leaving Lumina Tinker.\n".to_string()
            }
        }
    }
}

/// Evaluator Engine untuk Lumina Tinker REPL
pub struct TinkerEvaluator {
    pool: Option<Arc<DatabasePool>>,
}

impl TinkerEvaluator {
    /// Buat evaluator baru dengan pool database opsional
    pub fn new(pool: Option<Arc<DatabasePool>>) -> Self {
        Self { pool }
    }

    /// Evaluasi input teks dari sesi REPL
    pub async fn eval(&self, input: &str) -> TinkerResult {
        let input = input.trim();
        let input = input.strip_suffix(';').unwrap_or(input).trim();

        if input.is_empty() {
            return TinkerResult::Info { message: String::new() };
        }

        // 1. Perintah Sistem & Shortcut
        match input.to_lowercase().as_str() {
            "exit" | "quit" => return TinkerResult::Exit,
            "clear" => return TinkerResult::Info { message: "\x1B[2J\x1B[1;1H".to_string() },
            "help" => return self.eval_help(),
            "env" => return self.eval_env(),
            "tables" | "models" => return self.eval_tables().await,
            _ => {}
        }

        // 2. Helper Facades
        if input.starts_with("Hash::make(") || input.starts_with("Hash::bcrypt(") {
            return self.eval_hash_make(input);
        }
        if input.starts_with("Hash::check(") {
            return self.eval_hash_check(input);
        }
        if input.starts_with("AutoSlug::slugify(") {
            return self.eval_auto_slug(input);
        }
        if input == "Uuid::new()" || input == "Uuid::new_v4()" {
            let start = Instant::now();
            let id = uuid::Uuid::new_v4().to_string();
            let dur = start.elapsed().as_secs_f64() * 1000.0;
            return TinkerResult::Scalar { value: format!("\"{}\"", id), duration_ms: dur };
        }

        // 3. Eloquent Syntax (contoh: User::all(), User::find(1), User::count())
        if input.contains("::") {
            if let Some(res) = self.eval_eloquent(input).await {
                return res;
            }
        }

        // 4. Raw SQL Statements
        let upper = input.to_uppercase();
        if upper.starts_with("SELECT ")
            || upper.starts_with("PRAGMA ")
            || upper.starts_with("SHOW ")
            || upper.starts_with("EXPLAIN ")
            || upper.starts_with("DESCRIBE ")
        {
            return self.eval_raw_select(input).await;
        }

        if upper.starts_with("INSERT ")
            || upper.starts_with("UPDATE ")
            || upper.starts_with("DELETE ")
            || upper.starts_with("CREATE ")
            || upper.starts_with("DROP ")
            || upper.starts_with("ALTER ")
        {
            return self.eval_raw_execute(input).await;
        }

        TinkerResult::Error {
            message: format!("Sintaks tidak dikenali: '{}'. Ketik 'help' untuk daftar perintah.", input),
        }
    }

    /// Evaluasi perintah 'help'
    fn eval_help(&self) -> TinkerResult {
        let help_text = r#"⚡ Lumina Tinker — Interactive REPL Help:

  [Perintah Sistem]
  help                    Tampilkan pesan bantuan ini
  tables / models         Tampilkan seluruh tabel yang ada di database
  env                     Tampilkan variabel lingkungan aktif (.env)
  clear                   Bersihkan layar terminal
  exit / quit             Keluar dari sesi Tinker

  [Sintaks Eloquent Model]
  <Model>::all()          Contoh: User::all(), Article::all()
  <Model>::find(<id>)     Contoh: User::find(1)
  <Model>::count()        Contoh: User::count()
  <Model>::where("<col>", "<val>")

  [Fluent Query Builder]
  DB::table("<table>").get()
  DB::table("<table>").count()
  DB::table("<table>").where_eq("<col>", "<val>").get()
  DB::table("<table>").limit(<n>).get()

  [Raw SQL Query]
  SELECT * FROM users LIMIT 5;
  INSERT INTO ... / UPDATE ... / DELETE ...

  [Security & Helper Facades]
  Hash::make("password")           Bcrypt hashing string
  Hash::check("plain", "hash")     Verifikasi hash Bcrypt
  AutoSlug::slugify("judul")       Generate slug SEO-friendly
  Uuid::new()                      Generate UUID v4 baru"#;

        TinkerResult::Info { message: help_text.to_string() }
    }

    /// Evaluasi perintah 'env'
    fn eval_env(&self) -> TinkerResult {
        let start = Instant::now();
        let mut rows = Vec::new();
        let columns = vec!["Variable".to_string(), "Value".to_string()];

        for (k, v) in std::env::vars() {
            let k_upper = k.to_uppercase();
            // Sensor variabel sensitif
            let display_val = if k_upper.contains("SECRET")
                || k_upper.contains("KEY")
                || k_upper.contains("PASSWORD")
                || k_upper.contains("TOKEN")
            {
                "********".to_string()
            } else {
                v
            };

            // Filter variabel yang relevan untuk proyek Lumina
            if k_upper.starts_with("APP_")
                || k_upper.starts_with("DATABASE_")
                || k_upper.starts_with("DB_")
                || k_upper.starts_with("REDIS_")
                || k_upper.starts_with("AWS_")
                || k_upper.starts_with("STORAGE_")
                || k_upper.starts_with("LUMINA_")
            {
                rows.push(vec![k, display_val]);
            }
        }

        rows.sort_by(|a, b| a[0].cmp(&b[0]));
        let dur = start.elapsed().as_secs_f64() * 1000.0;

        TinkerResult::Table { columns, rows, duration_ms: dur }
    }

    /// Evaluasi perintah 'tables' / 'models'
    async fn eval_tables(&self) -> TinkerResult {
        let pool = match &self.pool {
            Some(p) => p,
            None => return TinkerResult::Error { message: "Database pool belum terhubung.".to_string() },
        };

        let start = Instant::now();
        let sql = match pool.kind {
            crate::database::connection::DatabaseKind::Sqlite => {
                "SELECT name as table_name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
            }
            crate::database::connection::DatabaseKind::Postgres => {
                "SELECT table_name FROM information_schema.tables WHERE table_schema='public' ORDER BY table_name"
            }
            crate::database::connection::DatabaseKind::MySql => {
                "SELECT table_name FROM information_schema.tables WHERE table_schema=DATABASE() ORDER BY table_name"
            }
            _ => "SELECT name as table_name FROM sqlite_master WHERE type='table'",
        };

        match sqlx::query(sql).fetch_all(&pool.pool).await {
            Ok(rows_db) => {
                let columns = vec!["No".to_string(), "Table Name".to_string(), "Inferred Model".to_string()];
                let mut rows = Vec::new();

                for (idx, row) in rows_db.iter().enumerate() {
                    let table_name: String = row.try_get(0).unwrap_or_else(|_| "unknown".to_string());
                    let inferred_model = self.infer_model_name(&table_name);
                    rows.push(vec![(idx + 1).to_string(), table_name, inferred_model]);
                }

                let dur = start.elapsed().as_secs_f64() * 1000.0;
                TinkerResult::Table { columns, rows, duration_ms: dur }
            }
            Err(e) => TinkerResult::Error { message: format!("Gagal membaca daftar tabel: {}", e) },
        }
    }

    /// Konversi nama tabel menjadi nama Model (contoh: "users" -> "User", "order_items" -> "OrderItem")
    fn infer_model_name(&self, table: &str) -> String {
        let singular = if table.ends_with('s') && !table.ends_with("ss") {
            &table[..table.len() - 1]
        } else {
            table
        };

        singular
            .split('_')
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect()
    }

    /// Konversi nama Model menjadi nama tabel (contoh: "User" -> "users", "OrderItem" -> "order_items")
    fn infer_table_name(&self, model: &str) -> String {
        let mut table = String::new();
        for (i, c) in model.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 {
                    table.push('_');
                }
                table.push(c.to_ascii_lowercase());
            } else {
                table.push(c);
            }
        }
        if !table.ends_with('s') {
            table.push('s');
        }
        table
    }

    /// Evaluasi ekspresi Eloquent Model atau DB::table()
    async fn eval_eloquent(&self, input: &str) -> Option<TinkerResult> {
        let _pool = match &self.pool {
            Some(p) => p,
            None => return Some(TinkerResult::Error { message: "Database pool belum terhubung.".to_string() }),
        };

        // Case 1: DB::table("users").*
        if input.starts_with("DB::table(") {
            let re = regex::Regex::new(r#"DB::table\(\s*["']([^"']+)["']\s*\)(.*)"#).ok()?;
            if let Some(caps) = re.captures(input) {
                let table = caps.get(1)?.as_str();
                let rest = caps.get(2).map(|m| m.as_str()).unwrap_or("");

                if rest == ".get()" || rest.is_empty() {
                    let sql = format!("SELECT * FROM {}", table);
                    return Some(self.eval_raw_select(&sql).await);
                }
                if rest == ".count()" {
                    let sql = format!("SELECT COUNT(*) as count FROM {}", table);
                    return Some(self.eval_raw_select(&sql).await);
                }
                if rest.starts_with(".limit(") {
                    let limit_val = rest.trim_start_matches(".limit(").trim_end_matches(".get()").trim_end_matches(')').trim();
                    let sql = format!("SELECT * FROM {} LIMIT {}", table, limit_val);
                    return Some(self.eval_raw_select(&sql).await);
                }
            }
        }

        // Case 2: <Model>::all()
        if input.ends_with("::all()") {
            let model = input.strip_suffix("::all()")?.trim();
            let table = self.infer_table_name(model);
            let sql = format!("SELECT * FROM {}", table);
            return Some(self.eval_raw_select(&sql).await);
        }

        // Case 3: <Model>::count()
        if input.ends_with("::count()") {
            let model = input.strip_suffix("::count()")?.trim();
            let table = self.infer_table_name(model);
            let sql = format!("SELECT COUNT(*) as count FROM {}", table);
            return Some(self.eval_raw_select(&sql).await);
        }

        // Case 4: <Model>::find(<id>)
        if input.contains("::find(") {
            let parts: Vec<&str> = input.split("::find(").collect();
            if parts.len() == 2 {
                let model = parts[0].trim();
                let id_val = parts[1].trim_end_matches(')').trim();
                let table = self.infer_table_name(model);
                let sql = format!("SELECT * FROM {} WHERE id = {} LIMIT 1", table, id_val);
                return Some(self.eval_raw_select(&sql).await);
            }
        }

        None
    }

    /// Evaluasi raw query SELECT dan turunannya
    async fn eval_raw_select(&self, sql: &str) -> TinkerResult {
        let pool = match &self.pool {
            Some(p) => p,
            None => return TinkerResult::Error { message: "Database pool belum terhubung.".to_string() },
        };

        let start = Instant::now();
        let prepared = pool.prepare_sql(sql);

        match sqlx::query(&prepared).fetch_all(&pool.pool).await {
            Ok(rows_db) => {
                let dur = start.elapsed().as_secs_f64() * 1000.0;
                if rows_db.is_empty() {
                    return TinkerResult::Table {
                        columns: vec!["Result".to_string()],
                        rows: Vec::new(),
                        duration_ms: dur,
                    };
                }

                let columns: Vec<String> = rows_db[0]
                    .columns()
                    .iter()
                    .map(|c| c.name().to_string())
                    .collect();

                let mut rows = Vec::new();
                for row in &rows_db {
                    let mut row_data = Vec::new();
                    for idx in 0..columns.len() {
                        row_data.push(Self::decode_any_column(row, idx));
                    }
                    rows.push(row_data);
                }

                TinkerResult::Table { columns, rows, duration_ms: dur }
            }
            Err(e) => TinkerResult::Error { message: format!("SQL Error: {}", e) },
        }
    }

    /// Evaluasi raw statement INSERT / UPDATE / DELETE / DDL
    async fn eval_raw_execute(&self, sql: &str) -> TinkerResult {
        let pool = match &self.pool {
            Some(p) => p,
            None => return TinkerResult::Error { message: "Database pool belum terhubung.".to_string() },
        };

        let start = Instant::now();
        let prepared = pool.prepare_sql(sql);

        match sqlx::query(&prepared).execute(&pool.pool).await {
            Ok(res) => {
                let dur = start.elapsed().as_secs_f64() * 1000.0;
                let rows_affected = res.rows_affected();
                TinkerResult::Scalar {
                    value: format!("Query OK, {} row{} affected", rows_affected, if rows_affected == 1 { "" } else { "s" }),
                    duration_ms: dur,
                }
            }
            Err(e) => TinkerResult::Error { message: format!("Execution Error: {}", e) },
        }
    }

    /// Evaluasi Hash::make("password")
    fn eval_hash_make(&self, input: &str) -> TinkerResult {
        let start = Instant::now();
        let param = input
            .trim_start_matches("Hash::make(")
            .trim_start_matches("Hash::bcrypt(")
            .trim_end_matches(')')
            .trim_matches('"')
            .trim_matches('\'');

        match bcrypt::hash(param, 10) {
            Ok(h) => {
                let dur = start.elapsed().as_secs_f64() * 1000.0;
                TinkerResult::Scalar { value: format!("\"{}\"", h), duration_ms: dur }
            }
            Err(e) => TinkerResult::Error { message: format!("Bcrypt Hash Error: {}", e) },
        }
    }

    /// Evaluasi Hash::check("plain", "hash")
    fn eval_hash_check(&self, input: &str) -> TinkerResult {
        let start = Instant::now();
        let inner = input
            .trim_start_matches("Hash::check(")
            .trim_end_matches(')');

        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 2 {
            return TinkerResult::Error {
                message: "Format harus Hash::check(\"plain\", \"hash\")".to_string(),
            };
        }

        let plain = parts[0].trim().trim_matches('"').trim_matches('\'');
        let hash = parts[1].trim().trim_matches('"').trim_matches('\'');

        match bcrypt::verify(plain, hash) {
            Ok(valid) => {
                let dur = start.elapsed().as_secs_f64() * 1000.0;
                TinkerResult::Scalar { value: valid.to_string(), duration_ms: dur }
            }
            Err(e) => TinkerResult::Error { message: format!("Verify Error: {}", e) },
        }
    }

    /// Evaluasi AutoSlug::slugify("judul")
    fn eval_auto_slug(&self, input: &str) -> TinkerResult {
        let start = Instant::now();
        let raw = input
            .trim_start_matches("AutoSlug::slugify(")
            .trim_end_matches(')')
            .trim_matches('"')
            .trim_matches('\'');

        let slug = crate::database::observer::AutoSlug::slugify(raw);
        let dur = start.elapsed().as_secs_f64() * 1000.0;
        TinkerResult::Scalar { value: format!("\"{}\"", slug), duration_ms: dur }
    }

    /// Ekstraksi nilai kolom dinamis dari AnyRow tanpa panic
    fn decode_any_column(row: &sqlx::any::AnyRow, index: usize) -> String {
        if let Ok(val) = row.try_get::<Option<String>, _>(index) {
            return val.unwrap_or_else(|| "NULL".to_string());
        }
        if let Ok(val) = row.try_get::<Option<i64>, _>(index) {
            return val.map(|v| v.to_string()).unwrap_or_else(|| "NULL".to_string());
        }
        if let Ok(val) = row.try_get::<Option<i32>, _>(index) {
            return val.map(|v| v.to_string()).unwrap_or_else(|| "NULL".to_string());
        }
        if let Ok(val) = row.try_get::<Option<f64>, _>(index) {
            return val.map(|v| v.to_string()).unwrap_or_else(|| "NULL".to_string());
        }
        if let Ok(val) = row.try_get::<Option<bool>, _>(index) {
            return val.map(|v| v.to_string()).unwrap_or_else(|| "NULL".to_string());
        }
        "<?>".to_string()
    }
}
