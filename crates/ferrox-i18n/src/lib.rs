//! # Ferrox i18n (`ferrox-i18n`)
//!
//! `ferrox-i18n` provides backend internationalization (i18n) for translating error messages and localized responses based on HTTP `Accept-Language` headers.
//!
//! ## Key Features
use fluent::{FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::collections::HashMap;
use std::sync::Arc;

/// The i18n translation engine parsing and localizing strings via Project Fluent.
pub struct Translator {
    default_lang: String,
    bundles: HashMap<String, Arc<FluentBundle<FluentResource, IntlLangMemoizer>>>,
}

impl Translator {
    /// Loads translation catalogs from memory or disk (Fluent FTL format)
    pub fn new(default_lang: &str) -> Self {
        let mut bundles = HashMap::new();
        
        // Example fluent string for English
        let en_source = "welcome = Welcome to Ferrox\nerror-not-found = Resource missing";
        let en_res = FluentResource::try_new(en_source.to_string()).expect("Failed to parse FTL");
        let mut en_bundle = FluentBundle::new_concurrent(vec!["en".parse().unwrap()]);
        en_bundle.add_resource(en_res).unwrap();
        bundles.insert("en".to_string(), Arc::new(en_bundle));

        // Example fluent string for Italian
        let it_source = "welcome = Benvenuto in Ferrox\nerror-not-found = Risorsa mancante";
        let it_res = FluentResource::try_new(it_source.to_string()).expect("Failed to parse FTL");
        let mut it_bundle = FluentBundle::new_concurrent(vec!["it".parse().unwrap()]);
        it_bundle.add_resource(it_res).unwrap();
        bundles.insert("it".to_string(), Arc::new(it_bundle));

        Self {
            default_lang: default_lang.to_string(),
            bundles,
        }
    }

    /// Resolves a message using the Accept-Language header fallback logic
    pub fn get_message(&self, lang_header: Option<&str>, key: &str) -> String {
        let lang = lang_header.unwrap_or(&self.default_lang);
        let bundle = self.bundles.get(lang).unwrap_or_else(|| self.bundles.get(&self.default_lang).unwrap());

        if let Some(msg) = bundle.get_message(key) {
            if let Some(pattern) = msg.value() {
                let mut errors = vec![];
                let value = bundle.format_pattern(pattern, None, &mut errors);
                if errors.is_empty() {
                    return value.to_string();
                }
            }
        }
        format!("[Missing Translation: {}]", key)
    }
}