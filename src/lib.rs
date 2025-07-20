#![doc = "valida — Validator"]
extern crate self as valida;

pub mod core;
pub mod prelude;

#[cfg(feature = "i18n-localization")]
rust_i18n::i18n!(
    "dummy_in_memory",
    backend = core::features::localization::i18n::valida_backend::ValidaBackend::new()
);
