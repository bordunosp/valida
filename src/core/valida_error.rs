#[derive(Debug)]
pub enum ValidaError {
    Io(std::io::Error),
    InvalidLocale(String),
    MissingFile(String),
    FluentParse(String),
    FluentMessage(String),
    SerdeError(serde_json::Error),
}

impl std::fmt::Display for ValidaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidaError::Io(e) => write!(f, "I/O error: {e}"),
            ValidaError::InvalidLocale(l) => write!(f, "Invalid locale string: {l}"),
            ValidaError::MissingFile(p) => write!(f, "Translation file not found: {p}"),
            ValidaError::FluentParse(p) => write!(f, "Fluent parse error: {p}"),
            ValidaError::FluentMessage(k) => write!(f, "Missing Fluent message key: {k}"),
            ValidaError::SerdeError(e) => write!(f, "Serialization error: {e}"),
        }
    }
}

impl std::error::Error for ValidaError {}

impl From<std::io::Error> for ValidaError {
    fn from(e: std::io::Error) -> Self {
        ValidaError::Io(e)
    }
}

impl From<serde_json::Error> for ValidaError {
    fn from(e: serde_json::Error) -> Self {
        ValidaError::SerdeError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_display_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "test.txt");
        let valida = ValidaError::Io(io_err);
        assert!(valida.to_string().contains("I/O error:"));
        assert!(valida.to_string().contains("test.txt"));
    }

    #[test]
    fn test_display_fluent_parse_error() {
        let valida = ValidaError::FluentParse("malformed.ftl".into());
        assert_eq!(valida.to_string(), "Fluent parse error: malformed.ftl");
    }

    #[test]
    fn test_display_invalid_locale() {
        let valida = ValidaError::InvalidLocale("💩💩💩".into());
        assert_eq!(valida.to_string(), "Invalid locale string: 💩💩💩");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "no access");
        let valida: ValidaError = io_err.into();
        match valida {
            ValidaError::Io(e) => assert_eq!(e.kind(), io::ErrorKind::PermissionDenied),
            _ => panic!("Incorrect variant"),
        }
    }

    #[test]
    fn test_from_serde_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("not json").unwrap_err();
        let valida: ValidaError = json_err.into();
        match valida {
            ValidaError::SerdeError(e) => assert!(e.is_syntax()),
            _ => panic!("Incorrect variant"),
        }
    }
}
