use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use std::collections::HashMap;

pub(crate) struct MaxLength {
    pub max: usize,
}

impl<T: StrAsRef> IValidatorRule<T> for MaxLength {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            let actual = s.chars().count();
            if actual > self.max {
                return Err(ValidationError::new_with_params(
                    "validator.max_length",
                    HashMap::from([("max".to_string(), self.max.to_string())]),
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

    fn validator(max: usize) -> MaxLength {
        MaxLength { max }
    }

    #[test]
    fn validates_shorter_than_max() {
        let value = "Rusty";
        let result = validator(10).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_exact_length() {
        let value = "abcdefghij"; // length = 10
        let result = validator(10).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_exceeds_max_length() {
        let value = "abcdefghijk"; // length = 11
        let result = validator(10).validate(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.key, "validator.max_length");
        assert_eq!(error.params.get("max"), Some(&"10".into()));
    }

    #[test]
    fn validates_empty_string() {
        let value = "";
        let result = validator(5).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_option_some_valid() {
        let value = Some("short".to_string()); // length = 5
        let result = validator(5).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_option_some_too_long() {
        let value = Some("too-long-value".to_string()); // length = 14
        let result = validator(10).validate(&value);
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
    fn validates_unicode_length_within_limit() {
        let value = "бджола"; // 6 Unicode scalars
        let result = validator(10).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_unicode_exceeds_limit() {
        let value = "бджола🐝🚀"; // length = 8
        let result = validator(7).validate(&value);
        assert!(result.is_err());
    }
}
