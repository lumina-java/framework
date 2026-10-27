use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

mod generator;
mod skeleton;
mod audit;
mod migrator;
mod upgrader;

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
    /// Create a new Lumina project
    New {
        /// The name of the new project
        name: String,
        /// Database engine to configure: sqlite, postgres, mysql, all [default: sqlite]
        #[arg(short, long, default_value = "sqlite")]
        db: String,
        /// Force offline generation using embedded templates without fetching from GitHub
        #[arg(long)]
        offline: bool,
    },

    /// Start the application server (cargo run)
    Serve,

    /// Start the server with auto-reload on code change (cargo-watch)
    Watch,

    /// Clean unused build artifacts, compiler cache, and temporary files
    Clean {
        /// Force full clean (cargo clean) instead of smart sweep
        #[arg(long)]
        purge: bool,
    },

    /// Upgrade Lumina CLI toolchain and current project dependencies
    #[command(name = "upgrade", alias = "update")]
    Upgrade {
        /// Upgrade only the CLI toolchain without updating the current project
        #[arg(long)]
        cli_only: bool,
        /// Update only current project dependencies without upgrading the CLI
        #[arg(long)]
        project_only: bool,
        /// Upgrade CLI from a local directory path instead of GitHub
        #[arg(long)]
        local: Option<String>,
    },

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

    /// Generate a new Model Factory for testing and seeding
    #[command(name = "make:factory")]
    MakeFactory {
        /// The name of the factory (e.g., UserFactory)
        name: String,
    },

    /// Generate a new Blade view template (e.g., products.index)
    #[command(name = "make:view")]
    MakeView {
        /// The name of the view (e.g., products.index or auth.profile)
        name: String,
    },

    /// Generate a new Background Job class (e.g., SendWelcomeEmail)
    #[command(name = "make:job")]
    MakeJob {
        /// The name of the job (e.g., SendWelcomeEmail or ProcessInvoice)
        name: String,
    },

    /// Generate a new Lumina Live reactive component
    #[command(name = "make:live")]
    MakeLive {
        /// The name of the live component (e.g., Counter, SearchUser)
        name: String,
    },

    // ─── Queue & Background Workers ──────────────────────────────────────────
    /// Start processing background jobs from the queue
    #[command(name = "queue:work")]
    QueueWork {
        /// The queue name to listen to (default: default)
        #[arg(short, long, default_value = "default")]
        queue: String,
    },

    // ─── Security & Auditing ──────────────────────────────────────────────────
    /// Run Lumina Shield to audit security, secrets, and environment configuration
    Audit,

    // ─── Database ─────────────────────────────────────────────────────────────
    /// Run all pending database migrations
    Migrate,

    /// Rollback the last database migration
    #[command(name = "migrate:rollback")]
    MigrateRollback,

    /// Show the status of each migration (Ran or Pending)
    #[command(name = "migrate:status")]
    MigrateStatus,

    /// Seed the database with initial dummy data
    #[command(name = "db:seed")]
    DbSeed,

    // ─── DevOps / Deployment ──────────────────────────────────────────────────
    /// Generate production Dockerfile, docker-compose.yml & aaPanel/Nginx reverse proxy
    #[command(name = "make:docker")]
    MakeDocker {
        /// Project / domain / service name (e.g., pos.sugandi.my.id or service_pos_event)
        name: Option<String>,
        /// Port to expose on the host (default: 8000)
        #[arg(short, long)]
        port: Option<u16>,
    },

    /// Generate production Nginx reverse proxy configuration
    #[command(name = "make:nginx")]
    MakeNginx,

    /// Generate Supervisor daemon configuration
    #[command(name = "make:supervisor")]
    MakeSupervisor,

    // ─── WebAssembly Plugins ──────────────────────────────────────────────────
    /// Generate a new WASM plugin starter crate in plugins/<name>
    #[command(name = "plugin:new")]
    PluginNew {
        /// Name of the plugin (e.g. rupiah, markdown, sensor)
        name: String,
    },

    /// Compile WASM plugin crate(s) to target wasm32 and copy to plugins/<name>.wasm
    #[command(name = "plugin:build")]
    PluginBuild {
        /// Name of the plugin (optional, builds all if omitted)
        name: Option<String>,
    },

    /// List all installed WASM plugins and Indonesian Script files
    #[command(name = "plugin:list")]
    PluginList,
}

