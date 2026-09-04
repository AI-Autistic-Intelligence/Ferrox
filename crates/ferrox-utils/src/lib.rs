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
