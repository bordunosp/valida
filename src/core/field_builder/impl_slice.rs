use crate::core::contract::IValidatorRule;
use crate::core::field_builder::main::FieldBuilder;
use crate::core::primitive::PrimitiveRule;
use crate::core::rules::slice::each_rule::EachRule;
use crate::core::rules::slice::exact_items::ExactItems;
use crate::core::rules::slice::max_items::MaxItems;
use crate::core::rules::slice::min_items::MinItems;
use crate::core::rules::slice_ref::SliceRef;
use std::error::Error;

impl<'a, T, V, E> FieldBuilder<'a, T, V, E>
where
    V: SliceRef + Send + Sync + std::fmt::Debug + 'static,
    V::Item: Send + Sync + PrimitiveRule + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn each<R>(mut self, rule: R) -> Self
    where
        R: IValidatorRule<V::Item> + Send + Sync + 'static,
    {
        self.rules.rules.push(Box::new(EachRule { rule }));
        self
    }
}

impl<'a, T, V, E> FieldBuilder<'a, T, V, E>
where
    V: SliceRef,
    V: Send + Sync + std::fmt::Debug + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn exact_items(mut self, expected: usize) -> Self {
        self.rules.rules.push(Box::new(ExactItems { expected }));
        self
    }

    pub fn min_items(mut self, min: usize) -> Self {
        self.rules.rules.push(Box::new(MinItems { min }));
        self
    }

    pub fn max_items(mut self, max: usize) -> Self {
        self.rules.rules.push(Box::new(MaxItems { max }));
        self
    }
}
