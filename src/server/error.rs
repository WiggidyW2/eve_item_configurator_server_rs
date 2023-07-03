use super::{
    accessors::Error as AccessorError,
    service::{JsonError, ProtoFieldError},
};
use crate::character_validator::Error as ValidatorError;

#[derive(Debug)]
pub enum Error {
    ProtoFieldError(ProtoFieldError),
    ValidatorError(ValidatorError),
    AccessorError(AccessorError),
    JsonError(JsonError),
    ServeError(tonic::transport::Error),
}

impl From<AccessorError> for Error {
    fn from(e: AccessorError) -> Self {
        Self::AccessorError(e)
    }
}

impl From<ValidatorError> for Error {
    fn from(e: ValidatorError) -> Self {
        Self::ValidatorError(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Self::JsonError(e)
    }
}

impl From<ProtoFieldError> for Error {
    fn from(e: ProtoFieldError) -> Self {
        Self::ProtoFieldError(e)
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(e: tonic::transport::Error) -> Self {
        Self::ServeError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProtoFieldError(e) => {
                write!(f, "ProtoFieldError: {}", e)
            }
            Self::ValidatorError(e) => {
                write!(f, "ValidatorError: {}", e)
            }
            Self::AccessorError(e) => {
                write!(f, "AccessorError: {}", e)
            }
            Self::JsonError(e) => {
                write!(f, "JsonError: {}", e)
            }
            Self::ServeError(e) => {
                write!(f, "ServeError: {}", e)
            }
        }
    }
}

impl std::error::Error for Error {}

impl Into<tonic::Status> for Error {
    fn into(self) -> tonic::Status {
        match self {
            Self::ProtoFieldError(e) => e.into(),
            Self::ValidatorError(e) => tonic::Status::internal(e.to_string()),
            Self::AccessorError(e) => e.into(),
            Self::JsonError(e) => e.into(),
            Self::ServeError(e) => tonic::Status::internal(e.to_string()),
        }
    }
}
