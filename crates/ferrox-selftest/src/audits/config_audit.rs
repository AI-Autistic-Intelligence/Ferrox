use reqwest::Client;
use crate::{Finding, Severity, WstgCategory};

/// Runs all WSTG-CONF (Configuration & Deployment) domain audit tests
pub async fn run_config_audits(client: &Client, target_url: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    let resp = match client.get(target_url).send().await {
        Ok(res) => res,
        Err(e) => {
            findings.push(Finding {
                wstg_id: "WSTG-CONF-001".to_string(),
                category: WstgCategory::ConfigurationManagement,
                title: "Domain Availability & Connection Test".to_string(),
                severity: Severity::Critical,
                passed: false,
                description: format!("Failed to connect to target domain {}: {}", target_url, e),
                practical_risk: "Unavailability of application services leading to immediate business operation outage and unreachability.".to_string(),
                remediation: "Ensure the Ferrox Enterprise service is running and accessible.".to_string(),
            });
            return findings;
        }
    };

    let headers = resp.headers();

    // 1. WSTG-CONF-001: Security Headers - Strict-Transport-Security (HSTS)
    let hsts_present = headers.contains_key("strict-transport-security");
    findings.push(Finding {
        wstg_id: "WSTG-CONF-001.1".to_string(),
        category: WstgCategory::ConfigurationManagement,
        title: "HTTP Strict Transport Security (HSTS) Enforcement".to_string(),
        severity: Severity::High,
        passed: hsts_present,
        description: if hsts_present {
            format!("HSTS header found: {:?}", headers.get("strict-transport-security").unwrap())
        } else {
            "Missing Strict-Transport-Security header. Domain is vulnerable to SSL stripping and protocol downgrade attacks.".to_string()
        },
        practical_risk: "Enables Man-in-the-Middle (MitM) attackers on public/untrusted networks to intercept unencrypted HTTP traffic and harvest user session tokens.".to_string(),
        remediation: "Configure `Strict-Transport-Security: max-age=31536000; includeSubDomains; preload` in Ferrox middleware.".to_string(),
    });

    // 2. WSTG-CONF-001: Content-Security-Policy (CSP)
    let csp_present = headers.contains_key("content-security-policy");
    findings.push(Finding {
        wstg_id: "WSTG-CONF-001.2".to_string(),
        category: WstgCategory::ConfigurationManagement,
        title: "Content Security Policy (CSP)".to_string(),
        severity: Severity::High,
        passed: csp_present,
        description: if csp_present {
            format!("CSP header present: {:?}", headers.get("content-security-policy").unwrap())
        } else {
            "Missing Content-Security-Policy header. Domain lacks client-side XSS and resource injection protection.".to_string()
        },
        practical_risk: "Allows execution of malicious inline/third-party scripts (XSS), token theft, and unauthorized data exfiltration from client browsers.".to_string(),
        remediation: "Define a strict `Content-Security-Policy` header allowing trusted origins only.".to_string(),
    });

    // 3. WSTG-CONF-001: Clickjacking Defense (X-Frame-Options / frame-ancestors)
    let xfo_present = headers.contains_key("x-frame-options") || (csp_present && headers.get("content-security-policy").map_or(false, |v| v.to_str().unwrap_or("").contains("frame-ancestors")));
    findings.push(Finding {
        wstg_id: "WSTG-CONF-001.3".to_string(),
        category: WstgCategory::ConfigurationManagement,
        title: "Clickjacking Protection (X-Frame-Options)".to_string(),
        severity: Severity::Medium,
        passed: xfo_present,
        description: if xfo_present {
            "Frame embedding restrictions are configured.".to_string()
        } else {
            "Missing X-Frame-Options or CSP frame-ancestors directive. Domain may be embedded in malicious iFrames.".to_string()
        },
        practical_risk: "Attackers can embed the target application inside invisible iFrames on hostile websites to trick authenticated users into executing state-changing operations.".to_string(),
        remediation: "Set `X-Frame-Options: DENY` or `SAMEORIGIN` in Ferrox security middleware.".to_string(),
    });

    // 4. WSTG-CONF-001: MIME Sniffing Defense (X-Content-Type-Options)
    let xcto = headers.get("x-content-type-options").map(|v| v.to_str().unwrap_or(""));
    let xcto_passed = xcto == Some("nosniff");
    findings.push(Finding {
        wstg_id: "WSTG-CONF-001.4".to_string(),
        category: WstgCategory::ConfigurationManagement,
        title: "MIME-Type Sniffing Protection".to_string(),
        severity: Severity::Low,
        passed: xcto_passed,
        description: if xcto_passed {
            "X-Content-Type-Options: nosniff correctly enforced.".to_string()
        } else {
            "Missing or invalid X-Content-Type-Options header. Browsers may sniff response content types.".to_string()
        },
        practical_risk: "Browsers may treat non-executable files (e.g. uploaded images/text) as HTML/JS, introducing secondary XSS vulnerabilities.".to_string(),
        remediation: "Set `X-Content-Type-Options: nosniff` header.".to_string(),
    });

    // 5. WSTG-CONF-002: Information Disclosure via Server Header
    let server_header = headers.get("server").and_then(|v| v.to_str().ok());
    let powered_by = headers.get("x-powered-by").and_then(|v| v.to_str().ok());
    let info_leaked = server_header.is_some() || powered_by.is_some();
    findings.push(Finding {
        wstg_id: "WSTG-CONF-002".to_string(),
        category: WstgCategory::ConfigurationManagement,
        title: "Server Banner & Version Information Disclosure".to_string(),
        severity: Severity::Low,
        passed: !info_leaked,
        description: if info_leaked {
            format!("Server disclosures detected: Server={:?}, X-Powered-By={:?}", server_header, powered_by)
        } else {
            "No sensitive software version banners exposed in HTTP response headers.".to_string()
        },
        practical_risk: "Discloses exact server and framework versions, allowing attackers to efficiently target known public CVE exploits against the infrastructure.".to_string(),
        remediation: "Strip or mask `Server` and `X-Powered-By` headers in proxy or Axum middleware.".to_string(),
    });

    // 6. WSTG-CONF-007: CORS Policy Check
    let cors_req = client.get(target_url)
        .header("Origin", "https://malicious-attacker.com")
        .send()
        .await;

    if let Ok(cors_resp) = cors_req {
        let cors_headers = cors_resp.headers();
        let allow_origin = cors_headers.get("access-control-allow-origin").and_then(|v| v.to_str().ok());
        let allow_creds = cors_headers.get("access-control-allow-credentials").and_then(|v| v.to_str().ok());

        let wildcard = allow_origin == Some("*");
        let reflected_origin = allow_origin == Some("https://malicious-attacker.com");
        let unsafe_cors = wildcard || (reflected_origin && allow_creds == Some("true"));

        findings.push(Finding {
            wstg_id: "WSTG-CONF-007".to_string(),
            category: WstgCategory::ConfigurationManagement,
            title: "Cross-Origin Resource Sharing (CORS) Policy Audit".to_string(),
            severity: Severity::High,
            passed: !unsafe_cors,
            description: if unsafe_cors {
                format!("Insecure CORS configuration detected! Access-Control-Allow-Origin: {:?}, Allow-Credentials: {:?}", allow_origin, allow_creds)
            } else {
                "CORS policy restricts unauthorized external origins safely.".to_string()
            },
            practical_risk: "Allows arbitrary external websites to make authenticated API requests on behalf of users and extract sensitive private payload responses.".to_string(),
            remediation: "Avoid wildcard CORS origins and never reflect untrusted Origins with Allow-Credentials: true.".to_string(),
        });
    }

    findings
}
