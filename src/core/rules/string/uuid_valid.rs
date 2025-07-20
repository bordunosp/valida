use crate::core::contract::IValidatorRule;
use crate::core::errors::ValidationError;
use crate::core::rules::str_ref::StrAsRef;
use uuid::Uuid;

pub struct UuidValid {}

impl<T: StrAsRef> IValidatorRule<T> for UuidValid {
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        if let Some(s) = value.as_str_ref() {
            if Uuid::parse_str(s).is_err() {
                return Err(ValidationError::new("validator.uuid"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::contract::IValidatorRule;
    use uuid::{NoContext, Timestamp, Uuid};

    fn validator() -> UuidValid {
        UuidValid {}
    }

    #[test]
    fn validates_uuid_v1() {
        let ts = Timestamp::from_unix(&NoContext, 1_700_000_000, 42);
        let v1 = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]);
        assert_eq!(v1.get_version_num(), 1);
        assert!(validator().validate(&v1.to_string()).is_ok());
    }

    #[test]
    fn validates_uuid_v3() {
        let ns = Uuid::NAMESPACE_DNS;
        let name = "example.com";
        let v3 = Uuid::new_v3(&ns, name.as_bytes());
        assert_eq!(v3.get_version_num(), 3);
        assert!(validator().validate(&v3.to_string()).is_ok());
    }

    #[test]
    fn validates_uuid_v4() {
        let v4 = Uuid::new_v4();
        assert_eq!(v4.get_version_num(), 4);
        assert!(validator().validate(&v4.to_string()).is_ok());
    }

    #[test]
    fn validates_uuid_v5() {
        let ns = Uuid::NAMESPACE_URL;
        let name = "https://example.org";
        let v5 = Uuid::new_v5(&ns, name.as_bytes());
        assert_eq!(v5.get_version_num(), 5);
        assert!(validator().validate(&v5.to_string()).is_ok());
    }

    #[test]
    fn validates_uuid_v6_to_v8() {
        let samples = [
            // v6: reordered time-based UUID
            "1eb0c610-2eae-6d91-a399-3fd87c62e51c",
            // v7: timestamp-based with random part
            "0185c87d-efb5-7cc2-932c-bce379ec5492",
            // v8: experimental format
            "f2345678-9abc-8def-1234-56789abcdef0",
        ];

        for s in &samples {
            let uuid = Uuid::parse_str(s).expect("Valid UUID string");
            assert!(validator().validate(s).is_ok());
            assert!(matches!(uuid.get_version_num(), 6..=8));
        }
    }

    #[test]
    fn fails_invalid_uuid_format() {
        let bad = "not-a-valid-uuid";
        assert!(validator().validate(&bad).is_err());
    }
}
