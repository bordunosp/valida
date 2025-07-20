use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::value_ref::ValueRef;
use num_traits::Zero;

pub struct Negative {}

impl<V, T> IValidatorRule<V> for Negative
where
    V: ValueRef<Target = T>,
    T: PartialOrd + Zero + ToString + Send + Sync + 'static,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(actual) = value.value() {
            if actual >= &T::zero() {
                return Err(ValidationError::new("validator.negative"));
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

    macro_rules! test_negative {
        ($name:ident, $t:ty, $value:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let validator = Negative {};
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $expected);
            }
        };
    }

    // ┌────────────── Primitive types ──────────────┐
    test_negative!(neg_i8_ok, i8, -1, true);
    test_negative!(neg_i8_fail_zero, i8, 0, false);
    test_negative!(neg_i32_fail_pos, i32, 100, false);

    test_negative!(neg_f32_ok, f32, -0.001, true);
    test_negative!(neg_f64_fail_zero, f64, 0.0, false);

    // ┌────────────── Box<T> ──────────────┐
    test_negative!(neg_box_i64_ok, Box<i64>, Box::new(-999), true);
    test_negative!(neg_box_u32_fail, Box<u32>, Box::new(10), false);

    // ┌────────────── Rc<T> ──────────────┐
    test_negative!(neg_rc_i16_ok, Rc<i16>, Rc::new(-10), true);
    test_negative!(neg_rc_i16_fail_zero, Rc<i16>, Rc::new(0), false);

    // ┌────────────── Arc<T> ──────────────┐
    test_negative!(neg_arc_f64_ok, Arc<f64>, Arc::new(-5.55), true);
    test_negative!(neg_arc_u16_fail, Arc<u16>, Arc::new(1), false);

    // ┌────────────── Option<T> ──────────────┐
    test_negative!(neg_opt_i32_some_ok, Option<i32>, Some(-1), true);
    test_negative!(neg_opt_f32_some_fail_zero, Option<f32>, Some(0.0), false);
    test_negative!(neg_opt_u32_none, Option<u32>, None::<u32>, true);
}
