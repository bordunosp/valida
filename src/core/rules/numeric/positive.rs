use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::value_ref::ValueRef;
use num_traits::Zero;

pub struct Positive {}

impl<V, T> IValidatorRule<V> for Positive
where
    V: ValueRef<Target = T>,
    T: PartialOrd + Zero + ToString + Send + Sync + 'static,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(actual) = value.value() {
            if actual <= &T::zero() {
                return Err(ValidationError::new("validator.positive"));
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

    macro_rules! test_positive {
        ($name:ident, $t:ty, $value:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let validator = Positive {};
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $expected);
            }
        };
    }

    // ┌────────────── Primitive types ──────────────┐
    test_positive!(pos_i8_ok, i8, 1, true);
    test_positive!(pos_i8_fail_zero, i8, 0, false);
    test_positive!(pos_i16_fail_neg, i16, -10, false);
    test_positive!(pos_i32_ok, i32, 100, true);
    test_positive!(pos_u8_ok, u8, 200, true);
    test_positive!(pos_u16_fail_zero, u16, 0, false);

    test_positive!(pos_f32_ok, f32, 0.1, true);
    test_positive!(pos_f64_fail_zero, f64, 0.0, false);

    // ┌────────────── Box<T> ──────────────┐
    test_positive!(pos_box_i32_ok, Box<i32>, Box::new(3), true);
    test_positive!(pos_box_f64_fail_neg, Box<f64>, Box::new(-0.5), false);

    // ┌────────────── Rc<T> ──────────────┐
    test_positive!(pos_rc_u8_ok, Rc<u8>, Rc::new(1), true);
    test_positive!(pos_rc_i64_fail_zero, Rc<i64>, Rc::new(0), false);

    // ┌────────────── Arc<T> ──────────────┐
    test_positive!(pos_arc_f32_ok, Arc<f32>, Arc::new(2.5), true);
    test_positive!(pos_arc_i16_fail_neg, Arc<i16>, Arc::new(-1), false);

    // ┌────────────── Option<T> ──────────────┐
    test_positive!(pos_opt_i32_some_ok, Option<i32>, Some(5), true);
    test_positive!(pos_opt_f64_some_fail_neg, Option<f64>, Some(-0.1), false);
    test_positive!(pos_opt_u32_none, Option<u32>, None::<u32>, true);
}
