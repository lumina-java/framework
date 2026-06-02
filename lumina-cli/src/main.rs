use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

mod generator;
mod skeleton;

#[derive(Parser)]
#[command(
    name = "lumina",
    author,
    version,
    about = "⚡ Lumina Framework CLI — Build fast, build beautiful.",
    long_about = None
)]
struct LuminaCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Lumina project from skeleton
    New {
        /// The name of the new project
        name: String,
    },

    /// Start the application server (cargo run)
    Serve,

    /// Start the server with auto-reload on code change (cargo-watch)
    Watch,

    // ─── Generators ──────────────────────────────────────────────────────────
    /// Generate a new Controller
    #[command(name = "make:controller")]
    MakeController {
        /// The name of the controller (e.g., PostController)
        name: String,
    },

    /// Generate a new Model and its migration file
    #[command(name = "make:model")]
    MakeModel {
        /// The name of the model (e.g., User, Product)
        name: String,
    },

    /// Generate a new empty migration file
    #[command(name = "make:migration")]
    MakeMigration {
        /// Description of the migration (e.g., create_posts_table)
        name: String,
    },

    /// Generate a full CRUD setup (Model, Controller, Migration, Views)
    #[command(name = "make:crud")]
    MakeCrud {
        /// The name of the CRUD resource (e.g., Post, Article)
        name: String,
        /// Optional field definitions e.g. "title:string body:text"
        #[arg(trailing_var_arg = true)]
        fields: Vec<String>,
    },

    /// Generate a ready-to-use Authentication system (Register & Login)
    #[command(name = "make:auth")]
    MakeAuth,

    // ─── Database ─────────────────────────────────────────────────────────────
    /// Run all pending database migrations
    Migrate,

    /// Rollback the last database migration
    #[command(name = "migrate:rollback")]
    MigrateRollback,

    /// Seed the database with initial dummy data
    #[command(name = "db:seed")]
    DbSeed,

    // ─── DevOps / Deployment ──────────────────────────────────────────────────
    /// Generate production Dockerfile & docker-compose.yml
    #[command(name = "make:docker")]
    MakeDocker,

    /// Generate production Nginx reverse proxy configuration
    #[command(name = "make:nginx")]
    MakeNginx,

    /// Generate Supervisor daemon configuration
    #[command(name = "make:supervisor")]
    MakeSupervisor,
}

fn main() {
    let args = LuminaCli::parse();

    match args.command {
        Commands::New { name } => {
            handle_new_project(&name);
        }
        Commands::Serve => {
            generator::cmd_serve();
        }
        Commands::Watch => {
            generator::cmd_watch();
        }
        Commands::MakeController { name } => {
            generator::generate_controller(&name);
        }
        Commands::MakeModel { name } => {
            generator::generate_model(&name, &[]);
        }
        Commands::MakeMigration { name } => {
            generator::generate_migration(&name);
        }
        Commands::MakeCrud { name, fields } => {
            generator::generate_crud(&name, &fields);
        }
        Commands::MakeAuth => {
            generator::generate_auth();
        }
        Commands::Migrate => {
            generator::run_migrate();
        }
        Commands::MigrateRollback => {
            generator::run_migrate_rollback();
        }
        Commands::DbSeed => {
            generator::run_db_seed();
        }
        Commands::MakeDocker => {
            generator::generate_docker();
        }
        Commands::MakeNginx => {
            generator::generate_nginx();
        }
        Commands::MakeSupervisor => {
            generator::generate_supervisor();
        }
    }
}

fn handle_new_project(name: &str) {
    println!(
        "{}",
        "✨ Welcome to Lumina Framework".bright_purple().bold()
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
    println!("  cd {}", name.bright_cyan());
    println!("  lumina serve\n");
    println!("{}", "Happy coding! 🦀".bright_yellow());
}
