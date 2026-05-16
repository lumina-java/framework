use crate::core::backup::exporter::DatabaseExporter;
use chrono::Local;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;

pub struct BackupManager {
    backup_path: String,
}

impl BackupManager {
    pub fn new() -> Self {
        let backup_path = "storage/backups".to_string();
        if !Path::new(&backup_path).exists() {
            fs::create_dir_all(&backup_path).unwrap();
        }
        Self { backup_path }
    }

    /// Menjalankan backup penuh (DB + Storage)
    pub async fn run_full_backup(&self, connection: &str, db_url: &str) -> Result<String, String> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let folder_name = format!("backup_{}", timestamp);
        let target_dir = format!("{}/{}", self.backup_path, folder_name);

        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

        // 1. Backup Database
        let db_file = format!("{}/database.sql", target_dir);
        DatabaseExporter::export(connection, db_url, &db_file)?;

        // 2. Backup Storage (Archive)
        let storage_zip = format!("{}/storage.zip", target_dir);
        self.archive_directory("storage/app/public", &storage_zip)?;

        // 3. Archive the whole backup folder into one ZIP
        let final_zip = format!("{}/{}.zip", self.backup_path, folder_name);
        self.archive_directory(&target_dir, &final_zip)?;

        // Cleanup temporary folder
        let _ = fs::remove_dir_all(&target_dir);

        Ok(final_zip)
    }

    fn archive_directory(&self, src_dir: &str, dst_file: &str) -> Result<(), String> {
        if !Path::new(src_dir).exists() {
            return Ok(()); // Skip jika dir tidak ada
        }

        let file = File::create(dst_file).map_err(|e| e.to_string())?;
        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::<()>::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        let walkdir = walkdir::WalkDir::new(src_dir);
        let it = walkdir.into_iter();

        for entry in it.filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.strip_prefix(Path::new(src_dir)).unwrap();

            if path.is_file() {
                zip.start_file(name.to_string_lossy(), options)
                    .map_err(|e| e.to_string())?;
                let mut f = File::open(path).map_err(|e| e.to_string())?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                zip.write_all(&buffer).map_err(|e| e.to_string())?;
            } else if !name.as_os_str().is_empty() {
                zip.add_directory(name.to_string_lossy(), options)
                    .map_err(|e| e.to_string())?;
            }
        }

        zip.finish().map_err(|e| e.to_string())?;
        Ok(())
    }
}
