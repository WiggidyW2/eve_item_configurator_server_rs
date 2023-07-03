use super::error::Error;
use futures::stream::TryStreamExt;
use sqlx::{
    self,
    pool::{PoolConnection, PoolOptions},
    query_as,
    sqlite::{Sqlite, SqliteConnectOptions},
    SqlitePool,
};
use std::str::FromStr;

pub struct Client(SqlitePool);

impl Client {
    pub async fn new(
        url: impl AsRef<str>,
        pool_options: Option<PoolOptions<Sqlite>>,
    ) -> Result<Self, Error> {
        let connect_options =
            SqliteConnectOptions::from_str(url.as_ref()).map_err(|e| Error::InvalidUrlError(e))?;
        let pool = match pool_options {
            Some(p) => p
                .connect_with(connect_options)
                .await
                .map_err(|e| Error::CreatePoolError(e))?,
            None => SqlitePool::connect_with(connect_options)
                .await
                .map_err(|e| Error::CreatePoolError(e))?,
        };
        Ok(Self(pool))
    }

    pub(crate) async fn select_u32s(&self, sql: &'static str) -> Result<Vec<u32>, Error> {
        let mut conn = self.acquire_conn().await?;
        query_as::<_, (u32,)>(sql)
            .fetch(&mut conn)
            .map_ok(|row| row.0)
            .try_collect()
            .await
            .map_err(|e| Error::SelectU32Error(e))
    }

    pub(crate) async fn select_strings(
        &self,
        sql: &'static str,
        param: String,
    ) -> Result<Vec<String>, Error> {
        let mut conn = self.acquire_conn().await?;
        query_as::<_, (String,)>(sql)
            .bind(param)
            .fetch(&mut conn)
            .map_ok(|row| row.0)
            .try_collect()
            .await
            .map_err(|e| Error::SelectStringError(e))
    }

    async fn acquire_conn(&self) -> Result<PoolConnection<Sqlite>, Error> {
        self.0
            .acquire()
            .await
            .map_err(|e| Error::AcquireConnectionError(e))
    }
}
