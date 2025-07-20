use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Serialize)]
pub struct ValidationError {
    pub key: String,
    pub params: HashMap<String, String>,
}

impl ValidationError {
    pub fn new<K: Into<String>>(key: K) -> Self {
        Self {
            key: key.into(),
            params: HashMap::new(),
        }
    }

    pub fn new_with_params<K: Into<String>>(key: K, params: HashMap<String, String>) -> Self {
        Self {
            key: key.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ValidationNode {
    Leaf(ValidationError),
    Branch(HashMap<String, ValidationNode>),
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ValidationErrors {
    pub errors: HashMap<String, ValidationNode>,
}

impl ValidationErrors {
    pub(crate) fn add(&mut self, field_path: Vec<String>, error: ValidationError) {
        let mut current = &mut self.errors;
        for (i, part) in field_path.iter().enumerate() {
            if i == field_path.len() - 1 {
                current.insert(part.clone(), ValidationNode::Leaf(error.clone()));
            } else {
                current = match current
                    .entry(part.clone())
                    .or_insert_with(|| ValidationNode::Branch(HashMap::new()))
                {
                    ValidationNode::Branch(map) => map,
                    ValidationNode::Leaf(_) => unreachable!("Invalid structure"),
                };
            }
        }
    }

    pub(crate) fn add_nested(&mut self, field_path: Vec<String>, errors: ValidationErrors) {
        fn merge_at_path(
            target: &mut HashMap<String, ValidationNode>,
            path: &[String],
            subtree: HashMap<String, ValidationNode>,
        ) {
            if path.is_empty() {
                for (key, node) in subtree {
                    target.insert(key, node);
                }
            } else {
                let head = &path[0];
                let tail = &path[1..];

                let entry = target
                    .entry(head.clone())
                    .or_insert_with(|| ValidationNode::Branch(HashMap::new()));

                if let ValidationNode::Branch(branch_map) = entry {
                    merge_at_path(branch_map, tail, subtree);
                } else {
                    unreachable!("Cannot insert nested validation under a leaf node");
                }
            }
        }

        merge_at_path(&mut self.errors, &field_path, errors.errors);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_error_for_field(&self, field: &str) -> bool {
        self.errors.contains_key(field)
    }

    pub fn pretty_print_raw(&self) -> String {
        fn print_node(node: &HashMap<String, ValidationNode>, indent: usize, output: &mut String) {
            for (key, value) in node {
                match value {
                    ValidationNode::Leaf(error) => {
                        output.push_str(&"  ".repeat(indent));
                        output.push_str(&format!("{key}: {0}\n", error.key));
                    }
                    ValidationNode::Branch(sub_map) => {
                        output.push_str(&"  ".repeat(indent));
                        output.push_str(&format!("{key}:\n"));
                        print_node(sub_map, indent + 1, output);
                    }
                }
            }
        }

        let mut output = String::new();
        print_node(&self.errors, 0, &mut output);
        output
    }

    pub fn to_json_raw(&self) -> Value {
        fn convert_raw(node: &ValidationNode) -> Value {
            match node {
                ValidationNode::Leaf(err) => {
                    let mut obj = Map::new();
                    obj.insert("key".to_string(), Value::String(err.key.clone()));

                    let params = err
                        .params
                        .iter()
                        .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                        .collect::<Map<String, Value>>();

                    obj.insert("params".to_string(), Value::Object(params));
                    Value::Object(obj)
                }
                ValidationNode::Branch(map) => {
                    let mut obj = Map::new();
                    for (key, child) in map {
                        obj.insert(key.clone(), convert_raw(child));
                    }
                    Value::Object(obj)
                }
            }
        }

        let mut root = Map::new();

        for (field, node) in &self.errors {
            root.insert(field.clone(), convert_raw(node));
        }

        Value::Object(root)
    }

    pub fn to_json_form_raw(&self) -> Value {
        fn flatten(node: &ValidationNode, path: Vec<String>, output: &mut Map<String, Value>) {
            match node {
                ValidationNode::Leaf(err) => {
                    let key = path_to_html_key(&path);
                    let mut obj = Map::new();

                    obj.insert("key".into(), Value::String(err.key.clone()));

                    let params = err
                        .params
                        .iter()
                        .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                        .collect::<Map<_, _>>();

                    obj.insert("params".into(), Value::Object(params));

                    output.insert(key, Value::Object(obj));
                }
                ValidationNode::Branch(children) => {
                    for (child_key, child_node) in children {
                        let mut new_path = path.clone();
                        new_path.push(child_key.clone());
                        flatten(child_node, new_path, output);
                    }
                }
            }
        }

        fn path_to_html_key(path: &[String]) -> String {
            let mut iter = path.iter();
            if let Some(first) = iter.next() {
                let mut result = first.clone();
                for segment in iter {
                    result.push_str(&format!("[{segment}]"));
                }
                result
            } else {
                String::new()
            }
        }

        let mut output = Map::new();

        for (key, node) in &self.errors {
            flatten(node, vec![key.clone()], &mut output);
        }

        Value::Object(output)
    }

    pub fn to_json_dot_raw(&self) -> Value {
        use serde_json::{Map, Value};

        fn flatten(node: &ValidationNode, path: Vec<String>, output: &mut Map<String, Value>) {
            match node {
                ValidationNode::Leaf(err) => {
                    let key = path_to_dot_key(&path);
                    let mut obj = Map::new();

                    obj.insert("key".into(), Value::String(err.key.clone()));

                    let params = err
                        .params
                        .iter()
                        .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                        .collect::<Map<_, _>>();

                    obj.insert("params".into(), Value::Object(params));

                    output.insert(key, Value::Object(obj));
                }
                ValidationNode::Branch(children) => {
                    for (child_key, child_node) in children {
                        let mut new_path = path.clone();
                        new_path.push(child_key.clone());
                        flatten(child_node, new_path, output);
                    }
                }
            }
        }

        fn path_to_dot_key(path: &[String]) -> String {
            path.join(".")
        }

        let mut output = Map::new();

        for (key, node) in &self.errors {
            flatten(node, vec![key.clone()], &mut output);
        }

        Value::Object(output)
    }

    #[cfg(feature = "i18n-localization")]
    pub fn pretty_print(&self, locale: &str) -> String {
        crate::core::features::localization::i18n::pretty_print_i18n::pretty_print_i18n(
            self, locale,
        )
    }

    #[cfg(feature = "i18n-localization")]
    pub fn to_json(&self, locale: &str) -> Value {
        crate::core::features::localization::i18n::to_json_i18n::to_json_i18n(self, locale)
    }

    #[cfg(feature = "i18n-localization")]
    pub fn to_json_form(&self, locale: &str) -> Value {
        crate::core::features::localization::i18n::to_json_form_i18n::to_json_form_i18n(
            self, locale,
        )
    }

    #[cfg(feature = "i18n-localization")]
    pub fn to_json_dot(&self, locale: &str) -> Value {
        crate::core::features::localization::i18n::to_json_dot_i18n::to_json_dot_i18n(self, locale)
    }
}

impl Error for ValidationError {}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.key, self.params)
    }
}

impl Error for ValidationErrors {}

impl Display for ValidationErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_print_raw())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validation_error_new() {
        let error = ValidationError::new("validator.required");
        assert_eq!(error.key, "validator.required");
        assert!(error.params.is_empty());
    }

