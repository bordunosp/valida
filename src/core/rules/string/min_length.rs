use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use std::collections::HashMap;

pub(crate) struct MinLength {
    pub min: usize,
}

impl<T: StrAsRef> IValidatorRule<T> for MinLength {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            let actual = s.chars().count();
            if actual < self.min {
                return Err(ValidationError::new_with_params(
                    "validator.min_length",
                    HashMap::from([("min".to_string(), self.min.to_string())]),
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator(min: usize) -> MinLength {
        MinLength { min }
    }

    #[test]
    fn validates_string_with_min_length() {
        let value = "rustacean";
        let result = validator(5).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_string_shorter_than_min() {
        let value = "abc";
        let result = validator(5).validate(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.key, "validator.min_length");
        assert_eq!(error.params.get("min"), Some(&"5".into()));
    }

    #[test]
    fn validates_exact_length() {
        let value = "hello";
        let result = validator(5).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_empty_string_with_zero_min() {
        let value = "";
        let result = validator(0).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_empty_string_with_non_zero_min() {
        let value = "";
        let result = validator(1).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_some_valid() {
        let value = Some("valid".to_string());
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_option_some_too_short() {
        let value = Some("no".to_string());
        let result = validator(3).validate(&value);
        assert!(result.is_err());

        let _ = result.unwrap_err();
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        let result = validator(10).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_unicode_length() {
        let value = "бджола"; // 6 Unicode scalar values
        let result = validator(5).validate(&value);
        assert!(result.is_ok());
    }
}
