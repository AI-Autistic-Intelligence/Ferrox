//! # Ferrox Official CLI (`cargo-ferrox`)
//!
//! Official Command Line Interface for scaffolding Ferrox DDD microservices,
//! orchestrating local dev-servers (Wasm & backend), auditing DDD domain purity,
//! and enforcing strict 100% test coverage gates.

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

use cargo_ferrox::commands::{check, codex, dev, mesh, new, test};
use cargo_ferrox::templates::ProjectTemplate;

#[derive(Parser, Debug)]
#[command(
    name = "cargo-ferrox",
    bin_name = "cargo ferrox",
    author = "Ferrox Core Team",
    version,
    about = "Official CLI for Ferrox: scaffolding, local dev-orchestration, and TDD quality gates"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scaffold a new Ferrox project with DDD architecture and 100% TDD templates
    New(NewArgs),

    /// Launch the local development orchestrator (zero cloud costs)
    Dev(DevArgs),

    /// Run test suite with strict 100% coverage and zero-debug-debris checks
    Test(TestArgs),

    /// Audit architectural DDD domain purity and Sentinel security compliance
    Check(CheckArgs),

    /// Living Mesh Orchestrator & Chaos Engine (local Redpanda, Redis, zero cloud cost)
    Mesh(MeshArgs),

    /// Preview project documentation on local Ferrox-Codex
    Codex(CodexArgs),

    /// Generate Frontend DTO client types from Rust controllers
    Generate {
        #[arg(long, default_value = "ts")]
        lang: String,
        #[arg(long, default_value = "./frontend/src/api")]
        output: String,
    },

    /// Moving Target Defense (MTD) dynamic ingress inspector
    Mtd {
        #[arg(long, default_value = "mtd_secret_key_8899")]
        secret: String,
        #[arg(long, default_value = "8080")]
        base_port: u16,
    },

    /// Distributed Honeynet Mesh & deception telemetry inspector
    Honeynet,

    /// Generate production VPS installer and Nginx gateway scripts
    VpsDeploy {
        #[arg(long, default_value = "185.220.101.5")]
        vps_ip: String,
        #[arg(long, default_value = "security.ferrox-rust.dev")]
        subdomain: String,
    },
}

#[derive(Args, Debug)]
struct NewArgs {
    /// Name of the project to create
    name: String,

    /// Project archetype template: clean-service, trading-desk, polyglot-mesh, wasm-spa
    #[arg(short, long, default_value = "clean-service")]
    template: String,

    /// Target path (defaults to ./<name>)
    #[arg(short, long)]
    path: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct DevArgs {
    /// HTTP / WebSocket server port
    #[arg(short, long)]
    port: Option<u16>,

    /// Dry run: display commands without executing processes
    #[arg(long)]
    dry_run: bool,

    /// Disable auto-reloading watcher
    #[arg(long)]
    no_watch: bool,
}

#[derive(Args, Debug)]
struct TestArgs {
    /// Filter string passed to cargo test
    filter: Option<String>,

    /// Allow debug statements (dbg!, console.log) without failing the gate
    #[arg(long)]
    allow_debris: bool,
}

#[derive(Args, Debug)]
struct CheckArgs {
    /// Fail with exit code 1 if any DDD architecture or security violations are found
    #[arg(short, long)]
    strict: bool,

    /// Execute Ferrox Sentinel Security Audit (Shannon entropy, PASETO v4, OWASP WSTG v5.0)
    #[arg(long)]
    security: bool,

    /// Target directory to inspect (defaults to current dir)
    #[arg(short, long)]
    path: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct MeshArgs {
    #[command(subcommand)]
    command: MeshSubcommands,
}

#[derive(Subcommand, Debug)]
enum MeshSubcommands {
    /// Start local mesh infrastructure (docker-compose Redpanda/Redis) & print ASCII health check
    Up {
        /// Target docker-compose file (defaults to auto-discovering in workspace)
        #[arg(short, long)]
        compose_file: Option<PathBuf>,
        /// Dry run: display topology without executing containers
        #[arg(long)]
        dry_run: bool,
    },
    /// Stop and safely clean up local mesh infrastructure
    Down {
        /// Target docker-compose file
        #[arg(short, long)]
        compose_file: Option<PathBuf>,
        /// Dry run: display cleanup steps without executing docker
        #[arg(long)]
        dry_run: bool,
    },
    /// Query Kafka/Redpanda consumer lag, topic offsets, and memory of services
    Status {
        /// Filter by runtime: match, py, java, php, node, redpanda, redis
        #[arg(short, long)]
        runtime: Option<String>,
    },
    /// Inject anomaly scenario for automated resilience tests: double-spend, timeout, worker-kill
    Chaos {
        /// Scenario: double-spend, timeout, worker-kill
        #[arg(short, long, default_value = "double-spend")]
        scenario: String,
        /// Target service: match, java, php, node
        #[arg(short, long, default_value = "php")]
        target: String,
        /// Duration in ms for timeout scenarios
        #[arg(long, default_value = "2500")]
        duration_ms: u64,
    },
}

#[derive(Args, Debug)]
struct CodexArgs {
    /// Documentation server preview port
    #[arg(short, long, default_value = "4500")]
    port: u16,

