use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UuidVersion {
    V1,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
}

pub struct UuidVersionValidator {
    pub version: UuidVersion,
}

impl<T: StrAsRef> IValidatorRule<T> for UuidVersionValidator {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(input) = value.as_str_ref() {
            match Uuid::parse_str(input) {
                Ok(uuid) => {
                    let actual = match uuid.get_version_num() {
                        1 => UuidVersion::V1,
                        3 => UuidVersion::V3,
                        4 => UuidVersion::V4,
                        5 => UuidVersion::V5,
                        6 => UuidVersion::V6,
                        7 => UuidVersion::V7,
                        8 => UuidVersion::V8,
                        _ => return Err(ValidationError::new("validator.uuid.invalid_format")),
                    };

                    if actual != self.version {
                        return Err(ValidationError::new_with_params(
                            "validator.uuid.version.mismatch",
                            HashMap::from([("actual".into(), format!("{actual:?}"))]),
                        ));
                    }
                }
                Err(_) => {
                    return Err(ValidationError::new("validator.uuid.invalid_format"));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;

    fn validator(version: UuidVersion) -> UuidVersionValidator {
        UuidVersionValidator {
            // field: "id",
            version,
        }
    }

    #[test]
    fn passes_correct_v4_uuid() {
        let value = Uuid::new_v4().to_string();
        assert!(validator(UuidVersion::V4).validate(&value).is_ok());
    }

    #[test]
    fn fails_wrong_version() {
        let value = Uuid::new_v4().to_string();
        assert!(validator(UuidVersion::V1).validate(&value).is_err());
    }

    #[test]
    fn fails_invalid_uuid_format() {
        let value = "invalid-uuid";
        assert!(validator(UuidVersion::V4).validate(&value).is_err());
    }

    #[test]
    fn validates_option_none() {
        let value: Option<String> = None;
        assert!(validator(UuidVersion::V7).validate(&value).is_ok());
    }

    #[test]
    fn accepts_mock_v7() {
        let value = "0185c87d-efb5-7cc2-932c-bce379ec5492";
        assert!(validator(UuidVersion::V7).validate(&value).is_ok());
    }
}
