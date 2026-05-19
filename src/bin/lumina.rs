use clap::{Parser, Subcommand};
use lumina::cli;

#[derive(Parser)]
#[command(name = "lumina")]
#[command(about = "Lumina Framework CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Lumina project
    #[command(name = "new")]
    New {
        /// Name of the project
        name: String,
    },
    /// Generate a new controller
    #[command(name = "make:controller")]
    MakeController {
        /// Name of the controller (e.g. UserController)
        name: String,
    },
    /// Generate a new model
    #[command(name = "make:model")]
    MakeModel {
        /// Name of the model (e.g. Product)
        name: String,
    },
    /// Generate a new migration
    #[command(name = "make:migration")]
    MakeMigration {
        /// Name of the migration (e.g. create_products_table)
        name: String,
    },
    /// Generate a new background job
    #[command(name = "make:job")]
    MakeJob {
        /// Name of the job (e.g. SendEmailJob)
        name: String,
    },
    /// Generate a new request for validation
    #[command(name = "make:request")]
    MakeRequest {
        /// Name of the request (e.g. StoreUserRequest)
        name: String,
    },
    /// Generate a new service class
    #[command(name = "make:service")]
    MakeService {
        /// Name of the service (e.g. UserService)
        name: String,
    },
    /// Start the HTTP server
    #[command(name = "serve")]
    Serve,
    /// Watch for changes and auto-reload the server
    #[command(name = "watch")]
    Watch,
    /// Generate a full CRUD scaffolding (Model, Controller, Migration, Views)
    #[command(name = "make:crud")]
    MakeCrud {
        /// Name of the entity (e.g. Patient)
        name: String,
        /// Fields for the entity (e.g. name:string price:integer)
        #[arg(num_args = 0..)]
        fields: Vec<String>,
    },
    /// Generate a full Authentication scaffolding (Register, Login, Views)
    #[command(name = "make:auth")]
    MakeAuth,
    /// Run all pending database migrations
    #[command(name = "migrate")]
    Migrate,
    /// Check the status of database migrations
    #[command(name = "migrate:status")]
    MigrateStatus,
    /// Rollback the last applied migration
    #[command(name = "migrate:rollback")]
    MigrateRollback,
    /// Seed the database with records
    #[command(name = "db:seed")]
    DbSeed,
    /// Generate a new seeder
    #[command(name = "make:seeder")]
    MakeSeeder {
        /// Name of the seeder (e.g. UserSeeder)
        name: String,
    },
    /// Generate a new factory
    #[command(name = "make:factory")]
    MakeFactory {
        /// Name of the factory (e.g. UserFactory)
        name: String,
    },
    /// Start an interactive REPL session
    #[command(name = "tinker")]
    Tinker,
    /// Generate Dockerfile and docker-compose.yml
    #[command(name = "make:docker")]
    MakeDocker,
    /// Generate Nginx configuration
    #[command(name = "make:nginx")]
    MakeNginx,
    /// Generate Supervisor configuration
    #[command(name = "make:supervisor")]
    MakeSupervisor,
    /// Run full backup (Database & Storage)
    #[command(name = "backup:run")]
    BackupRun,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            cli::handle_new(&name).await;
        }
        Commands::MakeController { name } => {
            cli::handle_make_controller(&name).await;
        }
        Commands::MakeModel { name } => {
            cli::handle_make_model(&name).await;
        }
        Commands::MakeMigration { name } => {
            cli::handle_make_migration(&name).await;
        }
        Commands::MakeJob { name } => {
            cli::handle_make_job(&name).await;
        }
        Commands::MakeRequest { name } => {
            cli::handle_make_request(&name).await;
        }
        Commands::MakeService { name } => {
            cli::handle_make_service(&name).await;
        }
        Commands::MakeCrud { name, fields } => {
            cli::handle_make_crud(&name, fields).await;
        }
        Commands::MakeAuth => {
            cli::handle_make_auth().await;
        }
        Commands::Migrate => {
            println!("🔄 Running migrations via cargo...");
            let _ = std::process::Command::new("cargo")
                .arg("run")
                .arg("--")
                .arg("migrate")
                .status();
        }
        Commands::MigrateStatus => {
            let _ = std::process::Command::new("cargo")
                .arg("run")
                .arg("--")
                .arg("migrate:status")
                .status();
        }
        Commands::MigrateRollback => {
            let _ = std::process::Command::new("cargo")
                .arg("run")
                .arg("--")
                .arg("migrate:rollback")
                .status();
        }
        Commands::DbSeed => {
            println!("🌱 Seeding database via cargo...");
            let _ = std::process::Command::new("cargo")
                .arg("run")
                .arg("--")
                .arg("db:seed")
                .status();
        }
        Commands::MakeSeeder { name } => {
            cli::handle_make_seeder(&name).await;
        }
        Commands::MakeFactory { name } => {
            cli::handle_make_factory(&name).await;
        }
        Commands::Tinker => {
            println!("🔍 Starting Tinker via cargo...");
            let _ = std::process::Command::new("cargo")
                .arg("run")
                .arg("--")
                .arg("tinker")
                .status();
        }
        Commands::MakeDocker => {
            cli::handle_make_docker().await;
        }
        Commands::MakeNginx => {
            cli::handle_make_nginx().await;
        }
        Commands::MakeSupervisor => {
            cli::handle_make_supervisor().await;
        }
        Commands::BackupRun => {
            cli::handle_backup().await;
        }
        Commands::Serve => {
            println!("🚀 Starting Lumina Server...");
            let mut child = std::process::Command::new("cargo")
                .arg("run")
                .spawn()
                .expect("Failed to start server");

            let _ = child.wait();
        }
        Commands::Watch => {
            println!("👀 Lumina is watching your code... (Auto-reload enabled)");
            let mut child = std::process::Command::new("cargo")
                .arg("watch")
                .arg("-x")
                .arg("run")
                .spawn()
                .expect("Failed to start cargo-watch. Make sure it's installed: cargo install cargo-watch");

            let _ = child.wait();
        }
    }
}
