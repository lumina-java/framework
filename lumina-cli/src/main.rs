use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

mod skeleton;

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

    pb.set_message("Preparing Lumina skeleton...");
    pb.enable_steady_tick(std::time::Duration::from_millis(120));

    std::thread::sleep(std::time::Duration::from_secs(1));

    let path = Path::new(name);
    if path.exists() {
        pb.finish_with_message("❌ Error: Directory already exists!");
        std::process::exit(1);
    }

    if let Err(e) = fs::create_dir_all(path) {
        pb.finish_with_message(format!("❌ Error: Could not create directory: {}", e));
        std::process::exit(1);
    }

    pb.set_message("Building project structure...");
    skeleton::create_full_skeleton(path, name);

    pb.finish_with_message("✅ Project created successfully!");

    println!("\n{}", "Next steps:".bright_green().bold());
    println!("  cd {}", name);
    println!("  cp .env.example .env");
    println!("  cargo run\n");
    println!("{}", "Enjoy building something amazing! 🦀".bright_yellow());
}
