use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Generates a secure, time-ordered UUID v7 string.
/// UUIDv7 is superior for database primary keys as it prevents index fragmentation.
pub fn generate_uuid() -> String {
    Uuid::now_v7().to_string()
}

/// Extension trait to add domain-specific utility methods to Strings
pub trait StringUtils {
    /// Masks a string, keeping only the first `keep_start` and last `keep_end` characters visible.
    fn mask(&self, keep_start: usize, keep_end: usize) -> String;
}

impl StringUtils for String {
    fn mask(&self, keep_start: usize, keep_end: usize) -> String {
        if self.len() <= keep_start + keep_end {
            return self.clone();
        }
        
        let start = &self[..keep_start];
        let end = &self[self.len() - keep_end..];
        let masked_len = self.len() - keep_start - keep_end;
        let mask_chars = "*".repeat(masked_len);
        
        format!("{}{}{}", start, mask_chars, end)
    }
}

pub fn setup() {
    println!("yalc-utils initialized: Provides utility functions and extension traits.");
}

#[cfg(test)]
mod tests {
    use super::*;

    // TDD: Test that uuid generator works
    #[test]
    fn test_generate_uuid() {
        let id1 = generate_uuid();
        let id2 = generate_uuid();
        
        assert_eq!(id1.len(), 36);
        assert_ne!(id1, id2); // Must be random
    }

    // TDD: Test String Extension trait
    #[test]
    fn test_string_masking() {
        let email = String::from("admin@antigravity.com");
        let masked = email.mask(3, 4); // "adm**************.com"
        
        assert!(masked.starts_with("adm"));
        assert!(masked.ends_with(".com"));
        assert!(masked.contains("*"));
        assert_eq!(masked.len(), email.len());
        
        // Edge case: string too short
        let short = String::from("a");
        assert_eq!(short.mask(3, 4), "a");
    }
}
