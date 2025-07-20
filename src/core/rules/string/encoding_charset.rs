use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use encoding_rs::Encoding;
use std::collections::HashMap;

pub struct EncodingCharset {
    pub charset: &'static str,
}

impl<T: StrAsRef> IValidatorRule<T> for EncodingCharset {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(str_ref) = value.as_str_ref() {
            let encoding = Encoding::for_label(self.charset.as_bytes());

            match encoding {
                Some(enc) => {
                    let (_, _, had_errors) = enc.decode(str_ref.as_bytes());
                    if had_errors {
                        return Err(ValidationError::new_with_params(
                            "validator.invalid_encoding",
                            HashMap::from([("charset".into(), self.charset.into())]),
                        ));
                    }
                    Ok(())
                }
                None => Err(ValidationError::new_with_params(
                    "validator.unknown_charset",
                    HashMap::from([("charset".into(), self.charset.into())]),
                )),
            }
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator(charset: &'static str) -> EncodingCharset {
        EncodingCharset { charset }
    }

    #[test]
    fn validates_utf8_str() {
        let value = "Привіт!";
        let result = validator("utf-8").validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_invalid_utf8_from_str() {
        // Тут спеціально використовуємо зіпсований UTF-8 через unsafe
        let bad_bytes = vec![0xFF, 0xFE, 0xFD];
        let invalid_str = unsafe { String::from_utf8_unchecked(bad_bytes) }; // небезпечно, але дозволено у тестах
        let result = validator("utf-8").validate(&invalid_str);
        assert!(result.is_err());
    }

    #[test]
    fn validates_windows_1251_like_string() {
        // Створимо строку, яка має Windows-1251 байти (але Rust бачить її як UTF-8)
        let value = "При"; // валідне UTF-8 представлення, але перевіримо як windows-1251
        let result = validator("windows-1251").validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_unknown_charset_with_string() {
        let value = "hello world";
        let result = validator("x-unknown").validate(&value);
        assert!(result.is_err());
    }
}
