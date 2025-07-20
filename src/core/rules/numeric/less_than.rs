use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::value_ref::ValueRef;
use std::collections::HashMap;

pub struct LessThan<T> {
    pub max: T,
}

impl<V, T> IValidatorRule<V> for LessThan<T>
where
    V: ValueRef<Target = T>,
    T: PartialOrd + ToString + Send + Sync + 'static,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(actual) = value.value() {
            if actual >= &self.max {
                return Err(ValidationError::new_with_params(
                    "validator.less_than",
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

    macro_rules! test_less_than {
        ($name:ident, $t:ty, $value:expr, $max:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let validator = LessThan { max: $max };
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $expected);
            }
        };
    }

    // ┌────────── Primitive types ─────────┐
    test_less_than!(lt_i8_ok, i8, 3, 5, true);
    test_less_than!(lt_i8_fail_eq, i8, 5, 5, false);
    test_less_than!(lt_i16_fail, i16, 10, 9, false);
    test_less_than!(lt_i32_ok, i32, -1, 0, true);
    test_less_than!(lt_i64_fail_eq, i64, 100, 100, false);

    test_less_than!(lt_u8_ok, u8, 5, 10, true);
    test_less_than!(lt_u16_fail, u16, 1000, 999, false);
    test_less_than!(lt_u32_ok, u32, 0, 1, true);
    test_less_than!(lt_u64_fail_eq, u64, 9999, 9999, false);

    test_less_than!(lt_f32_ok, f32, 3.1, 3.14, true);
    test_less_than!(lt_f64_fail, f64, 2.71, 2.70, false);

    // ┌──────────── Box<T> ─────────────┐
    test_less_than!(lt_box_i32_ok, Box<i32>, Box::new(1), 2, true);
    test_less_than!(lt_box_u64_fail_eq, Box<u64>, Box::new(100), 100, false);

    // ┌──────────── Rc<T> ─────────────┐
    test_less_than!(lt_rc_u8_ok, Rc<u8>, Rc::new(42), 100, true);
    test_less_than!(lt_rc_i64_fail, Rc<i64>, Rc::new(500), 100, false);

    // ┌──────────── Arc<T> ─────────────┐
    test_less_than!(lt_arc_f32_ok, Arc<f32>, Arc::new(1.618), 3.14, true);
    test_less_than!(lt_arc_i16_fail_eq, Arc<i16>, Arc::new(8), 8, false);

    // ┌──────────── Option<T> ─────────────┐
    test_less_than!(lt_opt_i32_some_ok, Option<i32>, Some(30), 50, true);
    test_less_than!(lt_opt_f64_some_fail, Option<f64>, Some(2.0), 1.5, false);
    test_less_than!(lt_opt_u32_none, Option<u32>, None::<u32>, 100, true); // None → skip
}
