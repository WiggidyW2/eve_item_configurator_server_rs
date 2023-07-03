#[derive(Debug)]
pub enum FirestoreError {
    CreateClientError(firestore::errors::FirestoreError),
    WriteError(firestore::errors::FirestoreError),
    ReadError(firestore::errors::FirestoreError),
    CollectionPathError(firestore::errors::FirestoreError),
}

impl std::fmt::Display for FirestoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirestoreError::CreateClientError(e) => write!(f, "{}", e),
            FirestoreError::WriteError(e) => write!(f, "{}", e),
            FirestoreError::ReadError(e) => write!(f, "{}", e),
            FirestoreError::CollectionPathError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for FirestoreError {}

#[derive(Debug)]
pub enum Error {
    FirestoreError(FirestoreError),
    CollectionPathError,
}

impl From<FirestoreError> for Error {
    fn from(e: FirestoreError) -> Self {
        Error::FirestoreError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::FirestoreError(e) => write!(f, "{}", e),
            Error::CollectionPathError => write!(
                f,
                "Collection path must be at least 1 element long and have an odd number of elements",
            ),
        }
    }
}

impl std::error::Error for Error {}
