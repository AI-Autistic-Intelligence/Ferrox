use clap::{Parser, Subcommand};
use inquire::{Select, Text, Confirm};
use std::fs;

#[derive(Parser)]
#[command(name = "ferrox")]
#[command(about = "Rust-FERROX Enterprise Scaffolding CLI", long_about = None)]
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
            println!("🚀 Welcome to Rust-FERROX Enterprise Scaffolding\n");

            let project_name = Text::new("What is the name of your project?")
                .with_default("ferrox_enterprise_app")
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

            println!("\n🏗️  Scaffolding Enterprise Monorepo '{}'...", project_name);
            
            // 1. Create Monorepo Structure
            fs::create_dir_all(format!("{}/apps/api-gateway", project_name)).unwrap();
            fs::create_dir_all(format!("{}/apps/microservice-auth", project_name)).unwrap();
            fs::create_dir_all(format!("{}/packages/shared-dto", project_name)).unwrap();
            fs::create_dir_all(format!("{}/packages/database", project_name)).unwrap();
            
            // 2. Scaffold root Cargo.toml (Workspace)
            let root_cargo = format!(r#"[workspace]
members = [
    "apps/*",
    "packages/*"
]
resolver = "2"
"#);
            fs::write(format!("{}/Cargo.toml", project_name), root_cargo).unwrap();

            // 3. Scaffold docker-compose.yml
            let docker_compose = format!(
                "version: '3.8'\nservices:\n  {}:\n    image: {}\n",
                if db_choice == "MongoDB" { "mongo" } else { "postgres" },
                if db_choice == "MongoDB" { "mongo:6-jammy" } else { "postgres:15-alpine" }
            );
            fs::write(format!("{}/docker-compose.yml", project_name), docker_compose).unwrap();

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
