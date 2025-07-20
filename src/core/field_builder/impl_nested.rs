use crate::core::contract::IValidate;
use crate::core::field_builder::main::FieldBuilder;
use crate::core::nested_wrapper::{
    NestedArcOptionValidatorWrapper, NestedArcValidatorWrapper, NestedMapValidatorWrapper,
    NestedOptionValidatorWrapper, NestedValidatorWrapper, NestedVecValidatorWrapper,
};
use crate::core::rules::nested::NestedField;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;

impl<'a, T, V, E> NestedField<V, E> for FieldBuilder<'a, T, V, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn nested<TValidator>(self, validator: TValidator) -> Self
    where
        TValidator: IValidate<V, E> + 'static,
    {
        let wrapper = Box::new(NestedValidatorWrapper::<T, V, E> {
            field_name: self.rules.field_name,
            accessor: self.rules.accessor.clone(),
            inner: Box::new(validator),
            _phantom: PhantomData,
        });

        self.rules_builder
            .nested
            .insert(self.rules.field_name.to_string(), wrapper);

        self
    }
}

impl<'a, T, V, E> NestedField<V, E> for FieldBuilder<'a, T, Vec<V>, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn nested<TValidator>(self, validator: TValidator) -> Self
    where
        TValidator: IValidate<V, E> + 'static,
    {
        let wrapper = Box::new(NestedVecValidatorWrapper::<T, V, E> {
            field_name: self.rules.field_name,
            accessor: self.rules.accessor.clone(),
            inner: Box::new(validator),
            _phantom: PhantomData,
        });

        self.rules_builder
            .nested
            .insert(self.rules.field_name.to_string(), wrapper);

        self
    }
}

impl<'a, T, K, V, E> NestedField<V, E> for FieldBuilder<'a, T, HashMap<K, V>, E>
where
    T: Send + Sync + 'static,
    K: Eq + Hash + ToString + Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn nested<TValidator>(self, validator: TValidator) -> Self
    where
        TValidator: IValidate<V, E> + 'static,
    {
        let wrapper = Box::new(NestedMapValidatorWrapper::<T, K, V, E> {
            field_name: self.rules.field_name,
            accessor: self.rules.accessor.clone(),
            inner: Box::new(validator),
            _phantom: PhantomData,
        });

        self.rules_builder
            .nested
            .insert(self.rules.field_name.to_string(), wrapper);

        self
    }
}

impl<'a, T, V, E> NestedField<V, E> for FieldBuilder<'a, T, Option<V>, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn nested<TValidator>(self, validator: TValidator) -> Self
    where
        TValidator: IValidate<V, E> + 'static,
    {
        let accessor = self.rules.accessor.clone();

        let wrapper = Box::new(NestedOptionValidatorWrapper::<T, V, E> {
            field_name: self.rules.field_name,
            accessor,
            inner: Box::new(validator),
            _phantom: PhantomData,
        });

        self.rules_builder
            .nested
            .insert(self.rules.field_name.to_string(), wrapper);

        self
    }
}

impl<'a, T, V, E> NestedField<V, E> for FieldBuilder<'a, T, Arc<V>, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn nested<TValidator>(self, validator: TValidator) -> Self
    where
        TValidator: IValidate<V, E> + 'static,
    {
        let wrapper = Box::new(NestedArcValidatorWrapper::<T, V, E> {
            field_name: self.rules.field_name,
            accessor: self.rules.accessor.clone(),
            inner: Box::new(validator),
            _phantom: PhantomData,
        });

        self.rules_builder
            .nested
            .insert(self.rules.field_name.to_string(), wrapper);

        self
    }
}

impl<'a, T, V, E> NestedField<V, E> for FieldBuilder<'a, T, Arc<Option<V>>, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn nested<TValidator>(self, validator: TValidator) -> Self
    where
        TValidator: IValidate<V, E> + 'static,
    {
        let wrapper = Box::new(NestedArcOptionValidatorWrapper::<T, V, E> {
            field_name: self.rules.field_name,
            accessor: self.rules.accessor.clone(),
            inner: Box::new(validator),
            _phantom: PhantomData,
        });

        self.rules_builder
            .nested
            .insert(self.rules.field_name.to_string(), wrapper);

        self
    }
}
