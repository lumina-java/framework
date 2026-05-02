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
    /// Start the HTTP server
    #[command(name = "serve")]
    Serve,
    /// Watch for changes and auto-reload the server
    #[command(name = "watch")]
    Watch,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
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
        Commands::Serve => {
            println!("🚀 Starting Lumina Server...");
            let mut child = std::process::Command::new("cargo")
                .arg("run")
                .arg("--bin")
                .arg("lumina-server")
                .spawn()
                .expect("Failed to start server");
            
            let _ = child.wait();
        }
        Commands::Watch => {
            println!("👀 Lumina is watching your code... (Auto-reload enabled)");
            let mut child = std::process::Command::new("cargo")
                .arg("watch")
                .arg("-x")
                .arg("run --bin lumina-server")
                .spawn()
                .expect("Failed to start cargo-watch. Make sure it's installed: cargo install cargo-watch");
            
            let _ = child.wait();
        }
    }
}
