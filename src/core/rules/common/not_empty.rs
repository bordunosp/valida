use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use std::collections::{BTreeMap, HashMap};

pub(crate) struct NotEmpty {}

impl<T: RuleTarget> IValidatorRule<T> for NotEmpty {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if value.is_empty() {
            return Err(ValidationError::new("validator.required"));
        }
        Ok(())
    }
}

pub trait RuleTarget {
    fn is_empty(&self) -> bool;
}

// Строкові типи
impl RuleTarget for &str {
    fn is_empty(&self) -> bool {
        self.trim().is_empty()
    }
}

impl RuleTarget for String {
    fn is_empty(&self) -> bool {
        self.trim().is_empty()
    }
}

impl<T> RuleTarget for Vec<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Option<T>
impl<T: RuleTarget> RuleTarget for Option<T> {
    fn is_empty(&self) -> bool {
        self.as_ref().is_none_or(|v| v.is_empty())
    }
}

// Колекції
impl<K, V> RuleTarget for HashMap<K, V> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> RuleTarget for BTreeMap<K, V> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    macro_rules! test_not_empty {
        ($name:ident, $value:expr, $should_pass:expr) => {
            #[test]
            fn $name() {
                let validator = NotEmpty {};
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $should_pass);
            }
        };
    }

    // ┌──── Строкові типи ─────┐
    test_not_empty!(ne_str_ok, "hello", true);
    test_not_empty!(ne_str_fail_empty, "", false);
    test_not_empty!(ne_str_fail_whitespace, "   ", false);
    test_not_empty!(ne_string_ok, String::from("data"), true);
    test_not_empty!(ne_string_fail, String::from("  "), false);

    // ┌──── Вектор ─────┐
    test_not_empty!(ne_vec_ok, vec![1, 2, 3], true);
    test_not_empty!(ne_vec_fail, Vec::<u8>::new(), false);

    // ┌──── Option<T> ─────┐
    test_not_empty!(ne_option_some_str_ok, Some("value"), true);
    test_not_empty!(ne_option_some_str_fail, Some("   "), false);
    test_not_empty!(ne_option_none, None::<String>, false);
    test_not_empty!(ne_option_some_vec_ok, Some(vec![42]), true);
    test_not_empty!(ne_option_some_vec_empty, Some(Vec::<i32>::new()), false);

    // ┌──── HashMap ─────┐
    test_not_empty!(
        ne_hashmap_ok,
        {
            let mut map = HashMap::new();
            map.insert("key", 123);
            map
        },
        true
    );

    test_not_empty!(ne_hashmap_fail, HashMap::<String, i32>::new(), false);

    // ┌──── BTreeMap ─────┐
    test_not_empty!(
        ne_btreemap_ok,
        {
            let mut map = BTreeMap::new();
            map.insert("a", 1);
            map
        },
        true
    );

    test_not_empty!(ne_btreemap_fail, BTreeMap::<String, i32>::new(), false);
}
