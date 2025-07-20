use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use serde_json;

pub(crate) struct Json {}

impl<T: StrAsRef> IValidatorRule<T> for Json {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            if serde_json::from_str::<serde_json::Value>(s).is_err() {
                return Err(ValidationError::new("validator.invalid_json"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator() -> Json {
        Json {}
    }

    #[test]
    fn validates_valid_json_object() {
        let value = r#"{"name":"Pavel","age":30}"#;
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_valid_json_array() {
        let value = r#"["a", "b", "c"]"#;
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_valid_json_null() {
        let value = "null";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_invalid_json() {
        let value = r#"{"missing": "comma" "next": true}"#;
        let result = validator().validate(&value);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.key, "validator.invalid_json");
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }
}
