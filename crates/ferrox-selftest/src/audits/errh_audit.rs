use reqwest::Client;
use crate::{Finding, Severity, WstgCategory};

/// Runs all WSTG-ERRH (Error Handling) domain audit tests
pub async fn run_errh_audits(client: &Client, target_url: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    // 1. WSTG-ERRH-001: Information Leakage in 404 Route Errors
    let non_existent_url = format!("{}/api/v1/non_existent_wstg_test_path_9999", target_url);
    if let Ok(resp) = client.get(&non_existent_url).send().await {
        let body = resp.text().await.unwrap_or_default();
        let leaks_stack_trace = body.contains("src/") 
            || body.contains("stack backtrace") 
            || body.contains("thread 'main' panicked") 
            || body.contains("Cargo") 
            || body.contains("C:\\Users\\");

        findings.push(Finding {
            wstg_id: "WSTG-ERRH-001".to_string(),
            category: WstgCategory::ErrorHandling,
            title: "Stack Trace & Debug Leakage in 404 Handlers".to_string(),
            severity: Severity::High,
            passed: !leaks_stack_trace,
            description: if leaks_stack_trace {
                "404 Not Found response leaks internal system file paths or Rust stack trace details!".to_string()
            } else {
                "404 error handler returns clean, sanitized response without system disclosure.".to_string()
            },
            practical_risk: "Leaking filesystem paths, source file structures, and thread panics helps adversaries map internal server code structures and identify exploitable call stacks.".to_string(),
            remediation: "Ensure `AppError` in Ferrox maps unhandled errors to structured JSON without backtraces.".to_string(),
        });
    }

    // 2. WSTG-ERRH-002: Information Leakage on Malformed Content-Type / 400 Bad Requests
    let malformed_req = client.post(format!("{}/api/v1/auth/login", target_url))
        .header("Content-Type", "application/json")
        .body("{ malformed json body syntax error ... ")
        .send()
        .await;

    if let Ok(resp) = malformed_req {
        let body = resp.text().await.unwrap_or_default();
        let leaks_parser_internal = body.contains("serde_json::error") || body.contains("panic");

        findings.push(Finding {
            wstg_id: "WSTG-ERRH-002".to_string(),
            category: WstgCategory::ErrorHandling,
            title: "Detailed Exception & Schema Disclosure on Malformed Input".to_string(),
            severity: Severity::Medium,
            passed: !leaks_parser_internal,
            description: if leaks_parser_internal {
                "Malformed request body response exposes internal JSON parser implementation details.".to_string()
            } else {
                "Malformed input error handler presents sanitized validation message.".to_string()
            },
            practical_risk: "Exposing internal deserialization exceptions aids attackers in identifying backend libraries, versions, and schema expectations for crafting targeted payloads.".to_string(),
            remediation: "Use generic validation error messages in `AppError::ValidationError`.".to_string(),
        });
    }

    findings
}
