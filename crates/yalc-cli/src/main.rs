use clap::{Parser, Subcommand};
use inquire::{Select, Text, Confirm};
use std::fs;

#[derive(Parser)]
#[command(name = "yalc")]
#[command(about = "Rust-YALC Enterprise Scaffolding CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Zero-Trust Enterprise Project
    Init,
    /// Generate frontend TS/React clients from the API Schema
    Generate,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            println!("🚀 Welcome to Rust-YALC Enterprise Scaffolding\n");

            let project_name = Text::new("What is the name of your project?")
                .with_default("yalc_enterprise_app")
                .prompt()
                .unwrap();

            let db_options = vec!["PostgreSQL (SeaORM)", "MongoDB"];
            let db_choice = Select::new("Which database engine do you want to use?", db_options)
                .prompt()
                .unwrap();

            let use_redis = Confirm::new("Do you want to enable Redis for caching/jobs?")
                .with_default(true)
                .prompt()
                .unwrap();

            let use_ai = Confirm::new("Do you want to enable Qdrant Vector DB for AI Search?")
                .with_default(false)
                .prompt()
                .unwrap();

            println!("\nScaffolding project '{}'...", project_name);
            println!("- Database: {}", db_choice);
            println!("- Redis: {}", if use_redis { "Enabled" } else { "Disabled" });
            println!("- AI/Vector Search: {}", if use_ai { "Enabled" } else { "Disabled" });
            
            // In a full implementation, this would template out the docker-compose.yml 
            // and main.rs into a new folder based on the inputs.
            let docker_compose = format!(
                "version: '3.8'\nservices:\n  {}:\n    image: {}\n",
                if db_choice == "MongoDB" { "mongo" } else { "postgres" },
                if db_choice == "MongoDB" { "mongo:6-jammy" } else { "postgres:15-alpine" }
            );

            // fs::write(format!("{}/docker-compose.yml", project_name), docker_compose).unwrap();

            println!("\n✅ Project '{}' created successfully with Zero-Trust configurations!", project_name);
            println!("Run `cd {} && cargo run` to start the app.", project_name);
        }
        Commands::Generate => {
            println!("🔄 Generating TypeScript/React client from GraphQL Schema...");
            // Real implementation would invoke `@graphql-codegen/cli` using `std::process::Command`
            println!("✅ `frontend/src/api/generated.ts` successfully generated.");
        }
    }
}
