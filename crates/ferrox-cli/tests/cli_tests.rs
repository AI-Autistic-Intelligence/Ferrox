use cargo_ferrox::commands::{check, codex, mesh, new, test};
use cargo_ferrox::templates::ProjectTemplate;
use std::fs;

#[test]
fn test_template_resolution() {
    assert_eq!(ProjectTemplate::from_str_loose("clean-service"), Some(ProjectTemplate::CleanService));
    assert_eq!(ProjectTemplate::from_str_loose("clean"), Some(ProjectTemplate::CleanService));
    assert_eq!(ProjectTemplate::from_str_loose("trading-desk"), Some(ProjectTemplate::TradingDesk));
    assert_eq!(ProjectTemplate::from_str_loose("polyglot-mesh"), Some(ProjectTemplate::PolyglotMesh));
    assert_eq!(ProjectTemplate::from_str_loose("wasm-spa"), Some(ProjectTemplate::WasmSpa));
    assert_eq!(ProjectTemplate::from_str_loose("unknown-foo"), None);

    assert_eq!(ProjectTemplate::CleanService.as_str(), "clean-service");
    assert_eq!(ProjectTemplate::TradingDesk.as_str(), "trading-desk");
    assert_eq!(ProjectTemplate::PolyglotMesh.as_str(), "polyglot-mesh");
    assert_eq!(ProjectTemplate::WasmSpa.as_str(), "wasm-spa");
}

