use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::value_ref::ValueRef;
use std::collections::HashMap;

pub struct MinValue<T> {
    pub min: T,
}

impl<V, T> IValidatorRule<V> for MinValue<T>
where
    V: ValueRef<Target = T>,
    T: PartialOrd + ToString + Send + Sync + 'static,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(actual) = value.value() {
            if actual < &self.min {
                return Err(ValidationError::new_with_params(
                    "validator.min_value",
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

    macro_rules! test_min_value {
        ($name:ident, $t:ty, $value:expr, $min:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let validator = MinValue { min: $min };
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $expected);
            }
        };
    }

    // ┌────────── Примітиви ──────────┐
    test_min_value!(mv_i8_ok, i8, 10, 5, true);
    test_min_value!(mv_i8_fail, i8, -5, 0, false);
    test_min_value!(mv_i16_eq, i16, 100, 100, true);
    test_min_value!(mv_i32_fail, i32, -42, -40, false);
    test_min_value!(mv_i64_ok, i64, 500, 499, true);

    test_min_value!(mv_u8_ok, u8, 1, 0, true);
    test_min_value!(mv_u16_fail, u16, 10, 11, false);
    test_min_value!(mv_u32_eq, u32, 999, 999, true);
    test_min_value!(mv_u64_fail, u64, 100, 200, false);

    test_min_value!(mv_f32_ok, f32, 3.15, 3.14, true);
    test_min_value!(mv_f64_fail, f64, 2.71, 2.72, false);

    // ┌────────── Box<T> ───────────┐
    test_min_value!(mv_box_i32_ok, Box<i32>, Box::new(10), 5, true);
    test_min_value!(mv_box_f64_fail, Box<f64>, Box::new(1.99), 2.0, false);

    // ┌────────── Rc<T> ───────────┐
    test_min_value!(mv_rc_u8_ok, Rc<u8>, Rc::new(8), 7, true);
    test_min_value!(mv_rc_i64_eq, Rc<i64>, Rc::new(777), 777, true);
    test_min_value!(mv_rc_u16_fail, Rc<u16>, Rc::new(255), 256, false);

    // ┌────────── Arc<T> ───────────┐
    test_min_value!(mv_arc_f32_ok, Arc<f32>, Arc::new(1.01), 1.0, true);
    test_min_value!(mv_arc_u32_eq, Arc<u32>, Arc::new(2024), 2024, true);
    test_min_value!(mv_arc_i16_fail, Arc<i16>, Arc::new(-1), 0, false);

    // ┌────────── Option<T> ───────────┐
    test_min_value!(mv_opt_i32_some_ok, Option<i32>, Some(42), 10, true);
    test_min_value!(mv_opt_f64_some_fail, Option<f64>, Some(1.5), 1.6, false);
    test_min_value!(mv_opt_u32_none, Option<u32>, None::<u32>, 50, true); // None → skip
}
