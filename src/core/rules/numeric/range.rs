use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::value_ref::ValueRef;
use std::collections::HashMap;

pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl<V, T> IValidatorRule<V> for Range<T>
where
    V: ValueRef<Target = T>,
    T: PartialOrd + ToString + Send + Sync + 'static,
{
    fn validate(&self, value: &V) -> Result<(), ValidationError> {
        if let Some(actual) = value.value() {
            if actual < &self.min || actual > &self.max {
                return Err(ValidationError::new_with_params(
                    "validator.range",
                    HashMap::from([
                        ("min".into(), self.min.to_string()),
                        ("max".into(), self.max.to_string()),
                    ]),
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

    macro_rules! test_range {
        ($name:ident, $t:ty, $value:expr, $min:expr, $max:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let validator = Range {
                    min: $min,
                    max: $max,
                };
                let result = validator.validate(&$value);
                assert_eq!(result.is_ok(), $expected);
            }
        };
    }

    // ┌────────── Примітиви ──────────┐
    test_range!(rg_i8_inside, i8, 5, 0, 10, true);
    test_range!(rg_i8_below, i8, -1, 0, 10, false);
    test_range!(rg_i8_above, i8, 11, 0, 10, false);
    test_range!(rg_i8_eq_min, i8, 0, 0, 10, true);
    test_range!(rg_i8_eq_max, i8, 10, 0, 10, true);

    test_range!(rg_u16_inside, u16, 50, 10, 100, true);
    test_range!(rg_u16_below, u16, 5, 10, 100, false);
    test_range!(rg_u16_above, u16, 101, 10, 100, false);

    test_range!(rg_f32_inside, f32, 1.5, 1.0, 2.0, true);
    test_range!(rg_f32_fail_below, f32, 0.9, 1.0, 2.0, false);
    test_range!(rg_f32_fail_above, f32, 2.1, 1.0, 2.0, false);

    // ┌────────── Box<T> ──────────┐
    test_range!(rg_box_i32_ok, Box<i32>, Box::new(42), 40, 45, true);
    test_range!(
        rg_box_f64_fail,
        Box<f64>,
        Box::new(100.0),
        10.0,
        99.9,
        false
    );

    // ┌────────── Rc<T> ──────────┐
    test_range!(rg_rc_u8_ok, Rc<u8>, Rc::new(3), 1, 5, true);
    test_range!(rg_rc_i64_fail, Rc<i64>, Rc::new(2000), 0, 1000, false);

    // ┌────────── Arc<T> ──────────┐
    test_range!(rg_arc_f64_eq, Arc<f64>, Arc::new(2.5), 2.5, 2.5, true);
    test_range!(rg_arc_i16_below, Arc<i16>, Arc::new(-100), -99, 0, false);

    // ┌────────── Option<T> ──────────┐
    test_range!(rg_opt_i32_some_ok, Option<i32>, Some(25), 0, 30, true);
    test_range!(
        rg_opt_u32_some_fail,
        Option<u32>,
        Some(999),
        100,
        900,
        false
    );
    test_range!(rg_opt_f32_none, Option<f32>, None::<f32>, 0.0, 1.0, true); // skips
}
