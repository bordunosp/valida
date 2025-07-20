use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::primitive::PrimitiveRule;
use crate::core::rules::slice_ref::SliceRef;

pub struct EachRule<R> {
    pub rule: R,
}

impl<V, R> IValidatorRule<V> for EachRule<R>
where
    V: SliceRef,
    V::Item: PrimitiveRule,
    R: IValidatorRule<V::Item>,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(slice) = value.slice() {
            for (i, item) in slice.iter().enumerate() {
                if let Err(mut err) = self.rule.validate(item) {
                    err.params.insert("index".into(), i.to_string());
                    return Err(err);
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
    use crate::core::rules::string::max_length::MaxLength;

    fn validator(max: usize) -> EachRule<MaxLength> {
        EachRule {
            rule: MaxLength { max },
        }
    }

    #[test]
    fn validates_all_items_successfully() {
        let value = vec!["red", "green", "blue"];
        let result = validator(10).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_on_excessive_length_item() {
        let value = vec!["ok", "this-is-too-long"];
        let result = validator(5).validate(&value);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.key, "validator.max_length");
        assert_eq!(err.params.get("index"), Some(&"1".into()));
        assert_eq!(err.params.get("max"), Some(&"5".into()));
    }

    #[test]
    fn validates_empty_vec() {
        let value: Vec<&str> = vec![];
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<Vec<String>> = None;
        let result = validator(12).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_boxed_vec() {
        let value = Box::new(vec!["a", "b"]);
        let result = validator(2).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_rc_vec() {
        use std::rc::Rc;
        let value = Rc::new(vec!["one", "two"]);
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn fails_arc_vec_over_limit() {
        use std::sync::Arc;
        let value = Arc::new(vec!["short", "way-too-long"]);
        let result = validator(6).validate(&value);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.params.get("index"), Some(&"1".into()));
    }

    #[test]
    fn validates_ref_vec() {
        let vec = vec!["yes", "no"];
        let value = &vec;
        let result = validator(3).validate(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn validates_option_vec_empty() {
        let value: Option<Vec<String>> = Some(vec![]);
        let result = validator(10).validate(&value);
        assert!(result.is_ok());
    }
}
