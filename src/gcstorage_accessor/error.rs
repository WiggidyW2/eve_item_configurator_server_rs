#[derive(Debug)]
pub enum CreateClientError {
    CredentialsError(serde_json::Error),
    AuthError(google_cloud_auth::error::Error),
}

impl std::fmt::Display for CreateClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateClientError::CredentialsError(e) => write!(f, "CredentialsError: {}", e),
            CreateClientError::AuthError(e) => write!(f, "AuthError: {}", e),
        }
    }
}

impl std::error::Error for CreateClientError {}

#[derive(Debug)]
pub enum UploadError {
    SerializeError(serde_yaml::Error),
    HttpError(google_cloud_storage::http::Error),
}

impl std::fmt::Display for UploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UploadError::SerializeError(e) => write!(f, "SerializeError: {}", e),
            UploadError::HttpError(e) => write!(f, "HttpError: {}", e),
        }
    }
}

impl std::error::Error for UploadError {}

#[derive(Debug)]
pub enum DownloadError {
    DeserializeError(serde_yaml::Error),
    HttpError(google_cloud_storage::http::Error),
    Utf8Error(std::str::Utf8Error),
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadError::DeserializeError(e) => write!(f, "DeserializeError: {}", e),
            DownloadError::HttpError(e) => write!(f, "HttpError: {}", e),
            DownloadError::Utf8Error(e) => write!(f, "Utf8Error: {}", e),
        }
    }
}

impl std::error::Error for DownloadError {}

#[derive(Debug)]
pub enum Error {
    CreateClientError(CreateClientError),
    DownloadError(DownloadError),
    UploadError(UploadError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CreateClientError(e) => write!(f, "CreateClientError: {}", e),
            Error::DownloadError(e) => write!(f, "DownloadError: {}", e),
            Error::UploadError(e) => write!(f, "UploadError: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<CreateClientError> for Error {
    fn from(e: CreateClientError) -> Self {
        Error::CreateClientError(e)
    }
}

impl From<DownloadError> for Error {
    fn from(e: DownloadError) -> Self {
        Error::DownloadError(e)
    }
}

impl From<UploadError> for Error {
    fn from(e: UploadError) -> Self {
        Error::UploadError(e)
    }
}
