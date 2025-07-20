use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::value_ref::ValueRef;
use std::collections::HashMap;

pub struct GreaterThan<T> {
    pub min: T,
}

impl<V, T> IValidatorRule<V> for GreaterThan<T>
where
    V: ValueRef<Target = T>,
    T: PartialOrd + ToString + Send + Sync + 'static,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(actual) = value.value() {
            if actual <= &self.min {
                return Err(ValidationError::new_with_params(
                    "validator.greater_than",
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

    macro_rules! test_greater_than {
        ($name:ident, $t:ty, $value:expr, $min:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let validator = GreaterThan { min: $min };
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $expected);
            }
        };
    }

    // ┌────────────── Primitive types ──────────────┐
    test_greater_than!(gt_i8_ok, i8, 10, 5, true);
    test_greater_than!(gt_i8_fail_eq, i8, 5, 5, false);
    test_greater_than!(gt_i16_fail_less, i16, -1, 0, false);
    test_greater_than!(gt_i32_ok, i32, 100, 99, true);
    test_greater_than!(gt_i64_fail_eq, i64, 42, 42, false);

    test_greater_than!(gt_u8_ok, u8, 200, 199, true);
    test_greater_than!(gt_u16_fail, u16, 0, 1, false);
    test_greater_than!(gt_u32_ok, u32, 5, 2, true);
    test_greater_than!(gt_u64_fail_eq, u64, 1000, 1000, false);

    test_greater_than!(gt_f32_ok, f32, 1.5, 1.4, true);
    test_greater_than!(gt_f64_fail, f64, 0.0, 0.1, false);

    // ┌────────────── Box<T> ──────────────┐
    test_greater_than!(gt_box_i32_ok, Box<i32>, Box::new(7), 5, true);
    test_greater_than!(gt_box_f64_fail_eq, Box<f64>, Box::new(3.3), 3.3, false);

    // ┌────────────── Rc<T> ──────────────┐
    test_greater_than!(gt_rc_u8_ok, Rc<u8>, Rc::new(10), 1, true);
    test_greater_than!(gt_rc_i64_fail, Rc<i64>, Rc::new(-5), -1, false);

    // ┌────────────── Arc<T> ──────────────┐
    test_greater_than!(gt_arc_f32_ok, Arc<f32>, Arc::new(9.9), 8.0, true);
    test_greater_than!(gt_arc_i16_fail_eq, Arc<i16>, Arc::new(5), 5, false);

    // ┌────────────── Option<T> ──────────────┐
    test_greater_than!(gt_opt_i32_some_ok, Option<i32>, Some(100), 99, true);
    test_greater_than!(gt_opt_f64_some_fail_eq, Option<f64>, Some(0.5), 0.5, false);
    test_greater_than!(gt_opt_u32_none, Option<u32>, None::<u32>, 1, true);
}
