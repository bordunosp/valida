use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

pub struct NotNone<T> {
    pub _phantom: PhantomData<T>,
}

impl<T> IValidatorRule<Option<T>> for NotNone<T>
where
    T: Send + Sync + 'static,
{
    fn validate(&self, value: &Option<T>) -> Result<(), ValidationError> {
        if value.is_none() {
            return Err(ValidationError::new("validator.not_none"));
        }
        Ok(())
    }
}

// ┌──────────── Box<Option<T>> ─────────────┐
impl<T> IValidatorRule<Box<Option<T>>> for NotNone<T>
where
    T: Send + Sync + 'static,
{
    fn validate(&self, value: &Box<Option<T>>) -> Result<(), ValidationError> {
        if value.is_none() {
            return Err(ValidationError::new("validator.not_none"));
        }
        Ok(())
    }
}

// ┌──────────── Rc<Option<T>> ─────────────┐
impl<T> IValidatorRule<Rc<Option<T>>> for NotNone<T>
where
    T: Send + Sync + 'static,
{
    fn validate(&self, value: &Rc<Option<T>>) -> Result<(), ValidationError> {
        if value.is_none() {
            return Err(ValidationError::new("validator.not_none"));
        }
        Ok(())
    }
}

// ┌──────────── Arc<Option<T>> ─────────────┐
impl<T> IValidatorRule<Arc<Option<T>>> for NotNone<T>
where
    T: Send + Sync + 'static,
{
    fn validate(&self, value: &Arc<Option<T>>) -> Result<(), ValidationError> {
        if value.is_none() {
            return Err(ValidationError::new("validator.not_none"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;
    use std::collections::HashMap;
    use std::marker::PhantomData;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct CustomType {
        id: i32,
        name: String,
    }

    macro_rules! test_not_none {
        ($name:ident, $value:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let validator = NotNone::<_> {
                    _phantom: PhantomData,
                };
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $expected);
            }
        };
    }

    // ┌────────── Примітиви ──────────┐
    test_not_none!(nn_some_i32, Some(5), true);
    test_not_none!(nn_none_i32, None::<i32>, false);

    // ┌────────── Строки ──────────┐
    test_not_none!(nn_some_string, Some(String::from("hello")), true);
    test_not_none!(nn_none_string, None::<String>, false);

    // ┌────────── Колекції ──────────┐
    test_not_none!(nn_some_vec, Some(vec![1, 2, 3]), true);
    test_not_none!(nn_none_vec, None::<Vec<i32>>, false);

    test_not_none!(
        nn_some_map,
        {
            let mut map = HashMap::new();
            map.insert("key", "value");
            Some(map)
        },
        true
    );

    test_not_none!(nn_none_map, None::<HashMap<String, String>>, false);

    // ┌────────── Власні структури ──────────┐
    test_not_none!(
        nn_some_custom,
        {
            Some(CustomType {
                id: 1,
                name: "test".into(),
            })
        },
        true
    );

    test_not_none!(nn_none_custom, None::<CustomType>, false);

    // ┌────────── Edge-case ──────────┐
    test_not_none!(nn_none_unit, None::<()>, false);

    #[test]
    fn nn_box_none_fails() {
        let validator = NotNone::<i32> {
            _phantom: PhantomData,
        };
        let value = Box::new(None::<i32>);
        assert!(validator.validate(&value).is_err());
    }

    #[test]
    fn nn_rc_some_passes() {
        let validator = NotNone::<String> {
            _phantom: PhantomData,
        };
        let value = Rc::new(Some(String::from("abc")));
        assert!(validator.validate(&value).is_ok());
    }

    #[test]
    fn nn_arc_none_fails() {
        let validator = NotNone::<Vec<i32>> {
            _phantom: PhantomData,
        };
        let value = Arc::new(None::<Vec<i32>>);
        assert!(validator.validate(&value).is_err());
    }
}
