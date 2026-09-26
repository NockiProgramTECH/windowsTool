use serde::Serialize;
use std::fmt;

/// Erreur IPC structurée : { "type": "SystemError", "message": "…" }.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    SystemError(String),
    PermissionDenied(String),
    NotSupported(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SystemError(message) => write!(f, "Erreur système : {message}"),
            Self::PermissionDenied(message) => write!(f, "Permission refusée : {message}"),
            Self::NotSupported(message) => write!(f, "Non pris en charge : {message}"),
        }
    }
}

impl std::error::Error for AppError {}

#[cfg(test)]
mod tests {
    use super::AppError;
    use serde_json::json;

    #[test]
    fn serializes_each_variant_as_a_structured_error() {
        for (error, kind) in [
            (AppError::SystemError("test".into()), "SystemError"),
            (AppError::PermissionDenied("test".into()), "PermissionDenied"),
            (AppError::NotSupported("test".into()), "NotSupported"),
        ] {
            assert_eq!(
                serde_json::to_value(error).unwrap(),
                json!({ "type": kind, "message": "test" })
            );
        }
    }

    #[test]
    fn implements_standard_error_and_serialize() {
        fn assert_traits<T: std::error::Error + serde::Serialize + Send + Sync>() {}
        assert_traits::<AppError>();
    }
}
