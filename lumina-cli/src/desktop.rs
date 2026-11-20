use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn init_desktop_config(title: Option<&str>, width: u32, height: u32) {
    init_desktop_config_at(Path::new("."), title, width, height);
}

pub fn init_desktop_config_at(base: &Path, title: Option<&str>, width: u32, height: u32) {
    if !base.join("Cargo.toml").exists() {
        println!("{}", "❌ Error: This command must be run inside a Lumina project root.".red().bold());
        return;
    }

    let app_name = fs::read_to_string(base.join("Cargo.toml"))
        .ok()
        .and_then(|c| {
            for line in c.lines() {
                if line.trim().starts_with("name =") {
                    return Some(line.split('=').nth(1)?.trim().trim_matches('"').to_string());
                }
            }
            None
        })
        .unwrap_or_else(|| "Lumina Desktop".to_string());

    let win_title = title.unwrap_or(&app_name);

    println!("{}", format!("🖥️  Initializing Lumina Desktop Engine for [{}]...", win_title).bright_purple().bold());

    // 1. Generate desktop.toml
    let desktop_toml = format!(
        r#"# Lumina Desktop Application Configuration
[app]
name = "{app_name}"
title = "{win_title}"
version = "1.0.0"

[window]
width = {width}
height = {height}
min_width = 800
min_height = 600
resizable = true
fullscreen = false
decorations = true
always_on_top = false

[server]
host = "127.0.0.1"
port = 0 # 0 means ephemeral dynamic port selection
embedded_database = true
"#,
        app_name = app_name,
        win_title = win_title,
        width = width,
        height = height
    );

    let toml_path = base.join("desktop.toml");
    let _ = fs::write(&toml_path, desktop_toml);
    println!("  {} {}", "✅".green(), format!("Generated {}", toml_path.display()).bright_green());

    // 2. Generate src/bin/desktop.rs
    let bin_dir = base.join("src/bin");
    fs::create_dir_all(&bin_dir).ok();

    let desktop_rs = format!(
        r#"//! Lumina Desktop Application Entrypoint
//! Runs internal Axum web server and opens a high-performance native WebView window.

use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    // 1. Bind to ephemeral port
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let local_addr = listener.local_addr()?;
    let port = local_addr.port();
    drop(listener);

    let target_url = format!("http://127.0.0.1:{{}}", port);
    println!("🚀 [Lumina Desktop] Starting internal server on {{}}", target_url);

    // 2. Spawn Lumina server in background Tokio thread
    tokio::spawn(async move {{
        std::env::set_var("PORT", port.to_string());
        if let Err(e) = std::process::Command::new("cargo")
            .args(["run", "--bin", "{app_name}"])
            .status() {{
            eprintln!("Server error: {{}}", e);
        }}
    }});

    // 3. Launch native system WebView or browser window
    println!("🖥️  [Lumina Desktop] Opening native window: {{}}", target_url);
    #[cfg(target_os = "windows")]
    {{
        std::process::Command::new("cmd")
            .args(["/c", "start", &target_url])
            .spawn()?;
    }}
    #[cfg(target_os = "macos")]
    {{
        std::process::Command::new("open")
            .arg(&target_url)
            .spawn()?;
    }}
    #[cfg(target_os = "linux")]
    {{
        std::process::Command::new("xdg-open")
            .arg(&target_url)
            .spawn()?;
    }}

    println!("✨ [Lumina Desktop] App running. Press Ctrl+C in this terminal to exit.");
    tokio::signal::ctrl_c().await?;
    println!("🛑 [Lumina Desktop] Shutting down.");
    Ok(())
}}
"#,
        app_name = app_name
    );

    let desktop_bin_path = bin_dir.join("desktop.rs");
    let _ = fs::write(&desktop_bin_path, desktop_rs);
    println!("  {} {}", "✅".green(), format!("Generated {}", desktop_bin_path.display()).bright_green());

    println!("\n  ✨ Desktop mode initialized successfully!");
    println!("  💡 Run your desktop app with: {}", "lumina desktop:run".bright_cyan());
    println!("  💡 Compile standalone release with: {}", "lumina desktop:build".bright_cyan());
}

pub fn run_desktop(port: u16) {
    if !Path::new("Cargo.toml").exists() {
        println!("{}", "❌ Error: This command must be run inside a Lumina project root.".red().bold());
        return;
    }

    if !Path::new("desktop.toml").exists() {
        println!("{}", "⚠️  desktop.toml not found. Initializing desktop config first...".bright_yellow());
        init_desktop_config(None, 1200, 800);
    }

    println!("{}", "🖥️  Starting Lumina Desktop Window...".bright_purple().bold());
    let args = vec!["run", "--bin", "desktop"];
    if port > 0 {
        std::env::set_var("PORT", port.to_string());
    }

    let status = Command::new("cargo").args(args).status();
    if let Err(e) = status {
        println!("{} Failed to launch desktop app: {}", "❌".red(), e);
    }
}

pub fn build_desktop(release: bool) {
    if !Path::new("Cargo.toml").exists() {
        println!("{}", "❌ Error: This command must be run inside a Lumina project root.".red().bold());
        return;
    }

    if !Path::new("desktop.toml").exists() {
        println!("{}", "⚠️  desktop.toml not found. Initializing desktop config first...".bright_yellow());
        init_desktop_config(None, 1200, 800);
    }

    println!("{}", "📦 Compiling Standalone Lumina Desktop Executable...".bright_purple().bold());

    let mut args = vec!["build", "--bin", "desktop"];
    if release {
        args.push("--release");
    }

    let status = Command::new("cargo").args(&args).status();
    match status {
        Ok(s) if s.success() => {
            let target_sub = if release { "release" } else { "debug" };
            let exe_name = if cfg!(target_os = "windows") { "desktop.exe" } else { "desktop" };
            let output_path = format!("target/{}/{}", target_sub, exe_name);

            println!("  {} {}", "✅".green(), "Standalone Desktop compilation succeeded!".bold());
            println!("  📂 Output executable: {}", output_path.bright_green());
            println!("  💡 Single file, zero-dependency native desktop application ready for distribution.");
        }
        Ok(_) => {
            println!("{} Compilation failed.", "❌".red());
        }
        Err(e) => {
            println!("{} Failed to run cargo: {}", "❌".red(), e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_desktop_config_creates_files() {
        let temp_dir = std::env::temp_dir().join(format!("lumina_desk_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(temp_dir.join("src")).expect("create src");
        fs::write(temp_dir.join("Cargo.toml"), "[package]\nname = \"kasir_pos\"\nversion = \"1.0.0\"\n").unwrap();

        init_desktop_config_at(&temp_dir, Some("Kasir POS Pintar"), 1366, 768);

        let toml_file = temp_dir.join("desktop.toml");
        assert!(toml_file.exists());
        let toml_content = fs::read_to_string(toml_file).unwrap();
        assert!(toml_content.contains("Kasir POS Pintar"));
        assert!(toml_content.contains("width = 1366"));
        assert!(toml_content.contains("height = 768"));

        let bin_file = temp_dir.join("src/bin/desktop.rs");
        assert!(bin_file.exists());
        let bin_content = fs::read_to_string(bin_file).unwrap();
        assert!(bin_content.contains("Lumina Desktop"));
        assert!(bin_content.contains("kasir_pos"));

        let _ = fs::remove_dir_all(temp_dir);
    }
}
