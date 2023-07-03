use super::error::{CreateClientError, DownloadError, Error, UploadError};
use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_default::WithAuthExt;
use google_cloud_storage::{
    client::{Client as GCSClient, ClientConfig},
    http::{
        object_access_controls::PredefinedObjectAcl,
        objects::{
            download::Range,
            get::GetObjectRequest,
            upload::{Media, UploadObjectRequest, UploadType},
        },
    },
};
use serde::{Deserialize, Serialize};

pub struct Client {
    client: GCSClient,
    bucket: String,
    json_acl: Option<PredefinedObjectAcl>,
    item_acl: Option<PredefinedObjectAcl>,
}

impl Client {
    pub async fn new(
        creds: impl AsRef<[u8]>,
        bucket: String,
        json_acl: Option<PredefinedObjectAcl>,
        item_acl: Option<PredefinedObjectAcl>,
    ) -> Result<Self, Error> {
        let client = GCSClient::new(
            ClientConfig::default()
                .with_credentials(
                    serde_json::from_slice::<CredentialsFile>(creds.as_ref())
                        .map_err(|e| CreateClientError::CredentialsError(e))?,
                )
                .await
                .map_err(|e| CreateClientError::AuthError(e))?,
        );
        Ok(Self {
            client: client,
            bucket: bucket,
            json_acl: json_acl,
            item_acl: item_acl,
        })
    }

    async fn upload<O>(
        &self,
        object: O,
        filename: String,
        acl: Option<PredefinedObjectAcl>,
    ) -> Result<(), Error>
    where
        O: Serialize,
    {
        match self
            .client
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.bucket.clone(),
                    predefined_acl: acl,
                    ..Default::default()
                },
                serde_yaml::to_string(&object).map_err(|e| UploadError::SerializeError(e))?,
                &UploadType::Simple(Media::new(filename)),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(UploadError::HttpError(e).into()),
        }
    }

    async fn download<O>(&self, filename: String) -> Result<O, Error>
    where
        for<'de> O: Deserialize<'de>,
    {
        match self
            .client
            .download_object(
                &GetObjectRequest {
                    bucket: self.bucket.clone(),
                    object: filename,
                    ..Default::default()
                },
                &Range::default(),
            )
            .await
        {
            Ok(obj_bytes) => {
                let obj_str =
                    std::str::from_utf8(&obj_bytes).map_err(|e| DownloadError::Utf8Error(e))?;
                let obj: O = serde_yaml::from_str(obj_str)
                    .map_err(|e| DownloadError::DeserializeError(e))?;
                Ok(obj)
            }
            Err(e) => Err(DownloadError::HttpError(e).into()),
        }
    }

    pub(crate) async fn upload_json<O>(&self, object: O, name: &str) -> Result<(), Error>
    where
        O: Serialize,
    {
        self.upload(object, json_filename(name), self.json_acl.clone())
            .await
    }

    pub(crate) async fn upload_items<O>(&self, object: O, name: &str) -> Result<(), Error>
    where
        O: Serialize,
    {
        self.upload(object, items_filename(name), self.item_acl.clone())
            .await
    }

    pub(crate) async fn download_json<O>(&self, name: &str) -> Result<O, Error>
    where
        for<'de> O: Deserialize<'de>,
    {
        self.download(json_filename(name)).await
    }

    pub(crate) async fn download_items<O>(&self, name: &str) -> Result<O, Error>
    where
        for<'de> O: Deserialize<'de>,
    {
        self.download(items_filename(name)).await
    }
}

fn json_filename(name: &str) -> String {
    format!("{}_json.yaml", name)
}

fn items_filename(name: &str) -> String {
    format!("{}_items.yaml", name)
}
