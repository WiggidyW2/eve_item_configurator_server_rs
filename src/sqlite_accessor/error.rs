#[derive(Debug)]
pub enum Error {
    AcquireConnectionError(sqlx::Error),
    SelectStringError(sqlx::Error),
    SelectU32Error(sqlx::Error),
    CreatePoolError(sqlx::Error),
    InvalidUrlError(sqlx::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AcquireConnectionError(e) => write!(f, "AcquireConnectionError: {}", e),
            Error::SelectStringError(e) => write!(f, "SelectStringError: {}", e),
            Error::SelectU32Error(e) => write!(f, "SelectU32Error: {}", e),
            Error::CreatePoolError(e) => write!(f, "CreatePoolError: {}", e),
            Error::InvalidUrlError(e) => write!(f, "InvalidUrlError: {}", e),
        }
    }
}

impl std::error::Error for Error {}
