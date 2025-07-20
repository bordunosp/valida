use crate::core::errors::{ValidationErrors, ValidationNode};
use rust_i18n::{replace_patterns, t};
use std::collections::HashMap;

pub(crate) fn pretty_print_i18n(errors: &ValidationErrors, locale: &str) -> String {
    let mut result = String::new();

    print_node_i18n(
        &errors.errors,
        if locale == "ru" { "uk" } else { locale },
        0,
        &mut result,
    );

    result
}

fn print_node_i18n(
    node: &HashMap<String, ValidationNode>,
    locale: &str,
    indent: usize,
    output: &mut String,
) {
    for (key, value) in node {
        match value {
            ValidationNode::Leaf(error) => {
                output.push_str(&"  ".repeat(indent));
                let text = render_template(&error.key, locale, &error.params);
                output.push_str(&format!("{key}: {text}\n"));
            }
            ValidationNode::Branch(sub_map) => {
                output.push_str(&"  ".repeat(indent));
                output.push_str(&format!("{key}:\n"));
                print_node_i18n(sub_map, locale, indent + 1, output);
            }
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
    fn test_single_leaf_pretty_print() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["email".into()],
            make_error("validator.min_length", &[("min", "5")]),
        );

        let result = pretty_print_i18n(&errors, "uk");

        assert_eq!(result.trim(), "email: Мінімальна довжина — 5 символів");
    }

    #[test]
    fn test_nested_branch_output() {
        let mut errors = ValidationErrors::default();

        errors.add(
            vec!["profile".into(), "device".into(), "name".into()],
            make_error("validator.min_length", &[("min", "2")]),
        );

        let result = pretty_print_i18n(&errors, "uk");

        let expected = r#"
profile:
  device:
    name: Мінімальна довжина — 2 символів
"#
        .trim();

        assert_eq!(result.trim(), expected);
    }

    #[test]
    fn test_multiple_fields_pretty() {
        let mut errors = ValidationErrors::default();

        errors.add(
            vec!["profile".into(), "age".into()],
            make_error("validator.max_length", &[("max", "4")]),
        );
        errors.add(
            vec!["profile".into(), "device".into(), "name".into()],
            make_error("validator.min_length", &[("min", "2")]),
        );

        let result = pretty_print_i18n(&errors, "uk");

        assert!(result.contains("profile:"));
        assert!(
            result.contains("  age: Максимальна довжина — 4 символів")
                || result.contains("age: Максимальна довжина — 4 символів")
        );
        assert!(result.contains("  device:"));
        assert!(result.contains("    name: Мінімальна довжина — 2 символів"));
    }

    #[test]
    fn test_locale_fallback_ru_to_uk() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["username".into()],
            make_error("validator.min_length", &[("min", "6")]),
        );

        let result = pretty_print_i18n(&errors, "ru");

        assert!(result.contains("username: Мінімальна довжина — 6 символів"));
    }

    #[test]
    fn test_missing_key_returns_key_name() {
        let mut errors = ValidationErrors::default();
        errors.add(vec!["field".into()], make_error("unknown.key", &[]));

        let result = pretty_print_i18n(&errors, "uk");

        assert_eq!(result.trim(), "field: unknown.key");
    }
}
