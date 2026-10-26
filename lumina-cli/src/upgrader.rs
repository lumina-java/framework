use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Eksekusi perintah: lumina upgrade
pub fn run_upgrade(cli_only: bool, project_only: bool, local_path: Option<String>) {
    println!("{}", "⚡ LUMINA FRAMEWORK UPGRADE ENGINE".bright_purple().bold());
    println!("─────────────────────────────────────────────────────────────────");
    println!("  Current CLI Version : v{}", env!("CARGO_PKG_VERSION").bright_cyan());

    let in_project = is_lumina_project();

    // 1. Upgrade CLI Toolchain
    if !project_only {
        println!("\n[+] Meng-upgrade Lumina CLI Toolchain...");
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(120));

        #[cfg(windows)]
        prepare_windows_binary_for_replacement();

        let install_status = if let Some(local) = local_path {
            let cli_folder = Path::new(&local).join("lumina-cli");
            let target = if cli_folder.exists() {
                cli_folder
            } else {
                Path::new(&local).to_path_buf()
            };
            pb.set_message(format!("Mengompilasi CLI dari path lokal: {}...", target.display()));
            Command::new("cargo")
                .args(["install", "--path", target.to_str().unwrap(), "--force"])
                .output()
        } else {
            pb.set_message("Mengunduh & mengompilasi rilis terbaru dari GitHub (lumina-java/framework)...");
            Command::new("cargo")
                .args(["install", "--git", "https://github.com/lumina-java/framework.git", "lumina-cli", "--force"])
                .output()
        };

        match install_status {
            Ok(output) if output.status.success() => {
                pb.finish_with_message("✔ [CLI UPDATED] Lumina CLI toolchain berhasil diperbarui!".green().to_string());
            }
            Ok(output) => {
                let err = String::from_utf8_lossy(&output.stderr);
                pb.finish_with_message("✖ [CLI UPGRADE FAILED]".red().to_string());
                println!("  {} Gagal memperbarui CLI:", "Detail:".yellow());
                for line in err.lines().take(5) {
                    println!("    {}", line.dimmed());
                }
                println!("\n  {} Anda dapat mencoba manual:", "Saran:".cyan());
                println!("    cargo install --git https://github.com/lumina-java/framework.git lumina-cli --force\n");
            }
            Err(e) => {
                pb.finish_with_message("✖ [ERROR]".red().to_string());
                println!("  Gagal menjalankan cargo: {}", e);
            }
        }
    }

    // 2. Upgrade Project Dependencies (jika dijalankan di dalam proyek Lumina)
    if !cli_only {
        if in_project {
            println!("\n[+] Menyinkronkan Dependensi Proyek Lumina...");
            upgrade_project_dependencies();
        } else if project_only {
            println!("\n{} Tidak ditemukan berkas Cargo.toml proyek Lumina di direktori aktif.", "ℹ [INFO]".blue());
        }
    }

    println!("\n─────────────────────────────────────────────────────────────────");
    println!("✨ Proses upgrade selesai. Build fast, build beautiful! 🦀");
}

fn is_lumina_project() -> bool {
    let cargo_path = Path::new("Cargo.toml");
    if !cargo_path.exists() {
        return false;
    }
    if let Ok(content) = fs::read_to_string(cargo_path) {
        return content.contains("lumina-framework") || content.contains("lumina");
    }
    false
}

fn upgrade_project_dependencies() {
    let cargo_path = Path::new("Cargo.toml");
    if !cargo_path.exists() {
        return;
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Memeriksa pembaruan paket via 'cargo update'...");
    pb.enable_steady_tick(std::time::Duration::from_millis(120));

    // Coba update spesifik package lumina-framework dahulu
    let output = Command::new("cargo")
        .args(["update", "-p", "lumina-framework"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            pb.finish_with_message("✔ [PROJECT SYNCED] Dependensi lumina-framework berhasil diperbarui!".green().to_string());
        }
        _ => {
            // Fallback ke general cargo update
            pb.set_message("Menjalankan sinkronisasi 'cargo update'...");
            let fallback = Command::new("cargo").arg("update").output();
            if let Ok(f) = fallback {
                if f.status.success() {
                    pb.finish_with_message("✔ [PROJECT SYNCED] Lockfile proyek berhasil disinkronkan!".green().to_string());
                } else {
                    pb.finish_with_message("ℹ [INFO] Dependensi proyek sudah mutakhir.".blue().to_string());
                }
            } else {
                pb.finish_with_message("✖ Gagal menjalankan cargo update.".red().to_string());
            }
        }
    }
}

/// Menyiapkan executable Windows yang sedang aktif untuk di-overwrite oleh cargo install
/// dengan me-rename ke .old (karena Windows mengizinkan rename running exe).
#[cfg(windows)]
fn prepare_windows_binary_for_replacement() {
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(parent) = current_exe.parent() {
            let old_exe = parent.join("lumina.exe.old");
            let _ = fs::remove_file(&old_exe);
            let _ = fs::rename(&current_exe, &old_exe);
        }
    }
}

/// Membersihkan lumina.exe.old jika ada dari sesi update sebelumnya
pub fn cleanup_old_binary() {
    #[cfg(windows)]
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(parent) = current_exe.parent() {
            let old_exe = parent.join("lumina.exe.old");
            if old_exe.exists() {
                let _ = fs::remove_file(&old_exe);
            }
        }
    }
}

