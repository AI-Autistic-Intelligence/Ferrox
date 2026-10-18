use std::path::{Path, PathBuf};
use std::io;
use crate::templates::ProjectTemplate;

pub struct NewCommand {
    pub name: String,
    pub template: ProjectTemplate,
    pub path: Option<PathBuf>,
}

pub fn execute(cmd: NewCommand) -> io::Result<()> {
    let target_dir = match cmd.path {
        Some(p) => p,
        None => Path::new(".").join(&cmd.name),
    };

    if target_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Destination path '{}' already exists", target_dir.display()),
        ));
    }

    println!("⚡ Scaffolding new Ferrox project: '{}'", cmd.name);
    println!("📦 Template: {} ({})", cmd.template.as_str(), cmd.template.description());

    cmd.template.scaffold(&cmd.name, &target_dir)?;

    println!("✨ Successfully scaffolded '{}' at '{}'", cmd.name, target_dir.display());
    println!("\nNext steps:");
    println!("  cd {}", cmd.name);
    println!("  cargo ferrox dev    # Start development environment");
    println!("  cargo ferrox test   # Run TDD test suite with 100% coverage check");
    println!("  cargo ferrox check  # Audit architectural DDD purity");

    Ok(())
}
