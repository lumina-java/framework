use std::process::Command;
use std::fs;

pub struct DatabaseExporter;

impl DatabaseExporter {
    pub fn export(connection: &str, db_url: &str, output_path: &str) -> Result<(), String> {
        match connection {
            "sqlite" => {
                // SQLite cukup copy file
                let source = db_url.trim_start_matches("sqlite:");
                fs::copy(source, output_path)
                    .map(|_| ())
                    .map_err(|e| format!("Gagal copy SQLite file: {}", e))
            },
            "mysql" => {
                // Gunakan mysqldump (asumsi terinstall)
                // Format URL: mysql://user:pass@host:port/db
                let output = Command::new("mysqldump")
                    .arg(format!("--url={}", db_url))
                    .arg("--result-file")
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Gagal menjalankan mysqldump: {}", e))?;

                if !output.status.success() {
                    return Err(format!("mysqldump error: {}", String::from_utf8_lossy(&output.stderr)));
                }
                Ok(())
            },
            "postgres" | "postgresql" => {
                // Gunakan pg_dump (asumsi terinstall)
                let output = Command::new("pg_dump")
                    .arg(db_url)
                    .arg("-f")
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Gagal menjalankan pg_dump: {}", e))?;

                if !output.status.success() {
                    return Err(format!("pg_dump error: {}", String::from_utf8_lossy(&output.stderr)));
                }
                Ok(())
            },
            _ => Err(format!("Connection {} tidak didukung untuk backup otomatis", connection)),
        }
    }
}
