use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;

pub(crate) struct Lowercased {}

impl<T: StrAsRef> IValidatorRule<T> for Lowercased {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref()
            && s != s.to_lowercase()
        {
            return Err(ValidationError::new("validator.is_lowercase"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator() -> Lowercased {
        Lowercased {}
    }

    #[test]
    fn validates_fully_lowercased_string() {
        let value = "rustlang";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_mixed_case_string() {
        let value = "RustLang";
        let result = validator().validate(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.key, "validator.is_lowercase");
    }

    #[test]
    fn fails_uppercase_string() {
        let value = "RUST";
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
    fn validates_unicode_lowercase() {
        let value = "бджола"; // lowercased Unicode
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_unicode_mixed_case() {
        let value = "Бджола"; // starts with uppercase character
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_some_lowercase() {
        let value = Some("lowered".to_string());
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_option_some_uppercase() {
        let value = Some("NOT LOWER".to_string());
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }
}
