use super::super::{accessors::Accessor, error::Error, pb};
use super::service::{get_auth_name, Service};

impl<A: Accessor> Service<A> {
    pub fn list_characters_unauthorized(
        &self,
        new_token: String,
    ) -> pb::item_configurator::ListCharactersRep {
        pb::item_configurator::ListCharactersRep {
            characters: Vec::new(),
            refresh_token: new_token,
            authorized: false,
        }
    }

    pub async fn list_characters_authorized(
        &self,
        req: pb::item_configurator::ListCharactersReq,
        new_token: String,
    ) -> Result<pb::item_configurator::ListCharactersRep, Error> {
        let name = get_auth_name_repr(&req.name, req.auth_kind, req.auth_scope);
        let characters = self.accessor.get_characters(&name).await?;
        Ok(pb::item_configurator::ListCharactersRep {
            refresh_token: new_token,
            authorized: true,
            characters,
        })
    }

    pub fn add_characters_unauthorized(
        &self,
        new_token: String,
    ) -> pb::item_configurator::AddCharactersRep {
        pb::item_configurator::AddCharactersRep {
            refresh_token: new_token,
            authorized: false,
        }
    }

    pub async fn add_characters_authorized(
        &self,
        req: pb::item_configurator::AddCharactersReq,
        new_token: String,
    ) -> Result<pb::item_configurator::AddCharactersRep, Error> {
        let name = get_auth_name_repr(&req.name, req.auth_kind, req.auth_scope);
        let mut characters = self.accessor.get_characters(&name).await?;
        characters.reserve(req.characters.len());
        req.characters.into_iter().for_each(|c| characters.push(c));
        characters.sort();
        characters.dedup();
        self.accessor.set_characters(&name, characters).await?;
        Ok(pb::item_configurator::AddCharactersRep {
            refresh_token: new_token,
            authorized: true,
        })
    }

    pub fn del_characters_unauthorized(
        &self,
        new_token: String,
    ) -> pb::item_configurator::DelCharactersRep {
        pb::item_configurator::DelCharactersRep {
            refresh_token: new_token,
            authorized: false,
        }
    }

    pub async fn del_characters_authorized(
        &self,
        req: pb::item_configurator::DelCharactersReq,
        new_token: String,
    ) -> Result<pb::item_configurator::DelCharactersRep, Error> {
        let name = get_auth_name_repr(&req.name, req.auth_kind, req.auth_scope);
        let mut characters = self.accessor.get_characters(&name).await?;
        characters.retain(|c| !req.characters.contains(c));
        self.accessor.set_characters(&name, characters).await?;
        Ok(pb::item_configurator::DelCharactersRep {
            refresh_token: new_token,
            authorized: true,
        })
    }
}

// Format the name to match the authorization endpoint
fn get_auth_name_repr(name: &str, kind: i32, scope: i32) -> String {
    get_auth_name(
        name,
        pb::item_configurator::AuthKind::from_i32(kind).unwrap(),
        pb::item_configurator::AuthScope::from_i32(scope).unwrap(),
    )
}
