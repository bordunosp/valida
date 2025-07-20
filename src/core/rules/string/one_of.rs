use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use std::collections::{HashMap, HashSet};

pub(crate) struct OneOf {
    pub allowed: HashSet<String>,
}

impl<T: StrAsRef> IValidatorRule<T> for OneOf {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(v) = value.as_str_ref() {
            if !self.allowed.contains(v) {
                return Err(ValidationError::new_with_params(
                    "validator.one_of",
                    HashMap::from([(
                        "allowed".into(),
                        self.allowed.iter().cloned().collect::<Vec<_>>().join(", "),
                    )]),
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

    fn validator() -> OneOf {
        OneOf {
            allowed: HashSet::from(["UA".to_string(), "PL".to_string(), "DE".to_string()]),
        }
    }

    #[test]
    fn validates_allowed_value() {
        let value = "UA";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_disallowed_value() {
        let value = "FR";
        let result = validator().validate(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.key, "validator.one_of");
    }

    #[test]
    fn validates_option_some_allowed() {
        let value = Some("PL".to_string());
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_option_some_disallowed() {
        let value = Some("IT".to_string());
        let result = validator().validate(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.key, "validator.one_of");
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_empty_string_if_allowed() {
        let validator = OneOf {
            allowed: HashSet::from(["".to_string()]),
        };

        let value = "";
        let result = validator.validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_unicode_value_not_in_allowed() {
        let value = "🇺🇦";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }
}
