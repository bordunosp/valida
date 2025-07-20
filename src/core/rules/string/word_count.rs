use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use std::collections::HashMap;

pub(crate) struct WordCount {
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl<T: StrAsRef> IValidatorRule<T> for WordCount {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            let count = s.split_whitespace().count();

            if let Some(min) = self.min {
                if count < min {
                    return Err(ValidationError::new_with_params(
                        "validator.word_count.too_few",
                        HashMap::from([("min".into(), min.to_string())]),
                    ));
                }
            }

            if let Some(max) = self.max {
                if count > max {
                    return Err(ValidationError::new_with_params(
                        "validator.word_count.too_many",
                        HashMap::from([("max".into(), max.to_string())]),
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator(min: Option<usize>, max: Option<usize>) -> WordCount {
        WordCount { min, max }
    }

    #[test]
    fn passes_within_range() {
        let value = "this is valid";
        let result = validator(Some(2), Some(4)).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_below_min() {
        let value = "only one";
        let result = validator(Some(3), None).validate(&value);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.key, "validator.word_count.too_few");
    }

    #[test]
    fn fails_above_max() {
        let value = "this line has too many words";
        let result = validator(None, Some(4)).validate(&value);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.key, "validator.word_count.too_many");
    }

    #[test]
    fn validates_exact_min_and_max() {
        let value = "one two three";
        let result = validator(Some(3), Some(3)).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_empty_string_with_no_min() {
        let value = "";
        let result = validator(None, Some(0)).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        let result = validator(Some(1), Some(5)).validate(&value);
        assert!(result.is_ok());
    }
}
