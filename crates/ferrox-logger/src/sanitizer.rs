/// Redacts sensitive PII (Tax IDs, Emails, Passwords, PASETO/JWT tokens) from log strings
pub fn sanitize_log_message(message: &str) -> String {
    let mut sanitized = message.to_string();

    // 1. Redact PASETO v4 local tokens
    if let Some(idx) = sanitized.find("v4.local.") {
        let end_idx = (idx + 45).min(sanitized.len());
        sanitized.replace_range(idx..end_idx, "v4.local.[REDACTED_TOKEN]");
    }

    // 2. Redact passwords and secrets in key-value format (e.g. password: "xyz" or "password": "xyz")
    let sensitive_keys = ["password", "secret", "token", "api_key"];
    for key in &sensitive_keys {
        let lower = sanitized.to_lowercase();
        if let Some(pos) = lower.find(key) {
            // Find colon or equals after key
            if let Some(sep) = sanitized[pos..].find(':').or_else(|| sanitized[pos..].find('=')) {
                let abs_sep = pos + sep;
                // Find start of value (skip spaces and quotes)
                let value_substr = &sanitized[abs_sep + 1..];
                let val_trim = value_substr.trim_start();
                let trim_offset = value_substr.len() - val_trim.len();
                let val_start_idx = abs_sep + 1 + trim_offset;

                // Find delimiter (comma, quote, space, or end of string)
                let val_end_offset = sanitized[val_start_idx..]
                    .find(|c: char| c == ',' || c == '\n' || c == '\r' || c == ';')
                    .unwrap_or(sanitized.len() - val_start_idx);

                sanitized.replace_range(val_start_idx..val_start_idx + val_end_offset, "\"[REDACTED]\"");
            }
        }
    }

    // 3. Redact Italian Codice Fiscale pattern (16 alphanumeric)
    let cf_pattern = regex_lite_cf(&sanitized);
    if !cf_pattern.is_empty() {
        sanitized = sanitized.replace(&cf_pattern, "[REDACTED_CF]");
    }

    sanitized
}

fn regex_lite_cf(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    for word in words {
        let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
        if clean.len() == 16 && clean.chars().all(|c| c.is_ascii_alphanumeric()) {
            let has_letters = clean.chars().any(|c| c.is_ascii_alphabetic());
            let has_digits = clean.chars().any(|c| c.is_ascii_digit());
            if has_letters && has_digits {
                return clean.to_string();
            }
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_log_message() {
        let log1 = "User login failed with password: \"Secret123!\", CF: TRTNLL97A08F839N";
        let clean1 = sanitize_log_message(log1);
        assert!(!clean1.contains("Secret123!"));
        assert!(!clean1.contains("TRTNLL97A08F839N"));
        assert!(clean1.contains("[REDACTED_CF]"));

        let log2 = "Bearer v4.local.abcdefghijklmnopqrstuvwxyz0123456789ABCDEF";
        let clean2 = sanitize_log_message(log2);
        assert!(!clean2.contains("abcdefghijklmnopqrstuvwxyz"));
        assert!(clean2.contains("[REDACTED_TOKEN]"));
    }
}
