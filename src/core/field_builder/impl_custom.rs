use crate::core::contract::{IValidatorRuleCustom, IValidatorRuleCustomAsync};
use crate::core::field_builder::main::FieldBuilder;
use std::error::Error;

impl<'a, T, V, E> FieldBuilder<'a, T, V, E>
where
    V: 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn custom_async<R>(mut self, rule: R) -> Self
    where
        R: IValidatorRuleCustomAsync<V, E> + Send + Sync + 'static,
    {
        self.rules.rules_custom_async.push(Box::new(rule));
        self
    }
}

impl<'a, T, V, E> FieldBuilder<'a, T, V, E>
where
    V: 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn custom<R>(mut self, rule: R) -> Self
    where
        R: IValidatorRuleCustom<V, E> + Send + Sync + 'static,
    {
        self.rules.rules_custom.push(Box::new(rule));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use valida::prelude::*;

    pub struct MinAge(pub i32);

    #[async_trait]
    impl IValidatorRuleCustomAsync<i32, std::io::Error> for MinAge {
        async fn validate(&self, value: &i32) -> Result<Option<ValidationError>, std::io::Error> {
            if *value < self.0 {
                Ok(Some(ValidationError::new("age.too_young")))
            } else {
                Ok(None)
            }
        }
    }

    // ───── Sync Rule для тесту ─────
    pub struct NoNumber;

    impl IValidatorRuleCustom<String, std::io::Error> for NoNumber {
        fn validate(&self, value: &String) -> Result<Option<ValidationError>, std::io::Error> {
            if value.chars().any(|c| c.is_ascii_digit()) {
                Ok(Some(ValidationError::new("name.has_number")))
            } else {
                Ok(None)
            }
        }
    }

    // ───── DTO для тесту custom + custom_async ─────
    #[derive(Debug)]
    struct SampleDto {
        name: String,
        age: i32,
    }

    pub struct SampleValidator;

    #[async_trait]
    impl IValidate<SampleDto, std::io::Error> for SampleValidator {
        fn rules(
            &self,
            mut builder: RulesBuilder<SampleDto, std::io::Error>,
        ) -> RulesBuilder<SampleDto, std::io::Error> {
            builder
                .field("name", |x| &x.name)
                .trimmed()
                .custom(NoNumber)
                .build();

            builder
                .field("age", |x| &x.age)
                .custom_async(MinAge(18))
                .build();

            builder
        }
    }

    #[tokio::test]
    async fn detects_custom_rules_violation() {
        let dto = SampleDto {
            name: "John99".into(), // ❌ has digit
            age: 14,               // ❌ too young
        };

        let validator = SampleValidator;
        let errors = match validator.validate(&dto).await {
            Ok(_) => panic!("Should be invalid"),
            Err(ValidatorFailure::Invalid(e)) => e,
            Err(ValidatorFailure::System(e)) => panic!("System error: {:?}", e),
        };

        // 🔧 Виправлення виклику to_json
        let json = errors.to_json_raw();

        // 🔧 Перетворення Value → String для перевірки contains
        let json_str = serde_json::to_string(&json).expect("JSON serialization failed");

        assert!(json_str.contains("name.has_number"));
        assert!(json_str.contains("age.too_young"));
    }
}
