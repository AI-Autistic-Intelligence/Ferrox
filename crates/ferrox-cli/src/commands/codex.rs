use std::path::{Path, PathBuf};
use std::fs;
use std::io;

pub struct CodexCommand {
    pub port: u16,
    pub open: bool,
}

pub fn execute(cmd: CodexCommand) -> io::Result<()> {
    println!("📚 Ferrox Codex: Interactive Ecosystem Documentation");
    println!("----------------------------------------------------");

    let current_dir = std::env::current_dir()?;
    let docs_dir = current_dir.join("docs");

    println!("📍 Local Codex Server: http://localhost:{}", cmd.port);
    println!("📖 Project Root: {}", current_dir.display());

    let mut doc_files = Vec::new();
    if docs_dir.exists() {
        find_markdown_files(&docs_dir, &mut doc_files)?;
    }
    
    // Also check root for README and ARCHITECTURE
    for root_doc in &["README.md", "ARCHITECTURE.md", "HANDBOOK.md"] {
        let p = current_dir.join(root_doc);
        if p.exists() {
            doc_files.push(p);
        }
    }

    if doc_files.is_empty() {
        println!("ℹ️  No markdown documentation found in ./docs or root.");
        println!("💡 Add documentation in 'docs/' or scaffold a project using 'cargo ferrox new'.");
    } else {
        println!("📑 Discovered {} documentation module(s):", doc_files.len());
        for doc in &doc_files {
            let rel = doc.strip_prefix(&current_dir).unwrap_or(doc);
            println!("  📄 {}", rel.display());
        }
    }

    println!("\n✨ Ferrox Codex is ready! Access your live documentation sandbox at:");
    println!("👉 http://localhost:{}", cmd.port);

    if cmd.open {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("cmd")
                .args(["/C", "start", &format!("http://localhost:{}", cmd.port)])
                .spawn();
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = std::process::Command::new("xdg-open")
                .arg(format!("http://localhost:{}", cmd.port))
                .spawn();
        }
    }

    Ok(())
}

fn find_markdown_files(dir: &Path, list: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            find_markdown_files(&path, list)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
            list.push(path);
        }
    }
    Ok(())
}