    #[test]
    fn test_validation_error_with_params() {
        let mut params = HashMap::new();
        params.insert("min".into(), "5".into());

        let error = ValidationError::new_with_params("validator.min_length", params.clone());
        assert_eq!(error.key, "validator.min_length");
        assert_eq!(error.params, params);
    }

    #[test]
    fn test_validation_error_display() {
        let mut params = HashMap::new();
        params.insert("max".into(), "10".into());

        let error = ValidationError::new_with_params("validator.max_length", params);
        let display = format!("{error}");

        assert!(display.contains("validator.max_length"));
        assert!(display.contains("max"));
    }

    #[test]
    fn test_validation_errors_add_and_structure() {
        let mut errors = ValidationErrors::default();
        let err = ValidationError::new("validator.required");
        errors.add(vec!["profile".into(), "name".into()], err);

        match errors.errors.get("profile") {
            Some(ValidationNode::Branch(branch)) => match branch.get("name") {
                Some(ValidationNode::Leaf(inner_err)) => {
                    assert_eq!(inner_err.key, "validator.required");
                }
                _ => panic!("Missing leaf node"),
            },
            _ => panic!("Missing branch node"),
        }
    }

    #[test]
    fn test_validation_errors_is_empty_and_has_error() {
        let mut errors = ValidationErrors::default();
        assert!(errors.is_empty());

        let err = ValidationError::new("validator.required");
        errors.add(vec!["email".into()], err);

        assert!(!errors.is_empty());
        assert!(errors.has_error_for_field("email"));
    }