fn main() {
    upgrader::cleanup_old_binary();
    let args = LuminaCli::parse();

    match args.command {
        Commands::New { name, db, offline } => {
            handle_new_project(&name, &db, offline);
        }
        Commands::Serve => {
            generator::cmd_serve();
        }
        Commands::Watch => {
            generator::cmd_watch();
        }
        Commands::Clean { purge } => {
            generator::cmd_clean(purge);
        }
        Commands::MakeController { name } => {
            generator::generate_controller(&name);
        }
        Commands::MakeModel { name } => {
            generator::generate_model(&name);
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
        Commands::MakeFactory { name } => {
            generator::generate_factory(&name);
        }
        Commands::MakeView { name } => {
            generator::generate_view(&name);
        }
        Commands::MakeJob { name } => {
            generator::generate_job(&name);
        }
        Commands::MakeLive { name } => {
            generator::generate_live(&name);
        }
        Commands::QueueWork { queue } => {
            tokio::runtime::Runtime::new()
                .expect("Failed to start Tokio runtime for queue worker")
                .block_on(migrator::run_queue_worker_async(&queue));
        }
        Commands::Audit => {
            audit::run_audit();
        }
        Commands::Migrate => {
            tokio::runtime::Runtime::new()
                .expect("Failed to start Tokio runtime for migration")
                .block_on(migrator::run_migrate_async());
        }
        Commands::MigrateRollback => {
            tokio::runtime::Runtime::new()
                .expect("Failed to start Tokio runtime for migration")
                .block_on(migrator::run_rollback_async());
        }
        Commands::MigrateStatus => {
            tokio::runtime::Runtime::new()
                .expect("Failed to start Tokio runtime for migration")
                .block_on(migrator::run_status_async());
        }
        Commands::DbSeed => {
            tokio::runtime::Runtime::new()
                .expect("Failed to start Tokio runtime for seeding")
                .block_on(migrator::run_seed_async());
        }
        Commands::MakeDocker { name, port } => {
            generator::generate_docker(name, port);
        }
        Commands::MakeNginx => {
            generator::generate_nginx();
        }
        Commands::MakeSupervisor => {
            generator::generate_supervisor();
        }
        Commands::PluginNew { name } => {
            generator::generate_plugin_new(&name);
        }
        Commands::PluginBuild { name } => {
            generator::cmd_plugin_build(name.as_deref());
        }
        Commands::PluginList => {
            generator::cmd_plugin_list();
        }
        Commands::Upgrade { cli_only, project_only, local } => {
            upgrader::run_upgrade(cli_only, project_only, local);
        }
    }
}

fn handle_new_project(name: &str, db: &str, offline: bool) {
    let db_normalized = db.to_lowercase();
    println!(
        "{}",
        "✨ Welcome to Lumina Framework".bright_purple().bold()
    );
    println!("🚀 Creating new project: {} (Database: {})\n", name.bright_blue(), db_normalized.bright_yellow());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    pb.set_message("Preparing Lumina project...");
    pb.enable_steady_tick(std::time::Duration::from_millis(120));

    let path = Path::new(name);
    if path.exists() {
        pb.finish_with_message("❌ Error: Directory already exists!");
        std::process::exit(1);
    }

    let mut cloned_successfully = false;

    // Fetch live template from GitHub (lumina-java/lumina) unless offline flag is set
    if !offline {
        pb.set_message("Fetching latest starter template from GitHub (lumina-java/lumina)...");
        let git_clone_status = std::process::Command::new("git")
            .args(["clone", "--depth", "1", "https://github.com/lumina-java/lumina.git", name])
            .output();

        if let Ok(output) = git_clone_status {
            if output.status.success() {
                // Clean up cloned .git folder
                let _ = fs::remove_dir_all(path.join(".git"));
                // Personalize Cargo.toml with project name
                let cargo_path = path.join("Cargo.toml");
                if let Ok(content) = fs::read_to_string(&cargo_path) {
                    let customized = content.replace("name = \"lumina-app\"", &format!("name = \"{}\"", name));
                    let _ = fs::write(&cargo_path, customized);
                }
                // Initialize fresh git repo
                let _ = std::process::Command::new("git").args(["init", name]).output();
                cloned_successfully = true;
            }
        }
    }

    if !cloned_successfully {
        pb.set_message("Building project using internal offline template...");
        if let Err(e) = fs::create_dir_all(path) {
            pb.finish_with_message(format!("❌ Error: Could not create directory: {}", e));
            std::process::exit(1);
        }
        skeleton::create_full_skeleton(path, name, &db_normalized);
    }

    pb.finish_with_message("✅ Project created successfully!");

    println!("\n{}", "Next steps:".bright_green().bold());
    println!("  cd {}", name.bright_cyan());
    println!("  lumina serve\n");
    println!("{}", "Happy coding! 🦀".bright_yellow());
}
