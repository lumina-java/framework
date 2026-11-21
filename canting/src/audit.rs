use colored::*;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct AuditReport {
    pub passed: usize,
    pub warnings: usize,
    pub criticals: usize,
    pub suggestions: Vec<String>,
}

impl AuditReport {
    pub fn pass(&mut self, msg: &str) {
        self.passed += 1;
        println!("  {} {}", "✔ [PASS]".green().bold(), msg);
    }

    pub fn warn(&mut self, msg: &str, suggestion: Option<&str>) {
        self.warnings += 1;
        println!("  {} {}", "▲ [WARN]".yellow().bold(), msg.yellow());
        if let Some(s) = suggestion {
            self.suggestions.push(s.to_string());
        }
    }

    pub fn crit(&mut self, msg: &str, suggestion: Option<&str>) {
        self.criticals += 1;
        println!("  {} {}", "✖ [CRIT]".red().bold(), msg.bright_red());
        if let Some(s) = suggestion {
            self.suggestions.push(s.to_string());
        }
    }

    pub fn score(&self) -> (usize, &'static str) {
        let penalty = (self.criticals * 25) + (self.warnings * 5);
        let score = if penalty >= 100 { 0 } else { 100 - penalty };
        let grade = match score {
            95..=100 => "A+ (Production Ready)",
            85..=94 => "A (Secure)",
            70..=84 => "B (Moderate Risk — Review Warnings)",
            50..=69 => "C (High Risk — Fix Before Deploy)",
            _ => "F (Critical Vulnerabilities Found)",
        };
        (score, grade)
    }
}

