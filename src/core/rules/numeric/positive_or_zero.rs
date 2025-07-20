use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::value_ref::ValueRef;
use num_traits::Zero;

pub struct PositiveOrZero {}

impl<V, T> IValidatorRule<V> for PositiveOrZero
where
    V: ValueRef<Target = T>,
    T: PartialOrd + Zero + ToString + Send + Sync + 'static,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(actual) = value.value() {
            if actual < &T::zero() {
                return Err(ValidationError::new("validator.positive_or_zero"));
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

    macro_rules! test_positive_or_zero {
        ($name:ident, $t:ty, $value:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let validator = PositiveOrZero {};
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $expected);
            }
        };
    }

    // ┌────────────── Primitive types ──────────────┐
    test_positive_or_zero!(poz_i8_ok_pos, i8, 7, true);
    test_positive_or_zero!(poz_i8_ok_zero, i8, 0, true);
    test_positive_or_zero!(poz_i8_fail_neg, i8, -1, false);

    test_positive_or_zero!(poz_u16_zero, u16, 0, true);
    test_positive_or_zero!(poz_u32_positive, u32, 10, true);

    test_positive_or_zero!(poz_f32_ok_zero, f32, 0.0, true);
    test_positive_or_zero!(poz_f64_fail_neg, f64, -0.0001, false);

    // ┌────────────── Box<T> ──────────────┐
    test_positive_or_zero!(poz_box_i32_ok, Box<i32>, Box::new(0), true);
    test_positive_or_zero!(poz_box_f64_fail_neg, Box<f64>, Box::new(-3.3), false);

    // ┌────────────── Rc<T> ──────────────┐
    test_positive_or_zero!(poz_rc_u8_ok, Rc<u8>, Rc::new(255), true);
    test_positive_or_zero!(poz_rc_i64_fail_neg, Rc<i64>, Rc::new(-42), false);

    // ┌────────────── Arc<T> ──────────────┐
    test_positive_or_zero!(poz_arc_f32_zero, Arc<f32>, Arc::new(0.0), true);
    test_positive_or_zero!(poz_arc_i16_fail, Arc<i16>, Arc::new(-10), false);

    // ┌────────────── Option<T> ──────────────┐
    test_positive_or_zero!(poz_opt_i32_some_zero, Option<i32>, Some(0), true);
    test_positive_or_zero!(poz_opt_f64_some_neg, Option<f64>, Some(-1.0), false);
    test_positive_or_zero!(poz_opt_u32_none, Option<u32>, None::<u32>, true);
}
