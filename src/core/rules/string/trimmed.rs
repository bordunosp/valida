use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;

pub(crate) struct Trimmed {}

impl<T: StrAsRef> IValidatorRule<T> for Trimmed {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            if s != s.trim() {
                return Err(ValidationError::new("validator.trimmed"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator() -> Trimmed {
        Trimmed {}
    }

    #[test]
    fn validates_trimmed_string() {
        let value = "hello@example.com";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_leading_whitespace() {
        let value = "  hello@example.com";
        let result = validator().validate(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.key, "validator.trimmed");
    }

    #[test]
    fn fails_trailing_whitespace() {
        let value = "hello@example.com   ";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_both_sides_whitespace() {
        let value = "  hello@example.com   ";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_empty_string() {
        let value = "";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_option_some_trimmed() {
        let value = Some("TRIMMED".to_string());
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_option_some_untrimmed() {
        let value = Some("  NOT TRIMMED ".to_string());
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_unicode_trimmed() {
        let value = "Öäüß";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_unicode_with_spaces() {
        let value = "  Öäüß ";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }
}
