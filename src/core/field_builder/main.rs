use crate::core::builder::{FieldRules, RulesBuilder};
use std::error::Error;

pub struct FieldBuilder<'a, T, V, E>
where
    E: Error + Send + Sync + 'static,
{
    pub rules_builder: &'a mut RulesBuilder<T, E>,
    pub rules: FieldRules<T, V, E>,
}

impl<'a, T, V, E> FieldBuilder<'a, T, V, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn build(self) {
        self.rules_builder
            .fields
            .insert(self.rules.field_name.to_string(), Box::new(self.rules));
    }
}
