use crate::AuditReport;
use colored::*;

pub struct ReportPrinter;

impl ReportPrinter {
    /// Renders audit report to terminal stdout with colored indicators
    pub fn print_terminal(report: &AuditReport) {
        println!("\n{}", "================================================================".cyan());
        println!("{}", "       FERROX ENTERPRISE - OWASP WSTG DOMAIN SECURITY AUDIT    ".bold().cyan());
        println!("{}\n", "================================================================".cyan());
        
        println!("Target Domain    : {}", report.target_url.bold());
        println!("Timestamp        : {}", report.timestamp_rfc3339);
        println!("Total Tests Run  : {}", report.total_tests);
        println!("Passed           : {}", report.passed_count.to_string().green());
        println!("Failed           : {}", report.failed_count.to_string().red());
        
        let score_str = format!("{:.1}%", report.score_percentage);
        if report.score_percentage >= 90.0 {
            println!("Compliance Score : {}\n", score_str.green().bold());
        } else if report.score_percentage >= 70.0 {
            println!("Compliance Score : {}\n", score_str.yellow().bold());
        } else {
            println!("Compliance Score : {}\n", score_str.red().bold());
        }

        println!("{}", "----------------------------------------------------------------".cyan());
        println!("{:<15} {:<10} {:<8} {}", "WSTG ID", "CATEGORY", "STATUS", "TEST TITLE");
        println!("{}", "----------------------------------------------------------------".cyan());

        for finding in &report.findings {
            let status = if finding.passed {
                "PASS".green().bold()
            } else {
                "FAIL".red().bold()
            };

            let wstg_tag = finding.wstg_id.bold();
            let category_tag = finding.category.code().dimmed();

            println!("{:<15} {:<10} [{}] {}", wstg_tag, category_tag, status, finding.title);
            
            if !finding.passed {
                println!("   ↳ [{}] {}", finding.severity.to_string().red(), finding.description);
                println!("   ↳ Practical Risk: {}", finding.practical_risk.magenta());
                println!("   ↳ Remediation: {}", finding.remediation.yellow());
                println!();
            }
        }
        println!("{}\n", "================================================================".cyan());
    }

    /// Generates Markdown formatted OWASP WSTG Audit Report
    pub fn to_markdown(report: &AuditReport) -> String {
        let mut md = String::new();
        md.push_str("# 🛡️ Ferrox Enterprise OWASP WSTG Security Audit Report\n\n");
        md.push_str(&format!("- **Target Domain**: `{}`\n", report.target_url));
        md.push_str(&format!("- **Audit Timestamp**: `{}`\n", report.timestamp_rfc3339));
        md.push_str(&format!("- **Overall Compliance Score**: `{:.1}%` (`{}/{} Passed`)\n\n", report.score_percentage, report.passed_count, report.total_tests));

        md.push_str("## Findings Summary\n\n");
        md.push_str("| WSTG Reference | Category | Severity | Status | Title |\n");
        md.push_str("|---|---|---|---|---|\n");

        for f in &report.findings {
            let status_icon = if f.passed { "✅ PASS" } else { "❌ FAIL" };
            md.push_str(&format!("| `{}` | `{}` | `{}` | {} | {} |\n", f.wstg_id, f.category.code(), f.severity, status_icon, f.title));
        }

        md.push_str("\n## Detailed Vulnerability & Remediation Findings (Vulnerability Dictionary Bound)\n\n");
        for f in &report.findings {
            let status_icon = if f.passed { "✅ PASS" } else { "❌ FAIL" };
            md.push_str(&format!("### {} [{}] {}\n\n", status_icon, f.wstg_id, f.title));
            md.push_str(&format!("- **Category**: {}\n", f.category.code()));
            md.push_str(&format!("- **Severity**: `{}`\n", f.severity));
            
            if let Some(dict) = crate::vocabulary::VulnerabilityDictionary::lookup(&f.wstg_id) {
                md.push_str(&format!("- **Concetto Divulgativo**: {}\n", dict.plain_concept));
            }

            md.push_str(&format!("- **Description**: {}\n", f.description));
            md.push_str(&format!("- **Rischio Pratico**: {}\n", f.practical_risk));
            md.push_str(&format!("- **OWASP Remediation**: {}\n\n", f.remediation));
        }

        md
    }
}
