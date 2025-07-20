# 🛠 Custom Rules & Traits

Valida supports custom validation rules through well-defined traits, making it easy to extend the framework with your own domain-specific logic. Whether you're validating formats, applying business rules, or integrating external data checks — you can inject both **sync** and **async** rules into field definitions.

---

## 🎯 Available Traits

| Trait                         | Sync / Async | Description                                      |
|------------------------------|--------------|--------------------------------------------------|
| `IValidatorRule`             | ✅ Sync      | Built-in rules like `min_length`, `email`        |
| `IValidatorRuleCustom`       | ✅ Sync      | Your own quick checks (e.g. string patterns)     |
| `IValidatorRuleCustomAsync`  | ✅ Async     | Logic with async dependencies (e.g. database)    |

Each rule returns:

- `Ok(None)` → validation passed ✅
- `Ok(Some(error))` → validation failed ❌
- `Err(e)` → system-level failure 🛑 (e.g. DB timeout)

---

## 🧩 Custom Rule (Sync) — Basic

Use `ValidationError::new(...)` for simple errors:

```rust
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
```

# 🧩 Implementing a Custom Rule (Async)

Use ValidationError::new_with_params(...) when localized templates require variables:

```rust
pub struct MaxLength(pub usize);

#[async_trait::async_trait]
impl IValidatorRuleCustomAsync<String, std::io::Error> for MaxLength {
    async fn validate(&self, value: &String) -> Result<Option<ValidationError>, std::io::Error> {
        if value.len() > self.0 {
            Ok(Some(ValidationError::new_with_params(
                "validator.max_length",
                HashMap::from([("max".to_string(), self.0.to_string())]),
            )))
        } else {
            Ok(None)
        }
    }
}
```

This integrates with Fluent like:

```
validator.max_length = The value must not exceed { $max } characters.
```

---

# 🔧 Injecting Custom Rules into Validator

```rust
builder
    .field("name", |x| &x.name)
    .trimmed()
    .custom(NoNumber)
    .custom_async(MaxLength(10))
    .build();

builder
    .field("age", |x| &x.age)
    .custom_async(MinAge(18))
    .build();
```

You can mix them freely with built-in rules in any order.



---


# ✅ Example DTO

```rust
#[derive(Debug)]
struct SampleDto {
    name: String,
    age: i32,
}
```

# 🧪 Validating & Asserting Errors

```rust
let dto = SampleDto {
    name: "John99TooLong".into(), // ❌ has digit & too long
    age: 14,                      // ❌ too young
};

let validator = SampleValidator;
let result = validator.validate(&dto).await;

match result {
    Ok(_) => panic!("Should be invalid"),
    Err(ValidatorFailure::Invalid(errors)) => {
        let json = errors.to_json_raw();
        let output = serde_json::to_string(&json).unwrap();
        assert!(output.contains("name.has_number"));
        assert!(output.contains("validator.max_length"));
        assert!(output.contains("age.too_young"));
    }
    Err(ValidatorFailure::System(e)) => panic!("System error: {:?}", e),
}
```

---


# 🧠 Tips

* Prefer `new_with_params()` when using localized messages with variables.
* Keep error keys like `age.too_young` or `validator.max_length` consistent with your i18n bundle.
* System errors (`Err(e)`) are useful for handling failures from external services (DB, API, filesystem).