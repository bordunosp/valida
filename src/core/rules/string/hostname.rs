use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use regex::Regex;

pub(crate) struct Hostname {}

impl<T: StrAsRef> IValidatorRule<T> for Hostname {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            let total_len_ok = s.len() <= 253;

            let pattern =
                r"^(?:(?:[a-zA-Z0-9](?:[a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?)\.)*(?:[a-zA-Z]{2,63})$";
            let regex = Regex::new(pattern).unwrap();

            if !total_len_ok || !regex.is_match(s) {
                return Err(ValidationError::new("validator.hostname"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator() -> Hostname {
        Hostname {}
    }

    #[test]
    fn validates_simple_hostname() {
        let value = "example.com";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_localhost() {
        let value = "localhost";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_subdomain_hostname() {
        let value = "api.v1.example.co.uk";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_hostname_with_invalid_chars() {
        let value = "invalid_host@name!";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_hostname_with_double_dots() {
        let value = "example..com";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_hostname_too_long() {
        let value = "a.".repeat(127) + "com"; // >253 chars
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