    #[test]
    fn test_to_json_raw_nested_structure() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["profile".into(), "age".into()],
            ValidationError::new_with_params("validator.max_length", {
                let mut p = HashMap::new();
                p.insert("max".into(), "4".into());
                p
            }),
        );

        let json = errors.to_json_raw();
        let expected = json!({
            "profile": {
                "age": {
                    "key": "validator.max_length",
                    "params": { "max": "4" }
                }
            }
        });

        assert_eq!(json, expected);
    }

    #[test]
    fn test_to_json_form_raw_html_flattening() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["data".into(), "device".into(), "name".into()],
            ValidationError::new_with_params("validator.min_length", {
                let mut p = HashMap::new();
                p.insert("min".into(), "2".into());
                p
            }),
        );

        let json = errors.to_json_form_raw();
        let expected = json!({
            "data[device][name]": {
                "key": "validator.min_length",
                "params": { "min": "2" }
            }
        });

        assert_eq!(json, expected);
    }

    #[test]
    fn test_to_json_dot_raw_flattening() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["config".into(), "hostname".into()],
            ValidationError::new_with_params("validator.hostname", {
                let mut p = HashMap::new();
                p.insert("type".into(), "fqdn".into());
                p
            }),
        );

        let json = errors.to_json_dot_raw();
        let expected = json!({
            "config.hostname": {
                "key": "validator.hostname",
                "params": { "type": "fqdn" }
            }
        });

        assert_eq!(json, expected);
    }

    #[test]
    fn test_add_nested_merges_validation_errors_recursively() {
        let mut parent = ValidationErrors::default();
        let mut child = ValidationErrors::default();

        // Child має свою вкладену гілку
        child.add(
            vec!["address".into(), "city".into()],
            ValidationError::new("validator.required"),
        );

        // Parent вставляє child під profile
        parent.add_nested(vec!["profile".into()], child);

        let json = parent.to_json_dot_raw();

        let expected = json!({
            "profile.address.city": {
                "key": "validator.required",
                "params": {}
            }
        });

        assert_eq!(json, expected);
    }

    #[test]
    fn test_pretty_print_raw_nested_structure() {
        let mut errors = ValidationErrors::default();

        errors.add(
            vec!["profile".into(), "age".into()],
            ValidationError::new("validator.max_length"),
        );

        errors.add(
            vec!["profile".into(), "device".into(), "name".into()],
            ValidationError::new("validator.min_length"),
        );

        let output = errors.pretty_print_raw();

        assert!(output.contains("profile:"));
        assert!(
            output.contains("  age: validator.max_length")
                || output.contains("age: validator.max_length")
        );
        assert!(output.contains("  device:"));
        assert!(output.contains("    name: validator.min_length"));
    }

    #[test]
    fn test_display_trait_matches_pretty_print() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["email".into()],
            ValidationError::new("validator.required"),
        );

        let display_output = format!("{errors}");
        let raw_output = errors.pretty_print_raw();

        assert_eq!(display_output, raw_output);
    }
}

#[cfg(all(test, feature = "i18n-localization"))]
mod i18n_tests {
    use super::*;
    use crate::core::features::localization::i18n::valida_backend;
    use rust_i18n::i18n;

    i18n!("locales", backend = valida_backend::ValidaBackend::new());

    fn make_error(key: &str, params: &[(&str, &str)]) -> ValidationError {
        let mut map = HashMap::new();
        for (k, v) in params {
            map.insert(k.to_string(), v.to_string());
        }
        ValidationError::new_with_params(key.to_string(), map)
    }

    #[test]
    fn test_to_json_i18n_output() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["profile".into(), "age".into()],
            make_error("validator.max_length", &[("max", "4")]),
        );

        let json = errors.to_json("uk");

        let expected = serde_json::json!({
            "profile": {
                "age": "Максимальна довжина — 4 символів"
            }
        });

        assert_eq!(json, expected);
    }

    #[test]
    fn test_to_json_form_i18n_flat_keys() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["profile".into(), "device".into(), "name".into()],
            make_error("validator.min_length", &[("min", "2")]),
        );

        let json = errors.to_json_form("uk");

        let expected = serde_json::json!({
            "profile[device][name]": "Мінімальна довжина — 2 символів"
        });

        assert_eq!(json, expected);
    }

    #[test]
    fn test_to_json_dot_i18n_flat_keys() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["user".into(), "nickname".into()],
            make_error("validator.min_length", &[("min", "3")]),
        );

        let json = errors.to_json_dot("uk");

        let expected = serde_json::json!({
            "user.nickname": "Мінімальна довжина — 3 символів"
        });

        assert_eq!(json, expected);
    }

    #[test]
    fn test_pretty_print_i18n_nested_output() {
        let mut errors = ValidationErrors::default();
        errors.add(
            vec!["profile".into(), "name".into()],
            make_error("validator.required", &[]),
        );

        let result = errors.pretty_print("uk");

        assert!(result.contains("profile:"));
        assert!(result.contains("  name: Це поле є обов’язковим"));
    }
}
