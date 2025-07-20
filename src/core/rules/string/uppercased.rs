use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;

pub(crate) struct Uppercased {}

impl<T: StrAsRef> IValidatorRule<T> for Uppercased {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref()
            && s != s.to_uppercase()
        {
            return Err(ValidationError::new("validator.is_uppercase"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator() -> Uppercased {
        Uppercased {}
    }

    #[test]
    fn validates_uppercased_string() {
        let value = "HELLO";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_lowercase_string() {
        let value = "hello";
        let result = validator().validate(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.key, "validator.is_uppercase");
    }

    #[test]
    fn validates_mixed_case_upper_equivalent() {
        let value = "ÖÄÜ"; // non-ascii uppercase
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_mixed_case_string() {
        let value = "Hello";
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
    fn validates_option_some_uppercase() {
        let value = Some("UPPERCASE".to_string());
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_option_some_lowercase() {
        let value = Some("lowercase".to_string());
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
