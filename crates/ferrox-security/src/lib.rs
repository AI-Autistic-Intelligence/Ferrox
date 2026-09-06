//! # Ferrox Security (`ferrox-security`)
//!
//! `ferrox-security` provides zero-trust authentication mechanisms, including PASETO (Platform-Agnostic Security Tokens) v4 local/public token generation,
//! dual-token refresh rotation, password hashing abstractions, and authorization claim extractors.
//!
//! ## Why PASETO over JWT?
//! Traditional JSON Web Tokens (JWT) suffer from algorithm confusion attacks (e.g., `none` algorithm vulnerability, RSA vs HMAC confusion).
//! PASETO eliminates algorithm negotiation entirely by hardcoding modern cryptographic primitives (Ed25519, XChaCha20-Poly1305), making security token handling foolproof.
//!
//! ## Key Features
//! - 🛡️ **PASETO v4 Support**: Encrypted local tokens and signed public tokens.
//! - 🔑 **Token Engine**: Issue, verify, and refresh access tokens securely.
//! - 🔒 **Password Hashing**: Secure Argon2id password hashing integration.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use pasetors::{claims::Claims, claims::ClaimsValidationRules, Local};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use ferrox_errors::AppError;

pub mod auth_middleware;
pub mod dual_token;
pub mod public_id;

/// Hashes a password securely using Argon2.
/// The input is wrapped in `Secret<String>` to guarantee it doesn't leak in logs.
pub fn hash_password(password: Secret<String>) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

    Ok(password_hash.to_string())
}

/// Verifies a password against an Argon2 hash.
pub fn verify_password(password: Secret<String>, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::ValidationError(format!("Invalid hash format: {}", e)))?;

    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.expose_secret().as_bytes(), &parsed_hash)
        .is_ok())
}

/// A securely typed PASETO Auth Engine
pub struct PasetoAuth {
    key: SymmetricKey<V4>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub user_id: String,
    pub role: String,
}

impl PasetoAuth {
    pub fn new(secret: Secret<String>) -> Result<Self, AppError> {
        let key = SymmetricKey::<V4>::from(secret.expose_secret().as_bytes())
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        Ok(Self { key })
    }

    /// Generates a local (symmetric) PASETO v4 token
    pub fn generate_token(&self, payload: &AuthPayload, duration: Duration) -> Result<String, AppError> {
        let mut claims = Claims::new().map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        
        // Add expiration
        let exp = OffsetDateTime::now_utc() + duration;
        let exp_iso = exp.format(&time::format_description::well_known::Rfc3339)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        claims.expiration(&exp_iso).map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        // Add custom payload
        let user_id_val = serde_json::to_value(&payload.user_id)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        claims.add_additional("user_id", user_id_val)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
            
        let role_val = serde_json::to_value(&payload.role)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        claims.add_additional("role", role_val)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        // Encrypt and sign
        pasetors::local::encrypt(&self.key, &claims, None, Some(b"ferrox-auth-footer"))
            .map_err(|e| AppError::InternalServerError(Box::new(e)))
    }

    /// Validates a PASETO v4 token and returns the payload if successful
    pub fn validate_token(&self, token: &str) -> Result<AuthPayload, AppError> {
        let validation_rules = ClaimsValidationRules::new();
        let untrusted_token = UntrustedToken::<Local, V4>::try_from(token)
            .map_err(|_| AppError::Unauthorized("Invalid token format".into()))?;

        let claims = pasetors::local::decrypt(
            &self.key,
            &untrusted_token,
            &validation_rules,
            None,
            Some(b"ferrox-auth-footer"),
        )
        .map_err(|_| AppError::Unauthorized("Token validation failed".into()))?;

        let user_id = claims.get_claim("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::Unauthorized("Missing user_id".into()))?;
        
        let role = claims.get_claim("role")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::Unauthorized("Missing role".into()))?;

        Ok(AuthPayload {
            user_id: user_id.to_string(),
            role: role.to_string(),
        })
    }
}

pub fn setup() {
    println!("ferrox-security initialized: Argon2 Hashing and PASETO Authentication ready.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argon2_hashing() {
        let password = Secret::new("SuperSecureP@ssw0rd!".to_string());
        
        // Hash it
        let hash = hash_password(password.clone()).unwrap();
        assert!(hash.starts_with("$argon2"));

        // Verify with correct password
        let is_valid = verify_password(password, &hash).unwrap();
        assert!(is_valid);

        // Verify with wrong password
        let wrong_password = Secret::new("WrongPassword!".to_string());
        let is_valid_wrong = verify_password(wrong_password, &hash).unwrap();
        assert!(!is_valid_wrong);
    }

    #[test]
    fn test_paseto_token_lifecycle() {
        // Must be exactly 32 bytes for V4 local
        let secret = Secret::new("12345678901234567890123456789012".to_string());
        let auth = PasetoAuth::new(secret).unwrap();

        let payload = AuthPayload {
            user_id: "user-123".into(),
            role: "admin".into(),
        };

        // Generate token valid for 1 hour
        let token = auth.generate_token(&payload, Duration::hours(1)).unwrap();
        assert!(token.starts_with("v4.local.")); // PASETO V4 local format

        // Validate token
        let validated = auth.validate_token(&token).unwrap();
        assert_eq!(validated.user_id, "user-123");
        assert_eq!(validated.role, "admin");

        // Validate bad token
        let bad_token = "v4.local.bad_data_here";
        assert!(auth.validate_token(bad_token).is_err());
    }
}