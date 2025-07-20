use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::value_ref::ValueRef;
use num_traits::Zero;

pub struct NegativeOrZero {}

impl<V, T> IValidatorRule<V> for NegativeOrZero
where
    V: ValueRef<Target = T>,
    T: PartialOrd + Zero + ToString + Send + Sync + 'static,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(actual) = value.value() {
            if actual > &T::zero() {
                return Err(ValidationError::new("validator.negative_or_zero"));
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

    macro_rules! test_negative_or_zero {
        ($name:ident, $t:ty, $value:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let validator = NegativeOrZero {};
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $expected);
            }
        };
    }

    // ┌────────────── Primitive types ──────────────┐
    test_negative_or_zero!(noz_i8_ok_zero, i8, 0, true);
    test_negative_or_zero!(noz_i8_ok_neg, i8, -5, true);
    test_negative_or_zero!(noz_i32_fail_pos, i32, 10, false);

    test_negative_or_zero!(noz_f32_ok_neg, f32, -0.1, true);
    test_negative_or_zero!(noz_f64_fail_pos, f64, 0.0001, false);

    // ┌────────────── Box<T> ──────────────┐
    test_negative_or_zero!(noz_box_i32_zero, Box<i32>, Box::new(0), true);
    test_negative_or_zero!(noz_box_u32_fail_pos, Box<u32>, Box::new(1), false);

    // ┌────────────── Rc<T> ──────────────┐
    test_negative_or_zero!(noz_rc_i16_neg, Rc<i16>, Rc::new(-100), true);
    test_negative_or_zero!(noz_rc_u8_fail_pos, Rc<u8>, Rc::new(5), false);

    // ┌────────────── Arc<T> ──────────────┐
    test_negative_or_zero!(noz_arc_f64_zero, Arc<f64>, Arc::new(0.0), true);
    test_negative_or_zero!(noz_arc_i64_fail_pos, Arc<i64>, Arc::new(1), false);

    // ┌────────────── Option<T> ──────────────┐
    test_negative_or_zero!(noz_opt_i32_some_zero, Option<i32>, Some(0), true);
    test_negative_or_zero!(noz_opt_f64_some_neg, Option<f64>, Some(-3.14), true);
    test_negative_or_zero!(noz_opt_u32_some_pos, Option<u32>, Some(42), false);
    test_negative_or_zero!(noz_opt_u64_none, Option<u64>, None::<u64>, true);
}
