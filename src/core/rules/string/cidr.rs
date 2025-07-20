use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use std::collections::HashMap;
use std::net::IpAddr;

pub struct Cidr {}

impl<T: StrAsRef> IValidatorRule<T> for Cidr {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() != 2 {
                return Err(ValidationError::new_with_params(
                    "validator.cidr.format",
                    HashMap::from([("input".into(), s.to_string())]),
                ));
            }

            let ip_part = parts[0];
            let mask_part = parts[1];

            if ip_part.parse::<IpAddr>().is_err() {
                return Err(ValidationError::new_with_params(
                    "validator.cidr.ip_invalid",
                    HashMap::from([("ip".into(), ip_part.to_string())]),
                ));
            }

            let mask_ok = match ip_part.parse::<IpAddr>() {
                Ok(IpAddr::V4(_)) => mask_part.parse::<u8>().is_ok_and(|m| m <= 32),
                Ok(IpAddr::V6(_)) => mask_part.parse::<u8>().is_ok_and(|m| m <= 128),
                Err(_) => false,
            };

            if !mask_ok {
                return Err(ValidationError::new_with_params(
                    "validator.cidr.mask_invalid",
                    HashMap::from([("mask".into(), mask_part.to_string())]),
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

    fn validator() -> Cidr {
        Cidr {}
    }

    #[test]
    fn validates_ipv4_cidr() {
        let value = "192.168.1.0/24";
        assert!(validator().validate(&value).is_ok());
    }

    #[test]
    fn validates_ipv6_cidr() {
        let value = "2001:db8::/32";
        assert!(validator().validate(&value).is_ok());
    }

    #[test]
    fn fails_invalid_ip() {
        let value = "300.300.300.300/24";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_missing_mask() {
        let value = "10.0.0.1";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_invalid_mask_for_ipv4() {
        let value = "192.168.0.0/40";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        assert!(validator().validate(&value).is_ok());
    }
}
