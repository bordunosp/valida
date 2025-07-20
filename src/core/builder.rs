use crate::core::contract::{
    IValidate, IValidatorRule, IValidatorRuleCustom, IValidatorRuleCustomAsync, ValidatorFailure,
};
use crate::core::errors::{ValidationError, ValidationErrors};
use crate::core::field_builder::main::FieldBuilder;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::marker::PhantomData;
use std::sync::Arc;

#[async_trait]
pub trait ValidateFieldAsync<T, E>: Send + Sync
where
    E: Error + Send + Sync + 'static,
{
    async fn validate_async(&self, dto: &T) -> Result<Vec<ValidationError>, E>;
}

pub struct FieldRules<T, V, E>
where
    E: Error + Send + Sync + 'static,
{
    pub field_name: &'static str,
    pub accessor: Arc<dyn Fn(&T) -> &V + Send + Sync>,
    pub rules: Vec<Box<dyn IValidatorRule<V>>>,
    pub rules_custom: Vec<Box<dyn IValidatorRuleCustom<V, E>>>,
    pub rules_custom_async: Vec<Box<dyn IValidatorRuleCustomAsync<V, E>>>,
}

#[async_trait]
impl<T, V, E> ValidateFieldAsync<T, E> for FieldRules<T, V, E>
where
    T: Sync,
    V: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    async fn validate_async(&self, dto: &T) -> Result<Vec<ValidationError>, E> {
        let mut errors = vec![];
        let value = (self.accessor)(dto);

        for rule in &self.rules {
            if let Err(e) = rule.validate(value) {
                errors.push(e);
            }
        }

        for rule in &self.rules_custom {
            if let Some(e) = rule.validate(value)? {
                errors.push(e);
            }
        }

        for rule in &self.rules_custom_async {
            if let Some(e) = rule.validate(value).await? {
                errors.push(e);
            }
        }

        Ok(errors)
    }
}

pub struct RulesBuilder<T, E>
where
    E: Error + Send + Sync + 'static,
{
    pub fields: HashMap<String, Box<dyn ValidateFieldAsync<T, E>>>,
    pub nested: HashMap<String, Box<dyn IValidate<T, E>>>,
    pub _phantom: PhantomData<E>,
}

impl<T, E> RulesBuilder<T, E>
where
    T: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            nested: HashMap::new(),
            _phantom: Default::default(),
        }
    }

    pub fn field<TField>(
        &mut self,
        field_name: &'static str,
        accessor: impl Fn(&T) -> &TField + Send + Sync + 'static,
    ) -> FieldBuilder<'_, T, TField, E>
    where
        TField: Send + Sync + 'static,
    {
        let accessor = Arc::new(accessor);

        let rules = FieldRules {
            field_name,
            accessor,
            rules: vec![],
            rules_custom: vec![],
            rules_custom_async: vec![],
        };

        FieldBuilder {
            rules_builder: self,
            rules,
        }
    }

    pub async fn validate(&self, dto: &T) -> Result<ValidationErrors, E> {
        let mut result = ValidationErrors::default();

        for (field_name, field) in &self.fields {
            let field_errors = field.validate_async(dto).await?;
            for error in field_errors {
                result.add(vec![field_name.clone()], error);
            }
        }

        for (field_name, validator) in &self.nested {
            match validator.validate(dto).await {
                Ok(_) => {}
                Err(ValidatorFailure::Invalid(nested_errors)) => {
                    result.add_nested(vec![field_name.clone()], nested_errors);
                }
                Err(ValidatorFailure::System(e)) => {
                    return Err(e);
                }
            }
        }

        Ok(result)
    }
}

impl<T, E> Default for RulesBuilder<T, E>
where
    T: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
