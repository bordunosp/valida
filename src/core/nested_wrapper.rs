use crate::core::errors::ValidationErrors;
use crate::prelude::{IValidate, RulesBuilder, ValidatorFailure};
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;

type Accessor<T, V> = Arc<dyn Fn(&T) -> &V + Send + Sync>;

pub struct NestedArcOptionValidatorWrapper<T, V, E>
where
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    #[allow(dead_code)]
    pub field_name: &'static str,
    pub accessor: Accessor<T, Arc<Option<V>>>,
    pub inner: Box<dyn IValidate<V, E>>,
    pub _phantom: PhantomData<T>,
}

#[async_trait::async_trait]
impl<T, V, E> IValidate<T, E> for NestedArcOptionValidatorWrapper<T, V, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn rules(&self, builder: RulesBuilder<T, E>) -> RulesBuilder<T, E> {
        builder
    }

    async fn validate(&self, dto: &T) -> Result<(), ValidatorFailure<E>> {
        match (self.accessor)(dto).as_ref() {
            Some(inner) => self.inner.validate(inner).await,
            None => Ok(()),
        }
    }
}

pub struct NestedArcValidatorWrapper<T, V, E>
where
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    #[allow(dead_code)]
    pub field_name: &'static str,
    pub accessor: Accessor<T, Arc<V>>,
    pub inner: Box<dyn IValidate<V, E>>,
    pub _phantom: PhantomData<T>,
}

#[async_trait::async_trait]
impl<T, V, E> IValidate<T, E> for NestedArcValidatorWrapper<T, V, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn rules(&self, builder: RulesBuilder<T, E>) -> RulesBuilder<T, E> {
        builder
    }

    async fn validate(&self, dto: &T) -> Result<(), ValidatorFailure<E>> {
        let arc_ref = (self.accessor)(dto);
        self.inner.validate(arc_ref.as_ref()).await
    }
}

pub struct NestedVecValidatorWrapper<T, U, E>
where
    U: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    #[allow(dead_code)]
    pub field_name: &'static str,
    pub accessor: Accessor<T, Vec<U>>,
    pub inner: Box<dyn IValidate<U, E>>,
    pub _phantom: PhantomData<T>,
}

#[async_trait]
impl<T, U, E> IValidate<T, E> for NestedVecValidatorWrapper<T, U, E>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn rules(&self, builder: RulesBuilder<T, E>) -> RulesBuilder<T, E> {
        builder
    }

    async fn validate(&self, dto: &T) -> Result<(), ValidatorFailure<E>> {
        let list = (self.accessor)(dto);
        let mut all_errors = ValidationErrors::default();

        for (i, item) in list.iter().enumerate() {
            match self.inner.validate(item).await {
                Ok(_) => {}
                Err(ValidatorFailure::Invalid(nested)) => {
                    all_errors.add_nested(vec![i.to_string()], nested);
                }
                Err(e) => return Err(e),
            }
        }

        if all_errors.is_empty() {
            Ok(())
        } else {
            Err(ValidatorFailure::Invalid(all_errors))
        }
    }
}

pub struct NestedOptionValidatorWrapper<T, V, E>
where
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    #[allow(dead_code)]
    pub field_name: &'static str,
    pub accessor: Accessor<T, Option<V>>,
    pub inner: Box<dyn IValidate<V, E>>,
    pub _phantom: PhantomData<T>,
}
#[async_trait::async_trait]
impl<T, V, E> IValidate<T, E> for NestedOptionValidatorWrapper<T, V, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn rules(&self, builder: RulesBuilder<T, E>) -> RulesBuilder<T, E> {
        builder
    }

    async fn validate(&self, dto: &T) -> Result<(), ValidatorFailure<E>> {
        match (self.accessor)(dto) {
            Some(inner_value) => match self.inner.validate(inner_value).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            None => Ok(()),
        }
    }
}

pub struct NestedMapValidatorWrapper<T, K, U, E>
where
    K: Eq + Hash + ToString + Send + Sync + 'static,
    U: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    #[allow(dead_code)]
    pub field_name: &'static str,
    pub accessor: Accessor<T, HashMap<K, U>>,
    pub inner: Box<dyn IValidate<U, E>>,
    pub _phantom: PhantomData<T>,
}

#[async_trait]
impl<T, K, U, E> IValidate<T, E> for NestedMapValidatorWrapper<T, K, U, E>
where
    T: Send + Sync + 'static,
    K: Eq + Hash + ToString + Send + Sync + 'static,
    U: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn rules(&self, builder: RulesBuilder<T, E>) -> RulesBuilder<T, E> {
        builder
    }

    async fn validate(&self, dto: &T) -> Result<(), ValidatorFailure<E>> {
        let map = (self.accessor)(dto);
        let mut all_errors = ValidationErrors::default();

        for (key, value) in map {
            match self.inner.validate(value).await {
                Ok(_) => {}
                Err(ValidatorFailure::Invalid(nested)) => {
                    all_errors.add_nested(vec![key.to_string()], nested);
                }
                Err(e) => return Err(e),
            }
        }

        if all_errors.is_empty() {
            Ok(())
        } else {
            Err(ValidatorFailure::Invalid(all_errors))
        }
    }
}

pub struct NestedValidatorWrapper<T, U, E>
where
    E: Error + Send + Sync + 'static,
{
    #[allow(dead_code)]
    pub field_name: &'static str,
    pub accessor: Accessor<T, U>,
    pub inner: Box<dyn IValidate<U, E>>,
    pub _phantom: PhantomData<T>,
}

#[async_trait]
impl<T, U, E> IValidate<T, E> for NestedValidatorWrapper<T, U, E>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn rules(&self, builder: RulesBuilder<T, E>) -> RulesBuilder<T, E> {
        builder
    }

    async fn validate(&self, dto: &T) -> Result<(), ValidatorFailure<E>> {
        let value = (self.accessor)(dto);
        self.inner.validate(value).await
    }
}
