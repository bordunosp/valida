# 🌐 Localization Setup

Valida natively integrates with [`rust-i18n`](https://github.com/longbridge/rust-i18n), enabling rich multi-language support without requiring external libraries like Fluent. You can use built-in language packs or define your own translations for custom error keys.

---

## ✨ Supported Languages

By default, Valida supports:

uk, en, de, es, pl, hi, fr, pt, ja

---


These are bundled with localized strings for built-in rules like `min_length`, `email`, `max`, `required`, etc.

---

## 🧩 Setup Guide

### 1. Add dependencies

```toml
[dependencies]
rust-i18n = { version = "" }
valida = { version = "" }
```

### 2. Enable rust-i18n in your main module

```rust
use rust_i18n::t;

rust_i18n::i18n!("locales"); // This loads all .yml files in /locales
```

# 🗂 Structure for Localization Files

Your project layout should include a folder:

```toml
your_project/
  └── locales/
        ├── en.yml
        ├── uk.yml
        └── ...
```

Each YAML file contains key-value translations:

```yaml
# locales/en.yml
validator.min_length: Minimum length is %{min} characters.
age.too_young: Must be at least 18 years old.
```
You can override any built-in key or add your own.

---

# 🧠 Using Error Keys in Custom Rules

```rust
ValidationError::new("age.too_young")
```

Or with parameters:

```rust
ValidationError::new_with_params(
    "validator.max_length",
    HashMap::from([("max".to_string(), "10".to_string())])
)
```

Which resolves to:

```yaml
validator.max_length: Value must not exceed %{max} characters.
```

---


# 🧪 Runtime Localization

You can change language dynamically:

```rust
println!("{}", errors.pretty_print("uk"));
```

---

# 📦 Overriding Messages

Simply redefine the key in your own YAML file. Valida will pick up your version if present in the selected locale.

---

# 🧠 Best Practices

* Use lowercase dotted keys: `validator.min_length`, `age.too_young`
* Define all custom rule keys in your `locales/*.yml`
* Keep parameter keys (`%{param}`) intuitive and consistent





