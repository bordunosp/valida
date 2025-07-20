use crate::core::errors::{ValidationErrors, ValidationNode};
use rust_i18n::{replace_patterns, t};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub(crate) fn to_json_i18n(errors: &ValidationErrors, locale: &str) -> Value {
    let mut root = Map::new();

    for (field, node) in &errors.errors {
        root.insert(
            field.clone(),
            convert(node, if locale == "ru" { "uk" } else { locale }),
        );
    }

    Value::Object(root)
}

fn convert(node: &ValidationNode, locale: &str) -> Value {
    match node {
        ValidationNode::Leaf(err) => Value::String(render_template(&err.key, locale, &err.params)),
        ValidationNode::Branch(map) => {
            let mut obj = Map::new();
            for (key, child) in map {
                obj.insert(key.clone(), convert(child, locale));
            }
            Value::Object(obj)
        }
    }
}

fn render_template(key: &str, locale: &str, params: &HashMap<String, String>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::errors::{ValidationError, ValidationErrors, ValidationNode};
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
    fn test_render_template_with_params() {
        let err = make_error("validator.min_length", &[("min", "4")]);
        let result = render_template(&err.key, "uk", &err.params);
        assert_eq!(result, "Мінімальна довжина — 4 символів");
    }

    #[test]
    fn test_render_template_without_params() {
        let err = make_error("validator.required", &[]);
        let result = render_template(&err.key, "uk", &err.params);
        assert_eq!(result, "Це поле є обов’язковим");
    }

    #[test]
    fn test_convert_leaf() {
        let err = make_error("validator.min_length", &[("min", "3")]);
        let node = ValidationNode::Leaf(err);
        let value = convert(&node, "uk");
        assert_eq!(
            value,
            Value::String("Мінімальна довжина — 3 символів".to_string())
        );
    }

    #[test]
    fn test_convert_branch() {
        let mut inner = HashMap::new();
        inner.insert(
            "name".into(),
            ValidationNode::Leaf(make_error("validator.min_length", &[("min", "2")])),
        );
        let branch = ValidationNode::Branch(inner);
        let value = convert(&branch, "uk");

        let expected = serde_json::json!({
            "name": "Мінімальна довжина — 2 символів"
        });

        assert_eq!(value, expected);
    }

    #[test]
    fn test_to_json_i18n_nested_tree() {
        let mut errors = ValidationErrors::default();

        errors.add(
            vec!["profile".into(), "age".into()],
            make_error("validator.max_length", &[("max", "4")]),
        );

        errors.add(
            vec!["profile".into(), "device".into(), "name".into()],
            make_error("validator.min_length", &[("min", "2")]),
        );

        let result = to_json_i18n(&errors, "ru");

        let expected = serde_json::json!({
            "profile": {
                "age": "Максимальна довжина — 4 символів",
                "device": {
                    "name": "Мінімальна довжина — 2 символів"
                }
            }
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_missing_key_returns_as_is() {
        let err = make_error("some.unknown.key", &[]);
        let rendered = render_template(&err.key, "uk", &err.params);
        assert_eq!(rendered, "some.unknown.key"); // fallback behavior
    }
}
