//! # Ferrox i18n (`ferrox-i18n`)
//!
//! `ferrox-i18n` provides backend internationalization (i18n) for translating error messages and localized responses based on HTTP `Accept-Language` headers.
//!
//! ## Key Features
// Using rust-i18n for robust, macro-based, compile-time checked translations.
// Requires a `locales/` directory in the project root.
rust_i18n::i18n!("locales", fallback = "en");

/// The i18n translation engine parsing and localizing strings.
#[derive(Clone)]
pub struct Translator {
    default_lang: String,
}

impl Translator {
    /// Initializes the Translation engine.
    /// In production, `rust_i18n` automatically loads `locales/*.yml` or `locales/*.json`.
    pub fn new(default_lang: &str) -> Self {
        // Sets the global default language for rust-i18n
        rust_i18n::set_locale(default_lang);
        Self {
            default_lang: default_lang.to_string(),
        }
    }

    /// Resolves a message using the Accept-Language header fallback logic
    pub fn get_message(&self, lang_header: Option<&str>, key: &str) -> String {
        let lang = lang_header.unwrap_or(&self.default_lang);
        rust_i18n::t!(key, locale = lang).to_string()
    }
}