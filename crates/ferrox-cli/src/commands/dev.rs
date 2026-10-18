use std::process::Command;
use std::fs;
use std::io;

pub struct DevCommand {
    pub port: Option<u16>,
    pub dry_run: bool,
    pub no_watch: bool,
}

pub fn execute(cmd: DevCommand) -> io::Result<()> {
    println!("⚡ Ferrox Dev Orchestrator (Zero Cloud Costs)");
    println!("----------------------------------------------");

    let current_dir = std::env::current_dir()?;
    let ferrox_toml_path = current_dir.join("Ferrox.toml");
    let trunk_toml_path = current_dir.join("Trunk.toml");
    let cargo_toml_path = current_dir.join("Cargo.toml");

    let mut template_name = "generic-service".to_string();
    let mut project_name = "ferrox-service".to_string();

    if ferrox_toml_path.exists() {
        if let Ok(content) = fs::read_to_string(&ferrox_toml_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("template") {
                    if let Some(val) = trimmed.split('=').nth(1) {
                        template_name = val.trim().trim_matches('"').to_string();
                    }
                } else if trimmed.starts_with("name") {
                    if let Some(val) = trimmed.split('=').nth(1) {
                        project_name = val.trim().trim_matches('"').to_string();
                    }
                }
            }
        }
        println!("🔍 Detected Ferrox project: '{}' [Template: {}]", project_name, template_name);
    } else {
        println!("ℹ️  No Ferrox.toml found, checking project files...");
    }

    let is_wasm = template_name == "wasm-spa" || trunk_toml_path.exists();

    if is_wasm {
        let port = cmd.port.unwrap_or(4000);
        println!("🌐 WebAssembly Frontend Detected!");
        println!("📍 Local Dev Server Target: http://localhost:{}", port);
        println!("🚀 Hot-Reload Engine: Trunk / Wasm-Bindgen");
        println!();

        if cmd.dry_run {
            println!("⚙️  [Dry Run] Command to execute: trunk serve --port {}", port);
            return Ok(());
        }

        println!("⚡ Launching Trunk development server on port {}...", port);
        let status = Command::new("trunk")
            .args(["serve", "--port", &port.to_string()])
            .status();

        match status {
            Ok(s) if s.success() => Ok(()),
            Ok(s) => Err(io::Error::new(io::ErrorKind::Other, format!("Trunk exited with status {}", s))),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                println!("⚠️  'trunk' binary not found on PATH.");
                println!("💡 You can install it via: cargo install trunk --locked");
                println!("💡 Or run manually with your preferred HTTP static file server.");
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else if cargo_toml_path.exists() {
        println!("🦀 Rust Backend Service Detected!");
        println!("📍 Local Fast-Path Server: http://localhost:{}", cmd.port.unwrap_or(8086));
        println!("📡 WebSocket Stream: ws://localhost:{}/ws/stream", cmd.port.unwrap_or(8086));
        println!("📦 Local Redpanda Broker: localhost:19092");
        println!("🔑 Local Redis Lock: localhost:6379");
        println!();

        if cmd.dry_run {
            println!("⚙️  [Dry Run] Command to execute: cargo run");
            return Ok(());
        }

        let run_cmd = if !cmd.no_watch {
            // Check if cargo-watch exists, otherwise fallback to cargo run
            let which_watch = Command::new("cargo")
                .args(["watch", "--version"])
                .output();
            if let Ok(output) = which_watch {
                if output.status.success() {
                    vec!["watch", "-x", "run"]
                } else {
                    vec!["run"]
                }
            } else {
                vec!["run"]
            }
        } else {
            vec!["run"]
        };

        println!("⚡ Executing 'cargo {}'...", run_cmd.join(" "));
        let status = Command::new("cargo")
            .args(&run_cmd)
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, format!("cargo process exited with {}", status)))
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No Cargo.toml or Ferrox.toml found in current directory.",
        ))
    }
}
