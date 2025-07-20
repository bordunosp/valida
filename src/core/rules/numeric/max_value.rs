use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::value_ref::ValueRef;
use std::collections::HashMap;

pub struct MaxValue<T> {
    pub max: T,
}

impl<V, T> IValidatorRule<V> for MaxValue<T>
where
    V: ValueRef<Target = T>,
    T: PartialOrd + ToString + Send + Sync + 'static,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(actual) = value.value() {
            if actual > &self.max {
                return Err(ValidationError::new_with_params(
                    "validator.max_value",
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

    macro_rules! test_max_value {
        ($name:ident, $t:ty, $value:expr, $max:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let validator = MaxValue { max: $max };
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $expected);
            }
        };
    }

    // ┌────────── Примітиви ──────────┐
    test_max_value!(mv_i8_ok, i8, 5, 10, true);
    test_max_value!(mv_i8_fail, i8, 15, 10, false);
    test_max_value!(mv_i16_eq, i16, 100, 100, true);
    test_max_value!(mv_i32_fail, i32, 999, 500, false);
    test_max_value!(mv_i64_ok, i64, 1000, 1001, true);

    test_max_value!(mv_u8_ok, u8, 42, 42, true);
    test_max_value!(mv_u16_fail, u16, 300, 299, false);
    test_max_value!(mv_u32_ok, u32, 0, 1, true);
    test_max_value!(mv_u64_eq, u64, 9999, 9999, true);

    test_max_value!(mv_f32_ok, f32, 3.14, 3.14, true);
    test_max_value!(mv_f64_fail, f64, 9.81, 9.80, false);

    // ┌────────── Box<T> ───────────┐
    test_max_value!(mv_box_i32_ok, Box<i32>, Box::new(50), 100, true);
    test_max_value!(mv_box_f64_fail, Box<f64>, Box::new(3.1416), 3.14, false);

    // ┌────────── Rc<T> ───────────┐
    test_max_value!(mv_rc_u8_ok, Rc<u8>, Rc::new(5), 10, true);
    test_max_value!(mv_rc_i64_eq, Rc<i64>, Rc::new(777), 777, true);
    test_max_value!(mv_rc_u16_fail, Rc<u16>, Rc::new(256), 255, false);

    // ┌────────── Arc<T> ───────────┐
    test_max_value!(mv_arc_f32_ok, Arc<f32>, Arc::new(1.0), 1.01, true);
    test_max_value!(mv_arc_u32_eq, Arc<u32>, Arc::new(2024), 2024, true);
    test_max_value!(mv_arc_i16_fail, Arc<i16>, Arc::new(128), 127, false);

    // ┌────────── Option<T> ───────────┐
    test_max_value!(mv_opt_i32_some_ok, Option<i32>, Some(42), 99, true);
    test_max_value!(mv_opt_f64_some_fail, Option<f64>, Some(1.618), 1.0, false);
    test_max_value!(mv_opt_u32_none, Option<u32>, None::<u32>, 100, true); // None → skip
}
