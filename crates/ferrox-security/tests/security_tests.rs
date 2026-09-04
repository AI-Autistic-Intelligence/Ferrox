use crate::dual_token::DualTokenManager;
use crate::paseto::PasetoAuth;
use crate::public_id::PublicId;
use secrecy::Secret;
use uuid::Uuid;
use crate::AuthPayload;

#[test]
fn test_public_id_masking() {
    let internal_uuid = Uuid::now_v7();
    let prefix = "usr";
    
    // Test Masking
    let public_id = PublicId::mask_uuid(prefix, internal_uuid);
    assert!(public_id.starts_with("usr_"));
    assert_eq!(public_id.len(), 4 + 32); // prefix (4) + hex (32)

    // Test Unmasking
    let unmasked = PublicId::unmask_uuid(prefix, &public_id).unwrap();
    assert_eq!(internal_uuid, unmasked);

    // Test Invalid Unmasking
    let bad_unmask = PublicId::unmask_uuid("org", &public_id);
    assert!(bad_unmask.is_none());
}

#[test]
fn test_dual_token_generation_and_validation() {
    let secret = Secret::new("my_super_secret_key_that_is_32_bytes_long_12345678901234567890".to_string());
    let manager = DualTokenManager::new(secret);
    
    let payload = AuthPayload {
        user_id: "usr_123".into(),
        role: "admin".into(),
    };

    // Generate Tokens
    let tokens = manager.generate_tokens(&payload).unwrap();
    assert!(!tokens.access_token.is_empty());
    assert!(!tokens.refresh_token.is_empty());
    
    // Validate Access Token
    let valid_user_id = manager.verify_access_token(&tokens.access_token).unwrap();
    assert_eq!(valid_user_id, "usr_123");
}
