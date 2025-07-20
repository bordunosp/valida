use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use regex::Regex;
use std::collections::HashMap;

pub struct RegexMatch {
    pub pattern: Regex,
}

impl<T: StrAsRef> IValidatorRule<T> for RegexMatch {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            if !self.pattern.is_match(s) {
                return Err(ValidationError::new_with_params(
                    "validator.regex",
                    HashMap::from([("pattern".into(), self.pattern.as_str().into())]),
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
    use regex::Regex;

    fn validator(pattern: &str) -> RegexMatch {
        RegexMatch {
            pattern: Regex::new(pattern).unwrap(),
        }
    }

    #[test]
    fn validates_simple_pattern() {
        let value = "abc123";
        let result = validator(r"^[a-z]+\d+$").validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_on_invalid_pattern_match() {
        let value = "123abc";
        let result = validator(r"^[a-z]+\d+$").validate(&value);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.key, "validator.regex");
    }

    #[test]
    fn validates_email_format() {
        let value = "user@example.com";
        let result = validator(r"^[\w\.-]+@[\w\.-]+\.\w+$").validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_invalid_email_format() {
        let value = "user@@example..com";
        let result = validator(r"^[\w\.-]+@[\w\.-]+\.\w+$").validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        let result = validator(r".*").validate(&value);
        assert!(result.is_ok());
    }
}