#[test]
fn test_scaffold_clean_service() {
    let tmp = std::env::temp_dir().join("ferrox_test_clean_svc");
    let _ = fs::remove_dir_all(&tmp);

    let res = new::execute(new::NewCommand {
        name: "test-clean-svc".to_string(),
        template: ProjectTemplate::CleanService,
        path: Some(tmp.clone()),
    });
    assert!(res.is_ok());

    assert!(tmp.join("Ferrox.toml").exists());
    assert!(tmp.join("Cargo.toml").exists());
    assert!(tmp.join(".coderabbit.yaml").exists());
    assert!(tmp.join(".github/workflows/ci.yml").exists());
    assert!(tmp.join("Dockerfile").exists());
    assert!(tmp.join("docs/ARCHITECTURE.md").exists());
    assert!(tmp.join("src/domain/mod.rs").exists());
    assert!(tmp.join("src/application/mod.rs").exists());
    assert!(tmp.join("src/main.rs").exists());

    let ferrox_toml = fs::read_to_string(tmp.join("Ferrox.toml")).unwrap();
    assert!(ferrox_toml.contains("template = \"clean-service\""));

    // Check DDD purity check on scaffolded clean service
    let check_res = check::execute(check::CheckCommand {
        strict: true,
        security: false,
        path: Some(tmp.clone()),
    });
    assert!(check_res.is_ok());

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_scaffold_trading_desk() {
    let tmp = std::env::temp_dir().join("ferrox_test_trading");
    let _ = fs::remove_dir_all(&tmp);

    let res = new::execute(new::NewCommand {
        name: "test-trading".to_string(),
        template: ProjectTemplate::TradingDesk,
        path: Some(tmp.clone()),
    });
    assert!(res.is_ok());

    assert!(tmp.join("Ferrox.toml").exists());
    assert!(tmp.join("Cargo.toml").exists());
    assert!(tmp.join("src/orderbook.rs").exists());
    assert!(tmp.join("src/lib.rs").exists());
    assert!(tmp.join("src/main.rs").exists());

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_scaffold_polyglot_mesh() {
    let tmp = std::env::temp_dir().join("ferrox_test_mesh");
    let _ = fs::remove_dir_all(&tmp);

    let res = new::execute(new::NewCommand {
        name: "test-mesh".to_string(),
        template: ProjectTemplate::PolyglotMesh,
        path: Some(tmp.clone()),
    });
    assert!(res.is_ok());

    assert!(tmp.join("Ferrox.toml").exists());
    assert!(tmp.join("docker-compose.yml").exists());
    assert!(tmp.join("src/cloudevent.rs").exists());
    assert!(tmp.join("src/main.rs").exists());

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_scaffold_wasm_spa() {
    let tmp = std::env::temp_dir().join("ferrox_test_wasm");
    let _ = fs::remove_dir_all(&tmp);

    let res = new::execute(new::NewCommand {
        name: "test-wasm".to_string(),
        template: ProjectTemplate::WasmSpa,
        path: Some(tmp.clone()),
    });
    assert!(res.is_ok());

    assert!(tmp.join("Ferrox.toml").exists());
    assert!(tmp.join("Trunk.toml").exists());
    assert!(tmp.join("index.html").exists());
    assert!(tmp.join("src/state.rs").exists());
    assert!(tmp.join("src/lib.rs").exists());

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_debris_scanner() {
    let tmp = std::env::temp_dir().join("ferrox_test_debris");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    let clean_file = tmp.join("clean.rs");
    fs::write(&clean_file, "pub fn compute() -> i32 { 42 }\n").unwrap();

    let mut violations = Vec::new();
    test::scan_directory_for_debris(&tmp, &mut violations).unwrap();
    assert_eq!(violations.len(), 0);

    let dirty_file = tmp.join("dirty.rs");
    fs::write(&dirty_file, "pub fn compute() -> i32 { dbg!(42); 42 }\n").unwrap();

    violations.clear();
    test::scan_directory_for_debris(&tmp, &mut violations).unwrap();
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].pattern, "dbg!");

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_codex_command() {
    let res = codex::execute(codex::CodexCommand {
        port: 4501,
        open: false,
    });
    assert!(res.is_ok());
}

#[test]
fn test_security_sentinel_audit() {
    let tmp = std::env::temp_dir().join("ferrox_test_security");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(tmp.join("src")).unwrap();

    let clean_code = r#"
        pub fn authenticate_user(token: &str) -> bool {
            token.starts_with("v4.local.")
        }
    "#;
    fs::write(tmp.join("src/auth.rs"), clean_code).unwrap();

    // Verify clean pass with security flag
    let res = check::execute(check::CheckCommand {
        strict: true,
        security: true,
        path: Some(tmp.clone()),
    });
    assert!(res.is_ok());

    // Verify Shannon entropy calculation
    let low_entropy = check::calculate_shannon_entropy("aaaaaaaaaaaaaaaa");
    assert!(low_entropy < 1.0);

    let high_entropy = check::calculate_shannon_entropy("4k7j9xL2pQ8mZ0wR5vY1bC3nF6tH");
    assert!(high_entropy > 4.0);

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_mesh_commands_lifecycle() {
    // 1. Mesh Up (dry-run)
    let up_res = mesh::execute(mesh::MeshCommand {
        subcommand: mesh::MeshSubcommand::Up(mesh::MeshUpOptions {
            compose_file: None,
            dry_run: true,
            standalone: false,
        }),
    });
    assert!(up_res.is_ok());

    // 1b. Mesh Up Standalone (dry-run)
    let up_standalone = mesh::execute(mesh::MeshCommand {
        subcommand: mesh::MeshSubcommand::Up(mesh::MeshUpOptions {
            compose_file: None,
            dry_run: true,
            standalone: true,
        }),
    });
    assert!(up_standalone.is_ok());

    // 2. Mesh Status
    let status_res = mesh::execute(mesh::MeshCommand {
        subcommand: mesh::MeshSubcommand::Status(mesh::MeshStatusOptions {
            runtime: Some("match".to_string()),
        }),
    });
    assert!(status_res.is_ok());

    // 3. Mesh Chaos (Double Spend)
    let chaos_double_spend = mesh::execute(mesh::MeshCommand {
        subcommand: mesh::MeshSubcommand::Chaos(mesh::MeshChaosOptions {
            scenario: "double-spend".to_string(),
            target: "php".to_string(),
            duration_ms: 2500,
        }),
    });
    assert!(chaos_double_spend.is_ok());

    // 4. Mesh Chaos (Timeout)
    let chaos_timeout = mesh::execute(mesh::MeshCommand {
        subcommand: mesh::MeshSubcommand::Chaos(mesh::MeshChaosOptions {
            scenario: "timeout".to_string(),
            target: "java".to_string(),
            duration_ms: 1500,
        }),
    });
    assert!(chaos_timeout.is_ok());

    // 5. Mesh Chaos (Worker Kill)
    let chaos_worker = mesh::execute(mesh::MeshCommand {
        subcommand: mesh::MeshSubcommand::Chaos(mesh::MeshChaosOptions {
            scenario: "worker-kill".to_string(),
            target: "php".to_string(),
            duration_ms: 500,
        }),
    });
    assert!(chaos_worker.is_ok());

    // 6. Mesh Publish (dry-run all)
    let publish_res = mesh::execute(mesh::MeshCommand {
        subcommand: mesh::MeshSubcommand::Publish(mesh::MeshPublishOptions {
            registry: "ghcr.io/ai-autistic-intelligence".to_string(),
            tag: "latest".to_string(),
            push: true,
            dry_run: true,
            no_cache: false,
            service: None,
        }),
    });
    assert!(publish_res.is_ok());

    // 6b. Mesh Publish (dry-run filtered service)
    let publish_front = mesh::execute(mesh::MeshCommand {
        subcommand: mesh::MeshSubcommand::Publish(mesh::MeshPublishOptions {
            registry: "ghcr.io/ai-autistic-intelligence".to_string(),
            tag: "v1.0.0".to_string(),
            push: false,
            dry_run: true,
            no_cache: true,
            service: Some("front".to_string()),
        }),
    });
    assert!(publish_front.is_ok());

    // 7. Mesh Down (dry-run)
    let down_res = mesh::execute(mesh::MeshCommand {
        subcommand: mesh::MeshSubcommand::Down(mesh::MeshDownOptions {
            compose_file: None,
            dry_run: true,
            standalone: true,
        }),
    });
    assert!(down_res.is_ok());
}

