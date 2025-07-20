use crate::core::errors::{ValidationErrors, ValidationNode};
use rust_i18n::{replace_patterns, t};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub(crate) fn to_json_dot_i18n(errors: &ValidationErrors, locale: &str) -> Value {
    let mut map = Map::new();

    for (key, node) in &errors.errors {
        flatten(
            node,
            vec![key.clone()],
            if locale == "ru" { "uk" } else { locale },
            &mut map,
        );
    }

    Value::Object(map)
}

fn render(key: &str, locale: &str, params: &HashMap<String, String>) -> String {
    let raw = t!(key, locale = locale);
    if params.is_empty() {
        return raw.into();
    }

    let keys: Vec<&str> = params.keys().map(|k| k.as_str()).collect();
    let values: Vec<String> = keys
        .iter()
        .map(|&k| params.get(k).cloned().unwrap_or_default())
        .collect();
    replace_patterns(&raw, &keys, &values)
}

fn flatten(
    node: &ValidationNode,
    path: Vec<String>,
    locale: &str,
    output: &mut Map<String, Value>,
) {
    match node {
        ValidationNode::Leaf(err) => {
            let key = path_to_dot_key(&path);
            let msg = render(&err.key, locale, &err.params);
            output.insert(key, Value::String(msg));
        }
        ValidationNode::Branch(children) => {
            for (child_key, child_node) in children {
                let mut new_path = path.clone();
                new_path.push(child_key.clone());
                flatten(child_node, new_path, locale, output);
            }
        }
    }
}

fn path_to_dot_key(path: &[String]) -> String {
    path.join(".")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::errors::{ValidationError, ValidationErrors};
    use crate::core::features::localization::i18n::valida_backend;
    use rust_i18n::i18n;
    use std::collections::HashMap;

    i18n!("locales", backend = valida_backend::ValidaBackend::new());

    fn make_error(key: &str, params: &[(&str, &str)]) -> ValidationError {
        let mut map = HashMap::new();
        for (k, v) in params {
            map.insert(k.to_string(), v.to_string());
        }
        ValidationError::new_with_params(key.to_string(), map)
    }

    #[test]
    fn test_single_leaf_key() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["email".into()],
            make_error("validator.min_length", &[("min", "5")]),
        );

        let result = to_json_dot_i18n(&errors, "uk");

        let expected = serde_json::json!({
            "email": "Мінімальна довжина — 5 символів"
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_dot_keys() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["profile".into(), "device".into(), "name".into()],
            make_error("validator.min_length", &[("min", "2")]),
        );

        let result = to_json_dot_i18n(&errors, "uk");

        let expected = serde_json::json!({
            "profile.device.name": "Мінімальна довжина — 2 символів"
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_fields_dot_flat() {
        let mut errors = ValidationErrors::default();

        errors.add(
            vec!["profile".into(), "age".into()],
            make_error("validator.max_length", &[("max", "4")]),
        );

        errors.add(
            vec!["profile".into(), "device".into(), "name".into()],
            make_error("validator.min_length", &[("min", "2")]),
        );

        let result = to_json_dot_i18n(&errors, "uk");

        let expected = serde_json::json!({
            "profile.age": "Максимальна довжина — 4 символів",
            "profile.device.name": "Мінімальна довжина — 2 символів"
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_locale_fallback_ru_to_uk() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["user".into(), "name".into()],
            make_error("validator.min_length", &[("min", "7")]),
        );

        let result = to_json_dot_i18n(&errors, "ru");

        let expected = serde_json::json!({
            "user.name": "Мінімальна довжина — 7 символів"
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_missing_key_falls_back_to_literal() {
        let mut errors = ValidationErrors::default();
        errors.add(vec!["meta".into()], make_error("unknown.key", &[]));

        let result = to_json_dot_i18n(&errors, "uk");

        let expected = serde_json::json!({
            "meta": "unknown.key"
        });

        assert_eq!(result, expected);
    }
}
