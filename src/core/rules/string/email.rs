use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use idna::domain_to_ascii;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

static LOCAL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[\p{L}0-9.!#$%&'*+/=?^_`{|}~-]+$").unwrap());

fn validate_email(email: &str) -> Result<(), String> {
    // Check total length (RFC 5321: max 254 chars)
    if email.len() > 254 {
        return Err("Email address is too long (max 254 characters)".to_string());
    }

    // Split into local part and domain
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err("Email must contain exactly one '@' symbol".to_string());
    }
    let (local, domain) = (parts[0], parts[1]);

    // Check local part (max 64 chars, RFC 5321)
    if local.is_empty() || local.len() > 64 {
        return Err("Local part must be 1 to 64 characters".to_string());
    }

    // Validate local part with Unicode support
    if !LOCAL_RE.is_match(local) {
        return Err("Invalid characters in local part".to_string());
    }
    if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
        return Err("Invalid dot sequence in local part".to_string());
    }

    // Convert domain to ASCII (handles IDN like приклад.укр)
    let domain_ascii =
        domain_to_ascii(domain).map_err(|_| "Invalid domain encoding".to_string())?;

    // Check domain length (max 255 chars, RFC 5321)
    if domain_ascii.is_empty() || domain_ascii.len() > 255 {
        return Err("Domain part must be 1 to 255 characters".to_string());
    }

    // Validate domain (ASCII form)
    let domain_parts: Vec<&str> = domain_ascii.split('.').collect();
    if domain_parts.len() < 2 {
        return Err("Domain must have at least one dot (e.g., example.com)".to_string());
    }

    for part in domain_parts.iter() {
        if part.is_empty() {
            return Err("Domain parts cannot be empty".to_string());
        }
        if part.starts_with('-') || part.ends_with('-') {
            return Err("Domain parts cannot start or end with a hyphen".to_string());
        }
        for c in part.chars() {
            if c.is_alphanumeric() || c == '-' {
                continue;
            }
            return Err("Invalid characters in domain".to_string());
        }
    }

    // Skip TLD length check to allow potential single-char TLDs
    // Or use tld.len() < 1 to prevent empty TLDs
    let tld = domain_parts.last().unwrap();
    if tld.is_empty() {
        return Err("Top-level domain cannot be empty".to_string());
    }

    Ok(())
}

pub(crate) struct Email {}

impl<T: StrAsRef> IValidatorRule<T> for Email {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(email) = value.as_str_ref() {
            if let Err(reason) = validate_email(email) {
                return Err(ValidationError::new_with_params(
                    "validator.email_format",
                    HashMap::from([("reason".into(), reason)]),
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
    use std::str;

    fn validator() -> Email {
        Email {}
    }

    #[test]
    fn validates_correct_email() {
        let value = "user@example.com";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_without_at_symbol() {
        let value = "user.example.com";
        let result = validator().validate(&value);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.key, "validator.email_format");
        assert!(err.params.get("reason").unwrap().contains("@"));
    }

    #[test]
    fn fails_multiple_at_symbols() {
        let value = "user@@example.com";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_long_local_part() {
        let value = &format!("{0}@domain.com", "a".repeat(65));
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_long_email() {
        let value = &format!("{0}@example.com", "a".repeat(245)); // total length > 254
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_invalid_local_characters() {
        let value = "inv@lid@domain.com";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_dot_sequence_in_local() {
        let value = "user..name@example.com";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_unicode_email_with_idn() {
        let value = "користувач@приклад.укр"; // will be punycoded
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_invalid_idn_domain() {
        let bytes = vec![0xF8, 0x88, 0x80, 0x80]; // недопустима UTF-8 послідовність
        let bad_domain = unsafe { str::from_utf8_unchecked(&bytes) };
        let value = format!("user@{bad_domain}.com");
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_domain_without_dot() {
        let value = "user@localhost";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_domain_part_starting_with_hyphen() {
        let value = "user@-invalid.com";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_domain_part_with_invalid_characters() {
        let value = "user@exa!mple.com";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_empty_tld() {
        let value = "user@example.";
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_some_valid_email() {
        let value = Some("user@mail.com".to_string());
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_option_some_invalid_email() {
        let value = Some("user.mail.com".to_string());
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
    fn validates_email_with_plus_sign() {
        let value = "test+filter@gmail.com";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }
}
