use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::slice_ref::SliceRef;
use std::collections::HashMap;

pub(crate) struct ExactItems {
    pub expected: usize,
}

impl<V> IValidatorRule<V> for ExactItems
where
    V: SliceRef,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(slice) = value.slice() {
            if slice.len() != self.expected {
                return Err(ValidationError::new_with_params(
                    "validator.exact_items",
                    HashMap::from([("expected".into(), self.expected.to_string())]),
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
    use std::rc::Rc;
    use std::sync::Arc;

    fn validator(expected: usize) -> ExactItems {
        ExactItems { expected }
    }

    #[test]
    fn validates_vec_with_exact_count() {
        let value = vec![1, 2, 3];
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_vec_with_too_few() {
        let value = vec![1];
        let result = validator(2).validate(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.key, "validator.exact_items");
        assert_eq!(error.params.get("expected"), Some(&"2".into()));
    }

    #[test]
    fn fails_vec_with_too_many() {
        let value = vec![1, 2, 3, 4];
        let result = validator(2).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_empty_vec_if_expected_zero() {
        let value: Vec<u8> = vec![];
        let result = validator(0).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_empty_vec_if_expected_nonzero() {
        let value: Vec<u8> = vec![];
        let result = validator(1).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_some_exact() {
        let value = Some(vec!["a", "b"]);
        let result = validator(2).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_option_some_too_short() {
        let value = Some(vec!["only"]);
        let result = validator(2).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<Vec<i32>> = None;
        let result = validator(3).validate(&value);
        assert!(result.is_ok()); // nothing to validate
    }

    #[test]
    fn validates_boxed_vec() {
        let value = Box::new(vec![1, 1, 1]);
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_ref_vec() {
        let vec = vec![1, 2];
        let value = &vec;
        let result = validator(2).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_rc_vec() {
        let value = Rc::new(vec![42]);
        let result = validator(1).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_arc_vec_exact() {
        let value = Arc::new(vec![7, 8, 9]);
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_arc_vec_overflow() {
        let value = Arc::new(vec![1, 2, 3, 4]);
        let result = validator(3).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_vec_empty_exact_zero() {
        let value: Option<Vec<i32>> = Some(vec![]);
        let result = validator(0).validate(&value);
        assert!(result.is_ok());
    }
}
