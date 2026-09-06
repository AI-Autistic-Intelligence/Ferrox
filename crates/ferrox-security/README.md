# Ferrox Security (`ferrox-security`)

`ferrox-security` provides zero-trust authentication mechanisms, including PASETO (Platform-Agnostic Security Tokens) v4 local/public token generation,
dual-token refresh rotation, password hashing abstractions, and authorization claim extractors.

## Why PASETO over JWT?
Traditional JSON Web Tokens (JWT) suffer from algorithm confusion attacks (e.g., `none` algorithm vulnerability, RSA vs HMAC confusion).
PASETO eliminates algorithm negotiation entirely by hardcoding modern cryptographic primitives (Ed25519, XChaCha20-Poly1305), making security token handling foolproof.

## Key Features
- 🛡️ **PASETO v4 Support**: Encrypted local tokens and signed public tokens.
- 🔑 **Token Engine**: Issue, verify, and refresh access tokens securely.
- 🔒 **Password Hashing**: Secure Argon2id password hashing integration.
