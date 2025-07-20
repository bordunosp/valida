# 🚧 Error Reporting

Valida provides a powerful and flexible error reporting system that separates **validation failures** from **system-level errors**. Errors are structured, deeply nestable, serializable, and localization-ready (when i18n is enabled).

---

## 🔍 Core Types

### `ValidationError`

Represents a single error leaf. Includes:

- `key: String` — the error code or i18n key
- `params: HashMap<String, String>` — optional arguments for localized message interpolation

Create errors with:

```rust
ValidationError::new("email.invalid")
ValidationError::new_with_params("validator.max_length", {
    "max" => "100"
})
```

---


### ValidationNode

Used internally to represent a tree of nested errors:

```rust
enum ValidationNode {
    Leaf(ValidationError),
    Branch(HashMap<String, ValidationNode>)
}
```

---

### ValidationErrors

The top-level container used throughout Valida:

* Stores errors as a tree keyed by field path
* Can add errors manually (`add(...)`) or aggregate nested errors (`add_nested(...)`)
* Implements `Error + Display`

---

# 📦 Formatting Output

Valida offers multiple error formats for UI and debugging.

## 🔡 Raw Pretty Print

```rust
println!("{}", errors.pretty_print_raw())
```

Outputs a tree-like view:

```yaml
email: email.invalid
profile:
  age: age.too_young
  device:
    name: validator.min_length
```

---

## 🔢 JSON (Raw Structure)

```rust
let json = errors.to_json_raw();
```

Includes both key and params:

```json
{
  "email": {
    "key": "email.invalid",
    "params": {}
  },
  "profile": {
    "age": {
      "key": "age.too_young",
      "params": {}
    }
  }
}
```

---

## 🧾 Flat HTML Form Style

```rust
let form = errors.to_json_form_raw();
```

Produces flat keys like `field[subfield]`:

```json
{
  "profile[device][name]": {
    "key": "validator.min_length",
    "params": {}
  }
}
```

Perfect for HTML-based form rendering.

---

## 🕸 Dotted Path Style

```rust
let dot = errors.to_json_dot_raw();
```

Useful for config files or path mapping:

```json
{
  "profile.device.name": {
    "key": "validator.min_length",
    "params": {}
  }
}
```

---

# 🌍 Localized Formats (Optional)

When compiled with `i18n-localization` feature, Valida supports:

* pretty_print(locale)
* to_json(locale)
* to_json_form(locale)
* to_json_dot(locale)
  
Localization is powered by `rust-i18n`

---

# 🧠 Best Practices

Prefer using `new_with_params(...)` for dynamic error content
Use error keys that map directly to translation files (`validator.min_length`, etc.)
Nest errors via `nested(...)` for multi-layered DTOs


