use crate::core::builder::{FieldRules, RulesBuilder};
use crate::core::errors::{ValidationError, ValidationErrors};
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;

pub trait IValidatorRule<T>: Send + Sync {
    fn validate(&self, value: &T) -> Result<(), ValidationError>;
}

#[async_trait]
pub trait IValidatorRuleCustomAsync<T, E>: Send + Sync
where
    E: Error + Send + Sync + 'static,
{
    async fn validate(&self, value: &T) -> Result<Option<ValidationError>, E>;
}

#[async_trait]
pub trait IValidatorRuleCustom<T, E>: Send + Sync
where
    E: Error + Send + Sync + 'static,
{
    fn validate(&self, value: &T) -> Result<Option<ValidationError>, E>;
}

pub enum ValidatorFailure<E> {
    Invalid(ValidationErrors),
    System(E),
}

impl<E: Error + Send + Sync + 'static> From<E> for ValidatorFailure<E> {
    fn from(e: E) -> Self {
        ValidatorFailure::System(e)
    }
}

#[async_trait]
pub trait IValidate<T, E>: Send + Sync
where
    T: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn rules(&self, builder: RulesBuilder<T, E>) -> RulesBuilder<T, E>;

    async fn validate(&self, dto: &T) -> Result<(), ValidatorFailure<E>> {
        let builder = RulesBuilder::new();
        match self.rules(builder).validate(dto).await {
            Ok(errors) => {
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(ValidatorFailure::Invalid(errors))
                }
            }
            Err(err) => Err(ValidatorFailure::System(err)),
        }
    }
}

#[async_trait]
pub trait ValidateAsyncField<T, E>: Send + Sync
where
    E: Error + Send + Sync + 'static,
{
    async fn validate_async(&self, dto: &T) -> Result<HashMap<String, ValidationError>, E>;
}

#[async_trait]
impl<T, V, E> ValidateAsyncField<T, E> for FieldRules<T, V, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    async fn validate_async(&self, dto: &T) -> Result<HashMap<String, ValidationError>, E> {
        let mut errors = HashMap::new();
        let value = (self.accessor)(dto);
        for rule in &self.rules_custom_async {
            if let Some(err) = rule.validate(value).await? {
                errors.insert(self.field_name.to_string(), err);
            }
        }
        Ok(errors)
    }
}

#[async_trait]
pub trait ValidateCustomField<T, E>: Send + Sync
where
    E: Error + Send + Sync + 'static,
{
    fn validate(&self, dto: &T) -> Result<HashMap<String, ValidationError>, E>;
}

#[async_trait]
impl<T, V, E> ValidateCustomField<T, E> for FieldRules<T, V, E>
where
    T: Send + Sync + 'static,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn validate(&self, dto: &T) -> Result<HashMap<String, ValidationError>, E> {
        let mut errors = HashMap::new();
        let value = (self.accessor)(dto);
        for rule in &self.rules_custom {
            if let Some(err) = rule.validate(value)? {
                errors.insert(self.field_name.to_string(), err);
            }
        }
        Ok(errors)
    }
}
