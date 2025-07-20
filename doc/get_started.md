# 🚀 Getting Started with Valida

**Valida** is a modular validation library for Rust designed to support nested data structures, internationalization, and both sync/async rule definitions. It offers a clear separation between validation errors and system-level failures, making it ideal for scalable and maintainable systems.

---

## ✨ Key Features

- 🔄 Supports both synchronous and asynchronous validation rules
- 🔧 Extendable with custom validators via traits:
    - `IValidatorRule`
    - `IValidatorRuleCustom`
    - `IValidatorRuleCustomAsync`
- 🌍 Built-in localization via Fluent — supports 10 major languages out of the box
- 🧱 Nested validation for `Option`, `Vec`, `HashMap`, `Arc`, and complex structs
- 🧪 Declarative DSL and macro-based configuration via `#[Validatable(...)]`
- 🚧 Clear separation between validation failures and system errors (e.g., DB access, API issues)
- 📄 Multiple error formats: JSON, dotted paths, flat form, pretty-printed tree views

---

## 📦 Installation

In your `Cargo.toml`:

```toml
[dependencies]
valida = { version = "" }
```

---

# 🧠 Using Macros

```rust
#[Validatable(std::io::Error)]
pub struct Device {
    #[validate(trimmed, min_length(2))]
    pub name: String,
}

#[Validatable(std::io::Error)]
pub struct User {
    #[validate(email, min_length(5))]
    pub email: String,

    #[validate(min(0))]
    pub age: i32,

    #[validate(nested(DeviceValidator))]
    pub device: Device,
}
```

Valida will automatically generate DeviceValidator and UserValidator.

---

# 🔧 Using DSL Style

```rust
pub struct UserValidator;

#[async_trait::async_trait]
impl IValidate<User, std::io::Error> for UserValidator {
    fn rules(&self, mut builder: RulesBuilder<User, std::io::Error>) -> RulesBuilder<User, std::io::Error> {
        builder
            .field("email", |u| &u.email)
            .email().min_length(5)
            .build();

        builder
            .field("age", |u| &u.age)
            .min(0)
            .build();

        builder
            .field("device", |u| &u.device)
            .nested(DeviceValidator)
            .build();

        builder
    }
}
```

---

# ✅ Validating Structs

```rust
let result = UserValidator.validate(&user).await;

if let Err(ValidatorFailure::Invalid(errors)) = result {
    println!("{}", errors.pretty_print("en"));
}

```

---

# 🌍 Localization Support

Valida uses `rust-i18n` under the hood and includes:

You can override or extend language files by supplying custom bundles.

---

# 📄 Error Output Styles

Valida supports various formats:

* to_json_raw()
* to_json_form_raw()
* to_json_dot_raw()
* pretty_print_raw()


* to_json("en")
* to_json_form("en")
* to_json_dot("en")
* pretty_print("en")