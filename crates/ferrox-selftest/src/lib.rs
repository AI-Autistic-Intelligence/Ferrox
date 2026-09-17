//! # Ferrox Self-Test (`ferrox-selftest`)
//!
//! Automated OWASP Web Security Testing Guide (WSTG) domain security auditor for Ferrox Enterprise backends.
//!
//! `ferrox-selftest` provides runtime diagnostic testing againstOWASP WSTG security controls,
//! including security header checks, authentication policy verification, cookie session flags,
//! error information leakage analysis, and CORS configurations.

use serde::{Deserialize, Serialize};

pub mod audits;
pub mod reporter;
pub mod vocabulary;
pub mod pipeline;
pub mod continuous_benchmark;
pub mod kali_audit_runner;

pub use continuous_benchmark::{BenchmarkReport, ContinuousSecurityBenchmark, InnovationCheckResult};
pub use kali_audit_runner::{KaliAuditFinding, KaliAuditReport, KaliAuditRunnerEngine, KaliTool};

/// Severity level for OWASP WSTG findings
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Low => write!(f, "LOW"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::High => write!(f, "HIGH"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// OWASP WSTG Category Code
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WstgCategory {
    /// Information Gathering (WSTG-INFO)
    InformationGathering,
    /// Configuration & Deployment Management (WSTG-CONF)
    ConfigurationManagement,
    /// Identity Management (WSTG-IDNT)
    IdentityManagement,
    /// Authentication Testing (WSTG-ATHN)
    Authentication,
    /// Authorization Testing (WSTG-ATHZ)
    Authorization,
    /// Session Management (WSTG-SESS)
    SessionManagement,
    /// Input Validation (WSTG-INPV)
    InputValidation,
    /// Error Handling (WSTG-ERRH)
    ErrorHandling,
    /// Cryptography (WSTG-CRYP)
    Cryptography,
    /// Business Logic (WSTG-BUSL)
    BusinessLogic,
    /// Client-Side Testing (WSTG-CLNT)
    ClientSide,
    /// API Testing (WSTG-APIT)
    ApiTesting,
}

impl WstgCategory {
    pub fn code(&self) -> &'static str {
        match self {
            WstgCategory::InformationGathering => "WSTG-INFO",
            WstgCategory::ConfigurationManagement => "WSTG-CONF",
            WstgCategory::IdentityManagement => "WSTG-IDNT",
            WstgCategory::Authentication => "WSTG-ATHN",
            WstgCategory::Authorization => "WSTG-ATHZ",
            WstgCategory::SessionManagement => "WSTG-SESS",
            WstgCategory::InputValidation => "WSTG-INPV",
            WstgCategory::ErrorHandling => "WSTG-ERRH",
            WstgCategory::Cryptography => "WSTG-CRYP",
            WstgCategory::BusinessLogic => "WSTG-BUSL",
            WstgCategory::ClientSide => "WSTG-CLNT",
            WstgCategory::ApiTesting => "WSTG-APIT",
        }
    }
}

/// An individual security finding produced by a WSTG audit test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// OWASP WSTG Test Reference ID (e.g., "WSTG-CONF-001")
    pub wstg_id: String,
    /// Category of the security test
    pub category: WstgCategory,
    /// Test title
    pub title: String,
    /// Severity rating
    pub severity: Severity,
    /// Indicates if the test passed
    pub passed: bool,
    /// Technical description of finding or failure rationale
    pub description: String,
    /// Practical real-world business and operational risk impact
    pub practical_risk: String,
    /// Remediation recommendation according to OWASP guidelines
    pub remediation: String,
}

/// Configuration settings for running WSTG self-tests
#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Target base URL (e.g. "http://localhost:8080")
    pub target_url: String,
    /// Timeout per HTTP request in seconds
    pub timeout_secs: u64,
    /// Include verbose descriptions
    pub verbose: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            target_url: "http://127.0.0.1:8080".to_string(),
            timeout_secs: 5,
            verbose: false,
        }
    }
}

/// Consolidated audit report containing all test findings and summary metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub target_url: String,
    pub timestamp_rfc3339: String,
    pub total_tests: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub score_percentage: f64,
    pub findings: Vec<Finding>,
}

/// Primary Auditor runner
pub struct WstgAuditor {
    config: AuditConfig,
    client: reqwest::Client,
}

impl WstgAuditor {
    pub fn new(config: AuditConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    /// Executes all OWASP WSTG audit suites against the configured target domain
    pub async fn run_all(&self) -> AuditReport {
        let mut findings = Vec::new();

        // 1. Configuration & Deployment Audits (WSTG-CONF)
        findings.extend(audits::config_audit::run_config_audits(&self.client, &self.config.target_url).await);

        // 2. Authentication Audits (WSTG-ATHN)
        findings.extend(audits::authn_audit::run_authn_audits(&self.client, &self.config.target_url).await);

        // 3. Session Management Audits (WSTG-SESS)
        findings.extend(audits::sess_audit::run_sess_audits(&self.client, &self.config.target_url).await);

        // 4. Error Handling Audits (WSTG-ERRH)
        findings.extend(audits::errh_audit::run_errh_audits(&self.client, &self.config.target_url).await);

        // 5. Configuration Hardening & Dev Exposure Audits
        findings.extend(audits::hardening_audit::run_config_hardening_audits(&self.client, &self.config.target_url).await);

        let total_tests = findings.len();
        let passed_count = findings.iter().filter(|f| f.passed).count();
        let failed_count = total_tests - passed_count;
        let score_percentage = if total_tests > 0 {
            (passed_count as f64 / total_tests as f64) * 100.0
        } else {
            100.0
        };

        AuditReport {
            target_url: self.config.target_url.clone(),
            timestamp_rfc3339: chrono::Utc::now().to_rfc3339(),
            total_tests,
            passed_count,
            failed_count,
            score_percentage,
            findings,
        }
    }
}
