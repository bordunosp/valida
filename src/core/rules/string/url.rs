use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use url;

pub(crate) struct UrlValid {}

impl<T: StrAsRef> IValidatorRule<T> for UrlValid {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            if url::Url::parse(s).is_err() {
                return Err(ValidationError::new("validator.url"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator() -> UrlValid {
        UrlValid {}
    }

    #[test]
    fn validates_http_url() {
        let value = "http://example.com";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_https_url() {
        let value = "https://example.org/page?query=1";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_ftp_url() {
        let value = "ftp://files.example.net";
        let result = validator().validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_missing_scheme() {
        let value = "example.com"; // no scheme
        let result = validator().validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn fails_invalid_url() {
        let value = "http:::/malformed_url";
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
