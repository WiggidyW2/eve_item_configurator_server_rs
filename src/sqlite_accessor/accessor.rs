use super::{client::Client, error::Error, queries};
use crate::server::{
    CategoryGetter, DivisionNames, GroupGetter, MarketGroupGetter, NameGetter, TypeIdGetter,
};
use tonic::async_trait;

#[async_trait]
impl TypeIdGetter for Client {
    type Error = Error;
    async fn get_type_ids(&self) -> Result<Vec<u32>, Self::Error> {
        self.select_u32s(queries::SELECT_TYPE_ID).await
    }
}

#[async_trait]
impl NameGetter for Client {
    type Error = Error;
    async fn get_names(&self, language: &str) -> Result<Vec<String>, Self::Error> {
        self.select_strings(queries::SELECT_TYPE_NAME, language.to_string())
            .await
    }
}

#[async_trait]
impl MarketGroupGetter for Client {
    type Error = Error;
    async fn get_market_groups(&self, language: &str) -> Result<DivisionNames, Self::Error> {
        let indexes_fut = self.select_u32s(queries::SELECT_MARKET_GROUP_IDX);
        let names_fut =
            self.select_strings(queries::SELECT_MARKET_GROUP_NAME, language.to_string());
        Ok(DivisionNames {
            indexes: indexes_fut.await?,
            names: names_fut.await?,
        })
    }
}

#[async_trait]
impl CategoryGetter for Client {
    type Error = Error;
    async fn get_categories(&self, language: &str) -> Result<DivisionNames, Self::Error> {
        let indexes_fut = self.select_u32s(queries::SELECT_CATEGORY_IDX);
        let names_fut = self.select_strings(queries::SELECT_CATEGORY_NAME, language.to_string());
        Ok(DivisionNames {
            indexes: indexes_fut.await?,
            names: names_fut.await?,
        })
    }
}

#[async_trait]
impl GroupGetter for Client {
    type Error = Error;
    async fn get_groups(&self, language: &str) -> Result<DivisionNames, Self::Error> {
        let indexes_fut = self.select_u32s(queries::SELECT_GROUP_IDX);
        let names_fut = self.select_strings(queries::SELECT_GROUP_NAME, language.to_string());
        Ok(DivisionNames {
            indexes: indexes_fut.await?,
            names: names_fut.await?,
        })
    }
}
