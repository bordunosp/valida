use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use std::collections::HashMap;

pub struct NoSuspiciousCharacters {
    pub blacklist: &'static [char],
}

impl<T: StrAsRef> IValidatorRule<T> for NoSuspiciousCharacters {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            if let Some(c) = s.chars().find(|c| self.blacklist.contains(c)) {
                return Err(ValidationError::new_with_params(
                    "validator.no_suspicious",
                    HashMap::from([("char".into(), c.to_string())]),
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

    const DEFAULT_BLACKLIST: &[char] = &['<', '>', '&', '\'', '"', '\\', ';', '(', ')'];

    fn validator() -> NoSuspiciousCharacters {
        NoSuspiciousCharacters {
            blacklist: DEFAULT_BLACKLIST,
        }
    }

    #[test]
    fn passes_clean_text() {
        let value = "HelloWorld_2024";
        assert!(validator().validate(&value).is_ok());
    }

    #[test]
    fn fails_on_angle_brackets() {
        let value = "<script>";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_sql_like_string() {
        let value = "admin' OR '1'='1";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        assert!(validator().validate(&value).is_ok());
    }
}
