use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use regex::Regex;

pub(crate) struct MacAddress {}

impl<T: StrAsRef> IValidatorRule<T> for MacAddress {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            let pattern = r"(?i)^([0-9A-F]{2}[:-]){5}([0-9A-F]{2})$|^[0-9A-F]{12}$";
            let regex = Regex::new(pattern).unwrap();

            if !regex.is_match(s) {
                return Err(ValidationError::new("validator.invalid_mac"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator() -> MacAddress {
        MacAddress {}
    }

    #[test]
    fn validates_mac_with_colons() {
        let value = "00:1A:2B:3C:4D:5E";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_mac_with_dashes() {
        let value = "00-1A-2B-3C-4D-5E";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_mac_without_separators() {
        let value = "001A2B3C4D5E";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_invalid_mac_length() {
        let value = "00:1A:2B:3C:4D"; // too short
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_invalid_mac_chars() {
        let value = "00:1G:2Z:3H:4T:5U"; // non-hex
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