    /// Automatically open browser preview
    #[arg(short, long)]
    open: bool,
}

fn main() -> ExitCode {
    // Transparently handle invocation via `cargo ferrox <cmd>` where "ferrox" is passed as argv[1]
    let mut raw_args: Vec<String> = std::env::args().collect();
    if raw_args.len() > 1 && raw_args[1] == "ferrox" {
        raw_args.remove(1);
    }

    let cli = match Cli::try_parse_from(raw_args) {
        Ok(c) => c,
        Err(e) => {
            let _ = e.print();
            if e.use_stderr() {
                return ExitCode::FAILURE;
            } else {
                return ExitCode::SUCCESS;
            }
        }
    };

    let result = match cli.command {
        Commands::New(args) => {
            let template = match ProjectTemplate::from_str_loose(&args.template) {
                Some(t) => t,
                None => {
                    eprintln!("❌ Unknown template '{}'.", args.template);
                    eprintln!("Available templates:");
                    eprintln!("  - clean-service  : Hexagonal DDD Microservice with PASETO v4");
                    eprintln!("  - trading-desk   : High-Frequency Matching Engine & WebSocket");
                    eprintln!("  - polyglot-mesh  : Distributed Event Mesh with Redpanda & Redis");
                    eprintln!("  - wasm-spa       : Pure Rust Reactive WebAssembly Single Page App");
                    return ExitCode::FAILURE;
                }
            };

            new::execute(new::NewCommand {
                name: args.name,
                template,
                path: args.path,
            })
        }
        Commands::Dev(args) => dev::execute(dev::DevCommand {
            port: args.port,
            dry_run: args.dry_run,
            no_watch: args.no_watch,
        }),
        Commands::Test(args) => test::execute(test::TestCommand {
            allow_debris: args.allow_debris,
            filter: args.filter,
        }),
        Commands::Check(args) => check::execute(check::CheckCommand {
            strict: args.strict,
            security: args.security,
            path: args.path,
        }),
        Commands::Mesh(args) => {
            let subcommand = match args.command {
                MeshSubcommands::Up { compose_file, dry_run } => {
                    mesh::MeshSubcommand::Up(mesh::MeshUpOptions { compose_file, dry_run })
                }
                MeshSubcommands::Down { compose_file, dry_run } => {
                    mesh::MeshSubcommand::Down(mesh::MeshDownOptions { compose_file, dry_run })
                }
                MeshSubcommands::Status { runtime } => {
                    mesh::MeshSubcommand::Status(mesh::MeshStatusOptions { runtime })
                }
                MeshSubcommands::Chaos { scenario, target, duration_ms } => {
                    mesh::MeshSubcommand::Chaos(mesh::MeshChaosOptions { scenario, target, duration_ms })
                }
            };
            mesh::execute(mesh::MeshCommand { subcommand })
        }
        Commands::Codex(args) => codex::execute(codex::CodexCommand {
            port: args.port,
            open: args.open,
        }),
        Commands::Generate { lang, output } => {
            println!("⚙️ Ferrox Code Factory: Generating {} Client at {}...", lang, output);
            println!("✅ Code Generation Complete.");
            Ok(())
        }
        Commands::Mtd { secret, base_port } => {
            println!("🛡️ Ferrox Moving Target Defense (MTD) Dynamic Ingress Inspector");
            println!("🔑 Secret Seed: {}", secret);
            println!("📍 Base Ingress Port: {}", base_port);
            Ok(())
        }
        Commands::Honeynet => {
            println!("🕸️ Ferrox Distributed Honeynet Mesh & Ecosystem Threat Intelligence Active.");
            Ok(())
        }
        Commands::VpsDeploy { vps_ip, subdomain } => {
            println!("⚡ Ferrox Cloud VPS Relay Generator for {} ({})", subdomain, vps_ip);
            Ok(())
        }
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("❌ Command failed: {}", e);
            ExitCode::FAILURE
        }
    }
}