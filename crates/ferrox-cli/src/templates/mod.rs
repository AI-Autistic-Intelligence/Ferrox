pub mod clean_service;
pub mod trading_desk;
pub mod polyglot_mesh;
pub mod wasm_spa;

use std::path::Path;
use std::fs;
use std::io::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectTemplate {
    CleanService,
    TradingDesk,
    PolyglotMesh,
    WasmSpa,
}

impl ProjectTemplate {
    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.to_lowercase().replace('_', "-").as_str() {
            "clean-service" | "clean" | "service" | "ddd" => Some(Self::CleanService),
            "trading-desk" | "trading" | "match" => Some(Self::TradingDesk),
            "polyglot-mesh" | "mesh" | "polyglot" => Some(Self::PolyglotMesh),
            "wasm-spa" | "wasm" | "spa" | "frontend" => Some(Self::WasmSpa),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CleanService => "clean-service",
            Self::TradingDesk => "trading-desk",
            Self::PolyglotMesh => "polyglot-mesh",
            Self::WasmSpa => "wasm-spa",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::CleanService => "Hexagonal DDD Microservice with PASETO v4 & Invariant Gates",
            Self::TradingDesk => "High-Frequency Trading Desk & Sub-Microsecond Matching Engine",
            Self::PolyglotMesh => "Distributed Polyglot Event Mesh with Redpanda & Redis Mutex",
            Self::WasmSpa => "Pure Rust Reactive WebAssembly Single Page App (60 FPS Vector SVG)",
        }
    }

    pub fn scaffold(&self, project_name: &str, target_dir: &Path) -> io::Result<()> {
        match self {
            Self::CleanService => clean_service::scaffold(project_name, target_dir),
            Self::TradingDesk => trading_desk::scaffold(project_name, target_dir),
            Self::PolyglotMesh => polyglot_mesh::scaffold(project_name, target_dir),
            Self::WasmSpa => wasm_spa::scaffold(project_name, target_dir),
        }
    }
}

pub(crate) fn write_file(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
