use super::error::{Error, FirestoreError};
use firestore::{FirestoreDb, FirestoreDbOptions};
use gcloud_sdk::{TokenSourceType, GCP_DEFAULT_SCOPES};
use serde::{Deserialize, Serialize};

pub struct Client {
    db: FirestoreDb,
    collection: String,
    collection_path: Option<String>,
}

impl Client {
    pub async fn new<S: AsRef<str>>(
        collection_path: &[S],
        google_project_id: String,
        max_retries: Option<usize>,
        firebase_api_url: Option<String>,
        token_scopes: Option<Vec<String>>,
        token_source_type: Option<TokenSourceType>,
    ) -> Result<Self, Error> {
        // collection path must be odd
        if collection_path.len() % 2 == 0 {
            return Err(Error::CollectionPathError);
        }

        // Get the Firestore DB
        let db = FirestoreDb::with_options_token_source(
            FirestoreDbOptions {
                google_project_id: google_project_id,
                max_retries: max_retries.unwrap_or(3),
                firebase_api_url: firebase_api_url,
            },
            token_scopes.unwrap_or(GCP_DEFAULT_SCOPES.clone()),
            token_source_type.unwrap_or(TokenSourceType::Default),
        )
        .await
        .map_err(|e| FirestoreError::CreateClientError(e))?;

        // Get the last collection, which is the immediate parent of the
        // documents that we will read and write
        let collection = collection_path[collection_path.len() - 1]
            .as_ref()
            .to_string();

        // Get the path to the collection (if the collection is not the root)
        let collection_path = {
            if collection_path.len() > 1 {
                let mut builder = db
                    .parent_path(
                        collection_path[collection_path.len() - 3].as_ref(),
                        collection_path[collection_path.len() - 2].as_ref(),
                    )
                    .map_err(|e| FirestoreError::CollectionPathError(e))?;
                if collection_path.len() > 3 {
                    for tuple in collection_path[..collection_path.len() - 4].rchunks(2) {
                        builder = builder.at(tuple[0].as_ref(), tuple[1].as_ref()).unwrap();
                    }
                }
                Some(builder.into())
            } else {
                None
            }
        };

        // Return the Client
        Ok(Self {
            db: db,
            collection: collection,
            collection_path: collection_path,
        })
    }

    pub(crate) async fn read<T, S>(&self, document_id: S) -> Result<Option<T>, Error>
    where
        for<'de> T: Deserialize<'de> + Send,
        S: AsRef<str> + Send,
    {
        let query = self.db.fluent().select().by_id_in(&self.collection);
        let query = match self.collection_path {
            Some(ref path) => query.parent(path),
            None => query,
        };
        query
            .obj::<T>()
            .one(document_id)
            .await
            .map_err(|e| FirestoreError::ReadError(e).into())
    }

    pub(crate) async fn write<T, S>(&self, document_id: S, object: T) -> Result<(), Error>
    where
        for<'de> T: Deserialize<'de> + Serialize + Send + Sync,
        S: AsRef<str> + Send,
    {
        let query = self
            .db
            .fluent()
            .update()
            .in_col(&self.collection)
            .document_id(document_id);
        let query = match self.collection_path {
            Some(ref path) => query.parent(path),
            None => query,
        };
        query
            .object(&object)
            .execute()
            .await
            .map_err(|e| FirestoreError::WriteError(e).into())
    }
}
