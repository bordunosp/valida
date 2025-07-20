use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use std::collections::HashMap;

pub struct Charset {
    pub allowed: fn(char) -> bool,
}

impl<T: StrAsRef> IValidatorRule<T> for Charset {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            if let Some(c) = s.chars().find(|c| !(self.allowed)(*c)) {
                return Err(ValidationError::new_with_params(
                    "validator.charset",
                    HashMap::from([("invalid".into(), c.to_string())]),
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

    fn ascii_only(c: char) -> bool {
        c.is_ascii()
    }

    fn latin_letters_only(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn validator_ascii() -> Charset {
        Charset {
            allowed: ascii_only,
        }
    }

    fn validator_latin() -> Charset {
        Charset {
            allowed: latin_letters_only,
        }
    }

    #[test]
    fn validates_ascii_string() {
        let value = "Hello123!";
        let result = validator_ascii().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_non_ascii_string() {
        let value = "Привіт";
        let result = validator_ascii().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_latin_letters() {
        let value = "RustLang";
        let result = validator_latin().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_latin_with_numbers() {
        let value = "Rust2023";
        let result = validator_latin().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        let result = validator_ascii().validate(&value);
        assert!(result.is_ok());
    }
}
