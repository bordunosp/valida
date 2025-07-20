use rust_i18n::Backend;
use serde::Deserialize;
use std::collections::HashMap;

static EMBEDDED_YAML: &str = include_str!("../../../../../locales/valida.yml");

#[derive(Debug, Deserialize)]
struct FullCatalog {
    #[serde(rename = "_version")]
    _version: u8,

    #[serde(flatten)]
    entries: HashMap<String, HashMap<String, String>>,
}

pub(crate) struct ValidaBackend {
    translations: HashMap<String, HashMap<String, String>>,
}

impl Backend for ValidaBackend {
    fn available_locales(&self) -> Vec<&str> {
        self.translations.keys().map(|s| s.as_str()).collect()
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        self.translations
            .get(locale)
            .and_then(|map| map.get(key).map(|s| s.as_str()))
    }
}

impl ValidaBackend {
    pub(crate) fn new() -> Self {
        let raw: FullCatalog =
            serde_yaml::from_str(EMBEDDED_YAML).expect("Invalid valida.yml format");

        let mut translations: HashMap<String, HashMap<String, String>> = HashMap::new();

        for (key, languages) in raw.entries {
            for (lang, text) in languages {
                translations
                    .entry(lang)
                    .or_default()
                    .insert(key.clone(), text);
            }
        }

        ValidaBackend { translations }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_languages_are_loaded() {
        let backend = ValidaBackend::new();

        let expected_locales = vec!["uk", "en", "de", "es", "pl", "hi", "fr", "pt", "ja"];

        let actual_locales = backend.available_locales();

        for expected in expected_locales {
            assert!(
                actual_locales.contains(&expected),
                "Locale '{}' missing in backend",
                expected
            );
        }

        for locale in &actual_locales {
            let msg = backend.translate(locale, "validator.min_length");
            assert!(
                msg.is_some(),
                "Missing 'validator.min_length' for locale '{}'",
                locale
            );
        }
    }
}
