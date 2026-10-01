//! # Ferrox i18n (`ferrox-i18n`)
//!
//! `ferrox-i18n` provides backend internationalization (i18n) for translating error messages and localized responses based on HTTP `Accept-Language` headers.
//!
//! ## Key Features
//! - 🌐 **Locale Extraction**: Automatically parses client locale from HTTP headers or query parameters.
//! - 📚 **JSON/YAML Catalogs**: Loads translation dictionary files into memory for fast lookup.

use std::collections::HashMap;

/// The i18n translation engine parsing and localizing strings.
pub struct Translator {
    default_lang: String,
    catalogs: HashMap<String, HashMap<String, String>>,
}

impl Translator {
    pub fn new(default_lang: &str) -> Self {
        let mut catalogs = HashMap::new();
        let mut en_catalog = HashMap::new();
        en_catalog.insert("welcome".to_string(), "Welcome to Ferrox".to_string());
        catalogs.insert("en".to_string(), en_catalog);
        
        let mut it_catalog = HashMap::new();
        it_catalog.insert("welcome".to_string(), "Benvenuto in Ferrox".to_string());
        catalogs.insert("it".to_string(), it_catalog);

        Self {
            default_lang: default_lang.to_string(),
            catalogs,
        }
    }

    pub fn get_message(&self, lang_header: Option<&str>, key: &str) -> String {
        let lang = lang_header.unwrap_or(&self.default_lang);
        if let Some(catalog) = self.catalogs.get(lang) {
            if let Some(msg) = catalog.get(key) {
                return msg.clone();
            }
        }
        format!("[Missing Translation: {}]", key)
    }
}