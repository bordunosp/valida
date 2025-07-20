use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::slice_ref::SliceRef;
use std::collections::HashMap;

pub(crate) struct MinItems {
    pub min: usize,
}

impl<V> IValidatorRule<V> for MinItems
where
    V: SliceRef,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(slice) = value.slice() {
            if slice.len() < self.min {
                return Err(ValidationError::new_with_params(
                    "validator.min_items",
                    HashMap::from([("min".into(), self.min.to_string())]),
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

    fn validator(min: usize) -> MinItems {
        MinItems { min }
    }

    #[test]
    fn validates_vec_with_enough_items() {
        let value = vec![1, 2, 3];
        let result = validator(2).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_vec_with_too_few_items() {
        let value = vec![42];
        let result = validator(3).validate(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.key, "validator.min_items");
        assert_eq!(error.params.get("min"), Some(&"3".into()));
    }

    #[test]
    fn validates_exact_item_count() {
        let value = vec!["a", "b"];
        let result = validator(2).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_empty_vec_if_min_zero() {
        let value: Vec<u8> = vec![];
        let result = validator(0).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_empty_vec_if_min_nonzero() {
        let value: Vec<&str> = vec![];
        let result = validator(1).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_some_vec_ok() {
        let value = Some(vec![1, 2, 3]);
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_option_some_vec_short() {
        let value = Some(vec![1]);
        let result = validator(2).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<Vec<u32>> = None;
        let result = validator(5).validate(&value);
        assert!(result.is_ok()); // nothing to validate
    }

    #[test]
    fn validates_boxed_vec() {
        let value = Box::new(vec![10, 20]);
        let result = validator(2).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_ref_vec() {
        let vec = vec![7, 8, 9];
        let result = validator(2).validate(&&vec);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_rc_vec() {
        let value = Rc::new(vec![1, 2, 3, 4]);
        let result = validator(4).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_arc_vec() {
        let value = Arc::new(vec![1, 2]);
        let result = validator(2).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_arc_vec_short() {
        let value = Arc::new(vec![1]);
        let result = validator(2).validate(&value);
        assert!(result.is_err());
    }

    #[test]
    fn validates_option_vec_empty_with_min_zero() {
        let value = Some(Vec::<i32>::new());
        let result = validator(0).validate(&value);
        assert!(result.is_ok());
    }
}
