use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::slice_ref::SliceRef;
use std::collections::HashMap;

pub(crate) struct MaxItems {
    pub max: usize,
}

impl<V> IValidatorRule<V> for MaxItems
where
    V: SliceRef,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(slice) = value.slice() {
            if slice.len() > self.max {
                return Err(ValidationError::new_with_params(
                    "validator.max_items",
                    HashMap::from([("max".into(), self.max.to_string())]),
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

    fn validator(max: usize) -> MaxItems {
        MaxItems { max }
    }

    #[test]
    fn validates_vec_within_limit() {
        let value = vec![1, 2];
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_vec_exact_limit() {
        let value = vec![10, 20];
        let result = validator(2).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_vec_exceeds_limit() {
        let value = vec![42, 99, 77];
        let result = validator(2).validate(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.key, "validator.max_items");
        assert_eq!(error.params.get("max"), Some(&"2".into()));
    }

    #[test]
    fn validates_empty_vec() {
        let value: Vec<u8> = vec![];
        let result = validator(0).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_option_some_valid() {
        let value = Some(vec![1, 2]);
        let result = validator(2).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_option_some_exceeds() {
        let value = Some(vec![1, 2, 3, 4]);
        let result = validator(3).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<Vec<String>> = None;
        let result = validator(5).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_boxed_vec_ok() {
        let value = Box::new(vec!["a", "b", "c"]);
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_boxed_vec_exceeds() {
        let value = Box::new(vec!["x"; 5]);
        let result = validator(3).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_rc_vec() {
        let value = Rc::new(vec![1, 2]);
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_arc_vec_exceeds() {
        let value = Arc::new(vec![0; 10]);
        let result = validator(5).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_ref_vec() {
        let vec = vec![1, 2, 3];
        let value = &vec;
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn slice_empty_option_vec_with_zero_max() {
        let value: Option<Vec<u8>> = Some(vec![]);
        let result = validator(0).validate(&value);
        assert!(result.is_ok());
    }
}
