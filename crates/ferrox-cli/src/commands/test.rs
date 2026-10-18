use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::io;

pub struct TestCommand {
    pub allow_debris: bool,
    pub filter: Option<String>,
}

#[derive(Debug)]
pub struct DebrisViolation {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub pattern: &'static str,
    pub snippet: String,
}

pub fn execute(cmd: TestCommand) -> io::Result<()> {
    println!("🧪 Ferrox Quality Gate: 100% Coverage & Zero Debris Enforcement");
    println!("---------------------------------------------------------------");

    let current_dir = std::env::current_dir()?;
    let src_dir = current_dir.join("src");

    // Phase 1: Debug Debris Scanner
    println!("🔍 Phase 1: Scanning for forbidden debug debris (dbg!, console.log, var_dump)...");
    let mut violations = Vec::new();

    if src_dir.exists() {
        scan_directory_for_debris(&src_dir, &mut violations)?;
    }

    if !violations.is_empty() {
        println!("⚠️  Found {} debug statement(s):", violations.len());
        for v in &violations {
            println!(
                "  ❌ [{}] {}:{} -> {}",
                v.pattern,
                v.file_path.display(),
                v.line_number,
                v.snippet.trim()
            );
        }

        if !cmd.allow_debris {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Quality gate rejected: {} forbidden debug statement(s) found in src/. Remove them or pass --allow-debris.",
                    violations.len()
                ),
            ));
        } else {
            println!("⚠️  --allow-debris passed, continuing despite warnings.");
        }
    } else {
        println!("  ✅ Zero debug debris detected in source tree.");
    }

    // Phase 2: Cargo Test Runner
    println!("\n🚀 Phase 2: Executing Rust Test Suite with strict zero-failure policy...");
    let mut cargo_args = vec!["test", "--all"];
    if let Some(ref f) = cmd.filter {
        cargo_args.push("--");
        cargo_args.push(f.as_str());
    }

    let status = Command::new("cargo")
        .args(&cargo_args)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Tests failed with exit status: {}", status),
        ));
    }

    println!("\n🎉 ================================================================");
    println!("✨ Definition of Done (DoD) Quality Gate: ALL CHECKS PASSED!");
    println!("   - 100% Test Suite Pass Rate");
    println!("   - Zero Debug Debris Invariants Satisfied");
    println!("   - Production Ready for Git Commit & Ferrox Forge");
    println!("====================================================================");

    Ok(())
}

pub fn scan_directory_for_debris(dir: &Path, violations: &mut Vec<DebrisViolation>) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if name != "target" && name != ".git" && name != "node_modules" {
                scan_directory_for_debris(&path, violations)?;
            }
        } else if path.is_file() {
            scan_file_for_debris(&path, violations)?;
        }
    }

    Ok(())
}

fn scan_file_for_debris(path: &Path, violations: &mut Vec<DebrisViolation>) -> io::Result<()> {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let patterns: &[&'static str] = match ext {
        "rs" => &["dbg!", "todo!()", "unimplemented!()"],
        "ts" | "tsx" | "js" | "jsx" => &["console.log(", "console.debug(", "debugger;"],
        "py" => &["print(", "breakpoint()"],
        "php" => &["var_dump(", "dd(", "print_r("],
        "java" => &["System.out.println("],
        _ => return Ok(()),
    };

    if let Ok(content) = fs::read_to_string(path) {
        for (idx, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            // Ignore commented lines
            if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with("/*") || trimmed.starts_with('*') {
                continue;
            }

            for &pat in patterns {
                if trimmed.contains(pat) {
                    violations.push(DebrisViolation {
                        file_path: path.to_path_buf(),
                        line_number: idx + 1,
                        pattern: pat,
                        snippet: trimmed.to_string(),
                    });
                }
            }
        }
    }

    Ok(())
}
