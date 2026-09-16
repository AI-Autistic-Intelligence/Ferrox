use reqwest::Client;
use crate::{Finding, Severity, WstgCategory};

/// Hardening Audit Suite for Environment Configurations, Secrets, and Exposure Safeguards
pub async fn run_config_hardening_audits(client: &Client, target_url: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    // 1. Audit for Exposed Sensitive Environment Variables or Dev Route Misconfigurations
    let dev_routes = [
        "/api/dev",
        "/admin/debug",
        "/swagger-ui",
        "/.env",
        "/config.json",
        "/metrics",
    ];

    for route in &dev_routes {
        let test_url = format!("{}{}", target_url, route);
        if let Ok(resp) = client.get(&test_url).send().await {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();

            let exposed = status.is_success() && (body.contains("DB_PASSWORD") || body.contains("SECRET") || body.contains("POSTGRES"));

            if exposed || (status.is_success() && route.contains(".env")) {
                findings.push(Finding {
                    wstg_id: "HARDENING-CONF-001".to_string(),
                    category: WstgCategory::ConfigurationManagement,
                    title: format!("Exposed Dev Route / File: {}", route),
                    severity: Severity::Critical,
                    passed: false,
                    description: format!("Route {} is accessible in public environment! Response status: {}", route, status),
                    practical_risk: "Publicly accessible environment files or debugging routes leak secrets (API keys, database credentials), giving total system control to unauthorized actors.".to_string(),
                    remediation: "Disable dev endpoints and block access to configuration files in production builds.".to_string(),
                });
            }
        }
    }

    // 2. Production Environment Mode Safeguard
    let is_prod = std::env::var("FERROX_ENV").unwrap_or_default() == "production";
    let is_dev_db = std::env::var("DATABASE_URL").map_or(false, |url| url.contains("localhost") || url.contains("127.0.0.1"));

    findings.push(Finding {
        wstg_id: "HARDENING-CONF-002".to_string(),
        category: WstgCategory::ConfigurationManagement,
        title: "Production Database Isolation Guard".to_string(),
        severity: Severity::High,
        passed: !(is_prod && is_dev_db),
        description: if is_prod && is_dev_db {
            "CRITICAL: Production environment is pointing to a local / localhost development database!".to_string()
        } else {
            "Database connection parameters match expected environment isolation rules.".to_string()
        },
        practical_risk: "Production services connecting to dev databases risk data contamination, development secrets leak, and unintended schema drops during active deployment.".to_string(),
        remediation: "Ensure production deployments point to dedicated isolated database instances.".to_string(),
    });

    findings
}
