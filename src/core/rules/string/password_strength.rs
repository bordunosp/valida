use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub enum StrengthLevel {
    VeryWeak = 0,
    Weak = 1,
    Medium = 2,
    Strong = 3,
    VeryStrong = 4,
}

pub fn estimate_password_strength(password: &str) -> StrengthLevel {
    let length = password.len();
    if length == 0 {
        return StrengthLevel::VeryWeak;
    }

    let mut control = 0;
    let mut digit = 0;
    let mut upper = 0;
    let mut lower = 0;
    let mut symbol = 0;
    let mut other = 0;

    let mut unique = std::collections::HashSet::new();

    for chr in password.chars() {
        unique.insert(chr);
        let code = chr as u32;
        match code {
            0..=31 | 127 => control = 33,
            48..=57 => digit = 10,
            65..=90 => upper = 26,
            97..=122 => lower = 26,
            128.. => other = 128,
            _ => symbol = 33,
        }
    }

    let pool: f64 = (lower + upper + digit + symbol + control + other) as f64;
    let chars = unique.len();
    let entropy = (chars as f64) * pool.log2() + ((length - chars) as f64) * (chars as f64).log2();

    match entropy {
        e if e >= 120.0 => StrengthLevel::VeryStrong,
        e if e >= 100.0 => StrengthLevel::Strong,
        e if e >= 80.0 => StrengthLevel::Medium,
        e if e >= 60.0 => StrengthLevel::Weak,
        _ => StrengthLevel::VeryWeak,
    }
}

pub struct PasswordStrength {
    pub level: StrengthLevel,
}

impl<T: StrAsRef> IValidatorRule<T> for PasswordStrength {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(password) = value.as_str_ref() {
            let strength = estimate_password_strength(password);
            if strength < self.level {
                return Err(ValidationError::new_with_params(
                    "validator.password_strength",
                    HashMap::from([("strength".into(), format!("{strength:?}"))]),
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator(level: StrengthLevel) -> PasswordStrength {
        PasswordStrength { level }
    }

    #[test]
    fn passes_strong_password() {
        let value = "Rust4Life!Secure2024";
        assert!(validator(StrengthLevel::Strong).validate(&value).is_ok());
    }

    #[test]
    fn fails_weak_password() {
        let value = "abc123";
        assert!(validator(StrengthLevel::Medium).validate(&value).is_err());
    }

    #[test]
    fn validates_empty_string_as_very_weak() {
        let value = "";
        assert!(validator(StrengthLevel::VeryWeak).validate(&value).is_ok());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        assert!(validator(StrengthLevel::Medium).validate(&value).is_ok());
    }
}
