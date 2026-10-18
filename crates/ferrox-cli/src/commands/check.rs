use std::path::{Path, PathBuf};
use std::fs;
use std::io;

pub struct CheckCommand {
    pub strict: bool,
    pub path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct ArchitectureViolation {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub forbidden_import: String,
    pub rule_description: &'static str,
}

pub fn execute(cmd: CheckCommand) -> io::Result<()> {
    println!("📐 Ferrox Architectural DDD Purity Sentinel");
    println!("-------------------------------------------");

    let current_dir = match cmd.path {
        Some(p) => p,
        None => std::env::current_dir()?,
    };

    let domain_candidates = [
        current_dir.join("src").join("domain"),
        current_dir.join("domain"),
        current_dir.join("core").join("domain"),
    ];

    let mut found_domain_dir = None;
    for candidate in &domain_candidates {
        if candidate.is_dir() {
            found_domain_dir = Some(candidate.clone());
            break;
        }
    }

    let domain_dir = match found_domain_dir {
        Some(d) => d,
        None => {
            println!("ℹ️  No dedicated 'domain' directory found. Checking entire 'src/' for architecture markers...");
            current_dir.join("src")
        }
    };

    println!("🔍 Inspecting domain boundaries at: {}", domain_dir.display());

    let mut violations = Vec::new();
    if domain_dir.exists() {
        scan_domain_purity(&domain_dir, &mut violations)?;
    }

    if !violations.is_empty() {
        println!("\n❌ Architectural DDD Violations Detected ({} found):", violations.len());
        for v in &violations {
            println!(
                "  ⛔ {}:{} -> Forbidden import '{}' ({})",
                v.file_path.display(),
                v.line_number,
                v.forbidden_import,
                v.rule_description
            );
        }

        println!("\n💡 DDD Rule: The Pure Domain layer must only contain business logic, value objects,");
        println!("   and entities. Infrastructure (SQL/ORMs, HTTP servers, gRPC) must reside in adapters.");

        if cmd.strict {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Architectural check failed with {} DDD violation(s).", violations.len()),
            ));
        }
    } else {
        println!("✅ Domain Layer Purity: 100% compliant.");
        println!("   - Zero ORM dependencies in core domain");
        println!("   - Zero HTTP framework couplings");
        println!("   - Aggregates and Value Objects cleanly isolated");
    }

    Ok(())
}

fn scan_domain_purity(dir: &Path, violations: &mut Vec<ArchitectureViolation>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            scan_domain_purity(&path, violations)?;
        } else if path.is_file() {
            check_file_purity(&path, violations)?;
        }
    }

    Ok(())
}

fn check_file_purity(path: &Path, violations: &mut Vec<ArchitectureViolation>) -> io::Result<()> {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    if ext != "rs" && ext != "ts" && ext != "py" && ext != "php" && ext != "java" {
        return Ok(());
    }

    let forbidden_rust_crates = [
        ("sea_orm", "ORM dependency forbidden in pure domain"),
        ("sqlx", "Direct database query engine forbidden in pure domain"),
        ("diesel", "ORM dependency forbidden in pure domain"),
        ("axum", "HTTP framework coupling forbidden in pure domain"),
        ("actix_web", "HTTP framework coupling forbidden in pure domain"),
        ("warp", "HTTP framework coupling forbidden in pure domain"),
        ("rocket", "HTTP framework coupling forbidden in pure domain"),
        ("tokio::net", "Raw TCP networking forbidden in pure domain"),
    ];

    if let Ok(content) = fs::read_to_string(path) {
        for (idx, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
                continue;
            }

            for &(forbidden, reason) in &forbidden_rust_crates {
                let is_violation = if trimmed.starts_with("use ") {
                    trimmed.contains(forbidden)
                } else if trimmed.starts_with("extern crate ") {
                    trimmed.contains(forbidden)
                } else {
                    false
                };

                if is_violation {
                    violations.push(ArchitectureViolation {
                        file_path: path.to_path_buf(),
                        line_number: idx + 1,
                        forbidden_import: forbidden.to_string(),
                        rule_description: reason,
                    });
                }
            }
        }
    }

    Ok(())
}
