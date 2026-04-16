use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid query")]
    InvalidQuery,
    #[error("source unavailable")]
    SourceUnavailable,
    #[error("permission denied")]
    PermissionDenied,
    #[error("policy blocked")]
    PolicyBlocked,
    #[error("storage failure")]
    StorageFailure,
    #[error("not found")]
    NotFound,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
