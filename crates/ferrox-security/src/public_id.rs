use uuid::Uuid;

/// Helper to mask internal Database UUIDs (e.g. UUIDv7) into Public IDs for the Frontend.
pub struct PublicId;

impl PublicId {
    /// Masks a primary UUID into a public string format with a prefix.
    /// Example: `mask_uuid("usr", my_uuid)` -> `usr_a1b2c3d4...`
    pub fn mask_uuid(prefix: &str, id: Uuid) -> String {
        let hex = id.simple().to_string();
        format!("{}_{}", prefix, hex)
    }

    /// Converts a public string back to an internal UUID.
    /// Returns None if the format is invalid.
    pub fn unmask_uuid(prefix: &str, public_id: &str) -> Option<Uuid> {
        let expected_prefix = format!("{}_", prefix);
        if public_id.starts_with(&expected_prefix) {
            let hex = &public_id[expected_prefix.len()..];
            Uuid::parse_str(hex).ok()
        } else {
            None
        }
    }
}
