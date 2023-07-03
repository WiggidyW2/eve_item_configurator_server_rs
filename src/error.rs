use crate::{
    firestore_accessor::Error as FirestoreError, gcstorage_accessor::Error as GCStorageError,
    server::Error as ServerError, sqlite_accessor::Error as SqliteError,
};

#[derive(Debug)]
pub enum EnvErrorInner {
    MissingOrInvalidError(std::env::VarError),
    InvalidObjectAclError(String),
    InvalidNumberError(String),
    InvalidBoolError(String),
    InvalidSocketAddrError(std::net::AddrParseError),
}

#[derive(Debug)]
pub struct EnvError {
    pub key: &'static str,
    pub inner: EnvErrorInner,
}

impl EnvError {
    pub fn new(key: &'static str, inner: EnvErrorInner) -> Self {
        EnvError { key, inner }
    }
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            EnvErrorInner::MissingOrInvalidError(err) => {
                write!(f, "Missing or invalid environment variable: {}", err)
            }
            EnvErrorInner::InvalidObjectAclError(err) => {
                write!(f, "Invalid object ACL: {}: {}", self.key, err)
            }
            EnvErrorInner::InvalidNumberError(err) => {
                write!(f, "Invalid number: {}: {}", self.key, err)
            }
            EnvErrorInner::InvalidBoolError(err) => {
                write!(f, "Invalid boolean: {}: {}", self.key, err)
            }
            EnvErrorInner::InvalidSocketAddrError(err) => {
                write!(f, "Invalid socket address: {}: {}", self.key, err)
            }
        }
    }
}

impl std::error::Error for EnvError {}

#[derive(Debug)]
pub enum CreateAccessorError {
    SqliteError(SqliteError),
    FirestoreError(FirestoreError),
    GCStorageError(GCStorageError),
}

impl std::fmt::Display for CreateAccessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAccessorError::SqliteError(err) => {
                write!(f, "SqliteError: {}", err)
            }
            CreateAccessorError::FirestoreError(err) => {
                write!(f, "FirestoreError: {}", err)
            }
            CreateAccessorError::GCStorageError(err) => {
                write!(f, "GCStorageError: {}", err)
            }
        }
    }
}

impl std::error::Error for CreateAccessorError {}

impl From<SqliteError> for CreateAccessorError {
    fn from(err: SqliteError) -> Self {
        CreateAccessorError::SqliteError(err)
    }
}

impl From<FirestoreError> for CreateAccessorError {
    fn from(err: FirestoreError) -> Self {
        CreateAccessorError::FirestoreError(err)
    }
}

impl From<GCStorageError> for CreateAccessorError {
    fn from(err: GCStorageError) -> Self {
        CreateAccessorError::GCStorageError(err)
    }
}

#[derive(Debug)]
pub enum Error {
    CreateAccessorError(CreateAccessorError),
    EnvError(EnvError),
    GRPCServerError(ServerError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CreateAccessorError(err) => {
                write!(f, "CreateAccessorError: {}", err)
            }
            Error::EnvError(err) => {
                write!(f, "EnvError: {}", err)
            }
            Error::GRPCServerError(err) => {
                write!(f, "GRPCServerError: {}", err)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<CreateAccessorError> for Error {
    fn from(err: CreateAccessorError) -> Self {
        Error::CreateAccessorError(err)
    }
}

impl From<EnvError> for Error {
    fn from(err: EnvError) -> Self {
        Error::EnvError(err)
    }
}

impl From<ServerError> for Error {
    fn from(err: ServerError) -> Self {
        Error::GRPCServerError(err)
    }
}
