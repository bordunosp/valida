# 🧱 Nested Wrappers

Valida supports nested validation through a flexible set of wrapper types and trait extensions. These wrappers allow you to apply validation rules to deeply nested structures such as `Option<T>`, `Vec<T>`, `HashMap<K, T>`, and `Arc<T>` without manual recursion or complex plumbing.

---

## 🧩 How It Works

When using `.field(...).nested(...)`, Valida automatically wraps the target field in the appropriate **Nested Wrapper**, depending on its type.

Each wrapper implements `IValidate<T, E>` and delegates validation to the corresponding inner validator.

---

## 📦 Supported Types

| Field Type           | Wrapper Used                         |
|----------------------|--------------------------------------|
| `T`                  | `NestedValidatorWrapper`             |
| `Vec<T>`             | `NestedVecValidatorWrapper`          |
| `HashMap<K, T>`      | `NestedMapValidatorWrapper`          |
| `Option<T>`          | `NestedOptionValidatorWrapper`       |
| `Arc<T>`             | `NestedArcValidatorWrapper`          |
| `Arc<Option<T>>`     | `NestedArcOptionValidatorWrapper`    |

Each wrapper is automatically chosen based on the type signature in your `.field(...)` declaration.

---

## ✨ Example

```rust
builder
    .field("profile", |x| &x.profile)
    .nested(ProfileValidator)
    .build();

builder
    .field("devices", |x| &x.devices)
    .nested(ProfileValidator) // applies to Vec<ProfileDto>
    .build();

builder
    .field("metadata", |x| &x.metadata)
    .nested(ProfileValidator) // applies to HashMap<String, ProfileDto>
    .build();

builder
    .field("optional", |x| &x.optional)
    .nested(ProfileValidator) // applies to Option<ProfileDto>
    .build();

builder
    .field("arc_optional", |x| &x.arc_optional)
    .nested(ProfileValidator) // applies to Arc<Option<ProfileDto>>
    .build();
```

No need to write manual loops or unwrap logic — Valida handles iteration, presence checks, and error accumulation for you.


# 🧠 Internal Mechanism

Each wrapper collects errors using `ValidationErrors::default()` and aggregates them using `add_nested(...)`, preserving the structure:

```yaml
devices:
  0:
    age: age.too_young
    device:
      name: validator.min_length
metadata:
  main:
    age: age.too_young
```