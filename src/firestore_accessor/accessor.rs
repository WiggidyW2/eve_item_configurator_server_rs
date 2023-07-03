use super::{characters::Characters, client::Client, error::Error};
use crate::server::{CharacterGetter, CharacterSetter};
use tonic::async_trait;

#[async_trait]
impl CharacterGetter for Client {
    type Error = Error;
    async fn get_characters(&self, config_name: &str) -> Result<Vec<String>, Self::Error> {
        let characters: Option<Characters> = self.read(config_name).await?;
        Ok(match characters {
            Some(characters) => characters.into_inner(),
            None => Vec::new(),
        })
    }
}

#[async_trait]
impl CharacterSetter for Client {
    type Error = Error;
    async fn set_characters(
        &self,
        config_name: &str,
        characters: Vec<String>,
    ) -> Result<(), Self::Error> {
        let characters = Characters::new(characters);
        self.write(config_name, characters).await
    }
}
