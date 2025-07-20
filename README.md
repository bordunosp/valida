[![Crates.io](https://img.shields.io/crates/v/valida.svg)](https://crates.io/crates/rust_bus)
![Build Status](https://github.com/bordunosp/valida/actions/workflows/rust.yml/badge.svg)
[![Docs.rs](https://docs.rs/valida/badge.svg)](https://docs.rs/valida)
[![License](https://img.shields.io/crates/l/valida)](https://crates.io/crates/valida)
![unsafe-free](https://img.shields.io/badge/unsafe-✗%20no%20unsafe-success)
[![Downloads](https://img.shields.io/crates/d/valida.svg?style=flat-square)](https://crates.io/crates/valida)


# 🧩 Valida — Modular Validation Library for Rust
#### Valida is a robust and extensible validation system for Rust, designed for flexibility, clarity, and internationalization. It supports both synchronous and asynchronous validators, custom rule injection, nested structures, and ergonomic error handling.

---

## ✨ Features

#### ✅ Sync & Async Support Mix traditional and async validation with ease — ideal for scenarios involving databases, APIs, or file systems.

#### 🛠 Custom Validators Implement your own logic: both sync and async rules are supported via traits.

#### 🌐 Built-in i18n (Internationalization) Comes with out-of-the-box support for 10 major languages (en, es, de, fr, uk, etc.) and can be easily extended or overridden using `rust-i18n`.

#### 🧱 Nested Validation Support Validate deeply nested fields inside Option, Vec, Arc, HashMap and custom structs.

#### 🧪 DSL or Macro-Based Configuration Choose your style: configure validators declaratively using a fluent DSL or use Rust macros for brevity — or combine both approaches.

#### 🚧 Precise Error Separation Validation errors are clearly separated from system-level failures (e.g. DB connection issues), simplifying custom error management and debugging.

#### 📄 Flexible Output Formats Validation errors can be rendered as structured messages in formats like JSON, tree views, or localized text.

#### 🧭 Debug-Friendly Nested Error Tree

```yaml
profile:
  age: age.too_young
  device:
    name: Minimum length is 2 characters
email: Minimum length is 5 characters
optional:
  age: age.too_young
  device:
    name: Minimum length is 2 characters
devices:
  0:
    device:
      name: Minimum length is 2 characters
    age: age.too_young
```

----

## 🧠 Simple Example `Macro` style

```rust
use valida::prelude::*;

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


#[tokio::main]
async fn main() {
    let user = User {
        email: "bad".into(),
        age: -5,
        device: Device { name: "".into() },
    };

    let result = UserValidator.validate(&user).await;

    if let Err(ValidatorFailure::Invalid(e)) = result {
        println!("{}", e.to_json("en"));
        println!("{}", e.to_json_form("en"));
    }
}
```


## 🧠 Simple Example `DSL` style

```rust
use valida::prelude::*;

#[derive(Debug)]
pub struct Device {
    pub name: String,
}

#[derive(Debug)]
pub struct User {
    pub email: String,
    pub age: i32,
    pub device: Device,
}

pub struct DeviceValidator;
#[async_trait::async_trait]
impl IValidate<Device, std::io::Error> for DeviceValidator {
    fn rules(&self, mut builder: RulesBuilder<Device, std::io::Error>) -> RulesBuilder<Device, std::io::Error> {
        builder.field("name", |d| &d.name).trimmed().min_length(2).build();
        builder
    }
}

pub struct UserValidator;
#[async_trait::async_trait]
impl IValidate<User, std::io::Error> for UserValidator {
    fn rules(&self, mut builder: RulesBuilder<User, std::io::Error>) -> RulesBuilder<User, std::io::Error> {
        builder.field("email", |u| &u.email).email().min_length(5).build();
        builder.field("age", |u| &u.age).min(0).build();
        builder.field("device", |u| &u.device).nested(DeviceValidator).build();
        builder
    }
}

#[tokio::main]
async fn main() {
    let user = User {
        email: "bad".into(),
        age: -5,
        device: Device { name: "".into() },
    };

    let result = UserValidator.validate(&user).await;

    if let Err(ValidatorFailure::Invalid(e)) = result {
        println!("{}", e.to_json("en"));
        println!("{}", e.to_json_form("en"));
    }
}
```

```json
{
  "age":"Value must be at least 0",
  "device":{
    "name":"Minimum length is 2 characters"
  },
  "email":"Minimum length is 5 characters"
}
```

```json
{
  "age":"Value must be at least 0",
  "device[name]":"Minimum length is 2 characters",
  "email":"Minimum length is 5 characters"
}
```

---

# 📚 Documentation

### For extended documentation and advanced guides:

[Getting Started](https://github.com/bordunosp/valida/blob/main/doc/get_started.md)

[Custom Rules & Traits](https://github.com/bordunosp/valida/blob/main/doc/custom.md)

[Localization Setup](https://github.com/bordunosp/valida/blob/main/doc/localization.md)

[Nested Wrappers](https://github.com/bordunosp/valida/blob/main/doc/nested.md)

[DSL vs Macros](https://github.com/bordunosp/valida/blob/main/doc/dsl_vs_macros.md)

[Error Reporting](https://github.com/bordunosp/valida/blob/main/doc/errors.md)

---


# #StandForUkraine 🇺🇦

This project aims to show support for Ukraine and its people amidst a war that has been ongoing since 2014. This war has
a genocidal nature and has led to the deaths of thousands, injuries to millions, and significant property damage. We
believe that the international community should focus on supporting Ukraine and ensuring security and freedom for its
people.