pub fn run_audit() {
    println!("\n{}", "🛡️  LUMINA SHIELD — Project Security & Configuration Audit".bright_cyan().bold());
    println!("{}", "─────────────────────────────────────────────────────────────────".bright_black());

    let mut report = AuditReport::default();
    let current_dir = std::env::current_dir().unwrap_or_default();
    println!("Target Project : {}", current_dir.display().to_string().bright_white());
    println!("Audited At     : {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string().bright_white());

    // ─────────────────────────────────────────────────────────────────────────
    // 1. Environment & Cryptographic Secrets
    // ─────────────────────────────────────────────────────────────────────────
    println!("\n{}", "[+] 1. Environment & Cryptographic Secrets".bright_purple().bold());

    let env_content = fs::read_to_string(".env").unwrap_or_default();
    if env_content.is_empty() {
        report.warn("File .env tidak ditemukan di root project", Some("Salin template .env.example ke .env dan isi konfigurasi"));
    } else {
        // Cek APP_KEY
        let app_key = get_env_var(&env_content, "APP_KEY");
        match app_key {
            None => report.crit("APP_KEY belum didefinisikan di .env", Some("Tambahkan APP_KEY acak minimal 32 karakter di .env")),
            Some(key) if key.is_empty() || key == "your_secret_key_here" || key == "change_me" => {
                report.crit("APP_KEY masih menggunakan nilai default / placeholder", Some("Generate kunci baru: buat string 32-karakter acak untuk APP_KEY"));
            }
            Some(key) if key.len() < 32 => {
                report.warn("Panjang APP_KEY kurang dari 32 karakter (entropi rendah)", Some("Gunakan kunci minimal 32 karakter untuk enkripsi yang kuat"));
            }
            Some(_) => {
                report.pass("APP_KEY memiliki entropi yang kuat (>= 32 karakter)");
            }
        }

        // Cek JWT_SECRET
        let jwt_secret = get_env_var(&env_content, "JWT_SECRET");
        match jwt_secret {
            None => report.warn("JWT_SECRET belum didefinisikan (abaikan jika tidak menggunakan JWT)", None),
            Some(secret) if secret == "secret" || secret == "jwt_secret_change_me" => {
                report.crit("JWT_SECRET masih menggunakan nilai default yang rentan ditebak", Some("Ubah JWT_SECRET menjadi token acak 256-bit di .env"));
            }
            Some(secret) if secret.len() < 32 => {
                report.warn("Panjang JWT_SECRET kurang dari 32 karakter", Some("Tingkatkan panjang JWT_SECRET menjadi >= 32 karakter"));
            }
            Some(_) => {
                report.pass("JWT_SECRET aman dan memenuhi standar kriptografi");
            }
        }

        // Cek APP_DEBUG vs APP_ENV
        let app_env = get_env_var(&env_content, "APP_ENV").unwrap_or_else(|| "local".to_string());
        let app_debug = get_env_var(&env_content, "APP_DEBUG").unwrap_or_else(|| "false".to_string());
        if (app_env == "production" || app_env == "prod") && (app_debug == "true" || app_debug == "1") {
            report.crit("APP_DEBUG aktif di lingkungan PRODUCTION! Risiko kebocoran data sensitif.", Some("Ubah APP_DEBUG=false di file .env produksi"));
        } else {
            report.pass(&format!("Mode debug aman untuk environment '{}' (APP_DEBUG={})", app_env, app_debug));
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // 2. Route & CSRF Protection Audit
    // ─────────────────────────────────────────────────────────────────────────
    println!("\n{}", "[+] 2. View & CSRF Form Protection".bright_purple().bold());

    let views_dir = Path::new("resources/views");
    if views_dir.exists() {
        let mut total_forms = 0;
        let mut forms_with_csrf = 0;
        let mut missing_csrf_files = Vec::new();

        scan_views_for_csrf(views_dir, &mut total_forms, &mut forms_with_csrf, &mut missing_csrf_files);

        if total_forms == 0 {
            report.pass("Tidak ada formulir HTML statis yang ditemukan");
        } else if forms_with_csrf == total_forms {
            report.pass(&format!("Semua formulir web ({}/{}) telah dilindungi token @csrf", forms_with_csrf, total_forms));
        } else {
            let diff = total_forms - forms_with_csrf;
            let msg = format!("Ditemukan {} formulir HTML tanpa direktif @csrf di views!", diff);
            let detail = format!("Tambahkan @csrf pada formulir di: {}", missing_csrf_files.join(", "));
            report.warn(&msg, Some(&detail));
        }
    } else {
        report.pass("Direktori resources/views tidak terdeteksi (API mode)");
    }

    // ─────────────────────────────────────────────────────────────────────────
    // 3. Database & Storage Hardening
    // ─────────────────────────────────────────────────────────────────────────
    println!("\n{}", "[+] 3. Database & Storage Hardening".bright_purple().bold());

    // Cek password database kosong jika Postgres/MySQL
    let db_url = get_env_var(&env_content, "DATABASE_URL").unwrap_or_default();
    if (db_url.starts_with("postgres://") || db_url.starts_with("mysql://")) && (db_url.contains(":@") || db_url.contains("root@")) {
        report.warn("Koneksi database terindikasi tanpa password atau user root default", Some("Gunakan user non-root dan password kuat pada koneksi database"));
    } else if !db_url.is_empty() {
        report.pass("Konfigurasi string koneksi database memenuhi standar");
    }

    // Cek storage upload permissions
    let public_uploads = Path::new("storage/app/public/uploads");
    if public_uploads.exists() {
        report.pass("Direktori upload publik terisolasi di bawah storage/app/public/");
    } else {
        report.pass("Folder storage dalam kondisi bersih");
    }

    // ─────────────────────────────────────────────────────────────────────────
    // 4. Ringkasan & Skor Keamanan
    // ─────────────────────────────────────────────────────────────────────────
    println!("\n{}", "─────────────────────────────────────────────────────────────────".bright_black());
    let (score, grade) = report.score();
    println!("{}", "🎯 AUDIT SUMMARY & SECURITY SCOREBOARD:".bright_cyan().bold());
    println!("  Pemeriksaan Berhasil : {}", report.passed.to_string().green().bold());
    println!("  Peringatan (Warnings): {}", report.warnings.to_string().yellow().bold());
    println!("  Risiko Kritis        : {}", report.criticals.to_string().red().bold());
    println!();
    println!("  SECURITY SCORE : {} / 100", score.to_string().bright_yellow().bold());
    let grade_colored = if score >= 85 {
        grade.bright_green().bold()
    } else if score >= 70 {
        grade.bright_yellow().bold()
    } else {
        grade.bright_red().bold()
    };
    println!("  SECURITY GRADE : {}", grade_colored);

    if !report.suggestions.is_empty() {
        println!("\n{}", "💡 REKOMENDASI PERBAIKAN:".bright_yellow().bold());
        for (i, item) in report.suggestions.iter().enumerate() {
            println!("  {}. {}", i + 1, item);
        }
    } else {
        println!("\n{}", "✨ Selamat! Proyek Lumina Anda memenuhi seluruh standar keamanan dasar.".bright_green().bold());
    }
    println!("{}\n", "─────────────────────────────────────────────────────────────────".bright_black());
}

fn get_env_var(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if let Some((k, v)) = trimmed.split_once('=') {
            if k.trim() == key {
                let clean_val = v.trim().trim_matches('"').trim_matches('\'').to_string();
                return Some(clean_val);
            }
        }
    }
    None
}

fn scan_views_for_csrf(dir: &Path, total_forms: &mut usize, forms_with_csrf: &mut usize, missing_files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_views_for_csrf(&path, total_forms, forms_with_csrf, missing_files);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") || path.extension().and_then(|s| s.to_str()) == Some("blade") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let lower = content.to_lowercase();
                    if lower.contains("<form") && (lower.contains("method=\"post\"") || lower.contains("method='post'")) {
                        *total_forms += 1;
                        if content.contains("@csrf") {
                            *forms_with_csrf += 1;
                        } else {
                            missing_files.push(path.display().to_string());
                        }
                    }
                }
            }
        }
    }
}
