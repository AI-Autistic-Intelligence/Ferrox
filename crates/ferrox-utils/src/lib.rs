use chrono::{DateTime, TimeZone, Utc};
use convert_case::{Case, Casing};
use time::OffsetDateTime;
use uuid::Uuid;

pub mod date {
    use super::*;

    /// Returns the current time strictly in UTC. 
    /// All databases in Ferrox MUST store dates in UTC.
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }

    /// Converts a given UTC DateTime to a formatted GMT string for the Frontend.
    pub fn to_gmt_string(dt: DateTime<Utc>) -> String {
        dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
    }
}

pub mod string {
    use super::*;

    pub trait StringExt {
        fn to_camel_case(&self) -> String;
        fn to_snake_case(&self) -> String;
        fn to_kebab_case(&self) -> String;
        fn mask(&self, keep_start: usize, keep_end: usize) -> String;
    }

    impl StringExt for String {
        fn to_camel_case(&self) -> String {
            self.to_case(Case::Camel)
        }

        fn to_snake_case(&self) -> String {
            self.to_case(Case::Snake)
        }

        fn to_kebab_case(&self) -> String {
            self.to_case(Case::Kebab)
        }

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
}

/// Generates a secure, time-ordered UUID v7 string.
pub fn generate_uuid() -> String {
    Uuid::now_v7().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::date::*;
    use super::string::*;

    #[test]
    fn test_generate_uuid() {
        let id1 = generate_uuid();
        let id2 = generate_uuid();
        
        assert_eq!(id1.len(), 36);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_date_utilities() {
        let utc = now_utc();
        let gmt_str = to_gmt_string(utc);
        assert!(gmt_str.ends_with("GMT"));
    }

    #[test]
    fn test_string_casing() {
        let original = String::from("hello world test");
        assert_eq!(original.to_camel_case(), "helloWorldTest");
        assert_eq!(original.to_snake_case(), "hello_world_test");
        assert_eq!(original.to_kebab_case(), "hello-world-test");
    }

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
        
        // Exact length
        let exact = String::from("abcdefg");
        assert_eq!(exact.mask(3, 4), "abcdefg");
    }
}
