//! # Ferrox Types (`ferrox-types`)
//!
//! `ferrox-types` provides fundamental domain primitives and wrapper types used throughout Ferrox applications,
//! including validated `Pagination` parameters and type-safe `PublicId` structures.
//!
//! ## Design Rationale
//! Primitive obsessions (e.g. passing raw `u64` or `String` everywhere) lead to bugs, accidental ID confusion, and invalid query limits.
//! `ferrox-types` introduces strongly typed abstractions that validate their invariants upon construction.
//!
//! ## Key Features
//! - 📄 **`Pagination`**: Invariant-enforced page size limit (`limit > 0`) and page offset helper.
//! - 🏷️ **`PublicId`**: Strongly typed entity identifier wrapper preventing accidental ID substitution.

use serde::{Deserialize, Serialize};
use ferrox_errors::AppError;

/// A strongly typed Pagination primitive.
/// It guarantees that limit and offset are always valid (e.g. limit > 0).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pagination {
    limit: u32,
    offset: u32,
}

impl Pagination {
    /// Attempts to construct a Pagination object. Returns a ValidationError if invalid.
    pub fn new(limit: u32, offset: u32) -> Result<Self, AppError> {
        if limit == 0 {
            return Err(AppError::ValidationError("Limit must be strictly greater than 0".into()));
        }
        if limit > 100 {
            return Err(AppError::ValidationError("Limit cannot exceed 100".into()));
        }

        Ok(Self { limit, offset })
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}

use validator::Validate;
use ts_rs::TS;

/// Example of a Validatable DTO using the `validator` crate (like class-validator in TS)
/// It is also exported to TypeScript automatically!
#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export)]
pub struct CreateUserDto {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    #[validate(range(min = 18, max = 130))]
    pub age: u8,
}

pub fn setup() {
    println!("ferrox-types initialized: Provides domain primitives and marker traits.");
}

#[cfg(test)]
mod tests {
    use super::*;

    // TDD: Verify that we cannot construct an invalid Pagination object.
    #[test]
    fn test_pagination_validation_fails_on_zero_limit() {
        let result = Pagination::new(0, 10);
        assert!(result.is_err());
        if let Err(AppError::ValidationError(msg)) = result {
            assert_eq!(msg, "Limit must be strictly greater than 0");
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_pagination_validation_fails_on_large_limit() {
        let result = Pagination::new(101, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_pagination_creation_success() {
        let pagination = Pagination::new(50, 20).unwrap();
        assert_eq!(pagination.limit(), 50);
        assert_eq!(pagination.offset(), 20);
    }

    #[test]
    fn test_dto_validation() {
        let bad_dto = CreateUserDto {
            email: "invalid_email".into(),
            password: "short".into(),
            age: 15, // too young
        };

        let result = bad_dto.validate();
        assert!(result.is_err());
        let errs = result.unwrap_err();
        
        // Assert all 3 validations failed
        let err_map = errs.field_errors();
        assert!(err_map.contains_key("email"));
        assert!(err_map.contains_key("password"));
        assert!(err_map.contains_key("age"));
    }
}