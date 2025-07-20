use crate::core::field_builder::main::FieldBuilder;
use crate::core::rules::numeric::greater_than::GreaterThan;
use crate::core::rules::numeric::less_than::LessThan;
use crate::core::rules::numeric::max_value::MaxValue;
use crate::core::rules::numeric::min_value::MinValue;
use crate::core::rules::numeric::negative::Negative;
use crate::core::rules::numeric::negative_or_zero::NegativeOrZero;
use crate::core::rules::numeric::positive::Positive;
use crate::core::rules::numeric::positive_or_zero::PositiveOrZero;
use crate::core::rules::numeric::range::Range;
use crate::core::rules::value_ref::ValueRef;
use std::error::Error;

impl<'a, T, V, E> FieldBuilder<'a, T, V, E>
where
    V: ValueRef,
    V::Target: PartialOrd + ToString + Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn greater_than(mut self, min: V::Target) -> Self {
        self.rules.rules.push(Box::new(GreaterThan { min }));
        self
    }

    pub fn less_than(mut self, max: V::Target) -> Self {
        self.rules.rules.push(Box::new(LessThan { max }));
        self
    }

    pub fn max_value(mut self, max: V::Target) -> Self {
        self.rules.rules.push(Box::new(MaxValue { max }));
        self
    }

    pub fn min_value(mut self, min: V::Target) -> Self {
        self.rules.rules.push(Box::new(MinValue { min }));
        self
    }

    pub fn max(mut self, value: V::Target) -> Self {
        self.rules.rules.push(Box::new(MaxValue { max: value }));
        self
    }

    pub fn min(mut self, value: V::Target) -> Self {
        self.rules.rules.push(Box::new(MinValue { min: value }));
        self
    }

    pub fn range(mut self, min: V::Target, max: V::Target) -> Self {
        self.rules.rules.push(Box::new(Range { min, max }));
        self
    }
}

impl<'a, T, V, E> FieldBuilder<'a, T, V, E>
where
    V: ValueRef,
    V::Target: num_traits::Zero + PartialOrd + ToString + Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn negative(mut self) -> Self {
        self.rules.rules.push(Box::new(Negative {}));
        self
    }

    pub fn negative_or_zero(mut self) -> Self {
        self.rules.rules.push(Box::new(NegativeOrZero {}));
        self
    }

    pub fn positive(mut self) -> Self {
        self.rules.rules.push(Box::new(Positive {}));
        self
    }

    pub fn positive_or_zero(mut self) -> Self {
        self.rules.rules.push(Box::new(PositiveOrZero {}));
        self
    }
}
