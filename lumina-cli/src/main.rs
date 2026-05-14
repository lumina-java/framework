use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum Cargo {
    Lumina(LuminaCli),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct LuminaCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Lumina project
    New {
        /// The name of the project
        name: String,
    },
}

fn main() {
    let Cargo::Lumina(args) = Cargo::parse();

    match args.command {
        Commands::New { name } => {
            handle_new_project(&name);
        }
    }
}

fn handle_new_project(name: &str) {
    println!(
        "{}",
        "✨ Welcoming you to Lumina Framework"
            .bright_purple()
            .bold()
    );
    println!("🚀 Creating new project: {}\n", name.bright_blue());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    pb.set_message("Downloading Lumina skeleton...");
    pb.enable_steady_tick(std::time::Duration::from_millis(120));

    // Simulation of download (In real world: reqwest::blocking::get(URL))
    std::thread::sleep(std::time::Duration::from_secs(2));

    pb.set_message("Extracting files...");

    let path = Path::new(name);
    if path.exists() {
        pb.finish_with_message("❌ Error: Directory already exists!");
        std::process::exit(1);
    }

    if let Err(e) = fs::create_dir_all(path) {
        pb.finish_with_message(format!("❌ Error: Could not create directory: {}", e));
        std::process::exit(1);
    }

    // In a real implementation, we would extract a ZIP here.
    // For this demonstration, we'll create the basic structure.
    create_skeleton(path, name);

    pb.finish_with_message("✅ Project created successfully!");

    println!("\n{}", "Next steps:".bright_green().bold());
    println!("  cd {}", name);
    println!("  cp .env.example .env");
    println!("  cargo run\n");
    println!("{}", "Enjoy building something amazing! 🦀".bright_yellow());
}

fn create_skeleton(base: &Path, name: &str) {
    // Create folders
    let folders = [
        "src",
        "src/app",
        "src/core",
        "resources",
        "resources/views",
        "storage",
    ];
    for folder in folders {
        fs::create_dir_all(base.join(folder)).ok();
    }

    // Create Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
lumina = {{ git = "https://github.com/lumina-java/framework.git" }}
tokio = {{ version = "1", features = ["full"] }}
axum = "0.7"
"#,
        name
    );

    fs::write(base.join("Cargo.toml"), cargo_toml).ok();

    // Create main.rs
    let main_rs = r#"use lumina::prelude::*;

#[tokio::main]
async fn main() {
    let app = Application::new();
    println!("🚀 Lumina server starting...");
    app.serve("127.0.0.1:8000").await;
}
"#;
    fs::write(base.join("src/main.rs"), main_rs).ok();

    // Create .env.example
    let env_example = "APP_NAME=Lumina\nDATABASE_URL=sqlite:database.sqlite\n";
    fs::write(base.join(".env.example"), env_example).ok();
}
