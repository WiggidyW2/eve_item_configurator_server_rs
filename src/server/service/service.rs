use super::super::{
    accessors::{Accessor, AccessorWrapper},
    error::Error,
    pb,
    validator::Validator,
};
use crate::character_validator::Response as ValidatorResponse;
use regex::Regex;

pub struct Service<A> {
    pub accessor: AccessorWrapper<A>,
    pub validator: Validator,
    pub weve_esi_client: pb::WeveEsiClient,
    pub buyback_client: pb::buyback_client::BuybackClient<tonic::transport::Channel>,
    pub buyback_contract_regex: Regex,
    pub buyback_corp: pb::weve_esi::Entity,
    pub buyback_contract_structure_token: String,
}

impl<A: Accessor> Service<A> {
    pub async fn authorize(
        &self,
        name: &str,
        token: &str,
        kind: pb::item_configurator::AuthKind,
        scope: pb::item_configurator::AuthScope,
    ) -> Result<ValidatorResponse, Error> {
        // Format the name to match the authorization endpoint
        let name = get_auth_name(name, kind, scope);

        // Get the list of characters that are authorized for the endpoint
        let characters = self.accessor.get_characters(&name).await?;

        // Get the validator response
        let rep = self.validator.validate(token, characters.iter()).await?;

        // Return the validator response
        Ok(rep)
    }

    pub async fn list_items(
        &self,
        req: pb::item_configurator::ListReq,
    ) -> Result<pb::item_configurator::ListRep, Error> {
        let auth_rep = self
            .authorize(
                &req.name,
                &req.refresh_token,
                pb::item_configurator::AuthKind::Read,
                pb::item_configurator::AuthScope::Items,
            )
            .await?;
        if auth_rep.valid {
            self.list_items_authorized(req, auth_rep.refresh_token)
                .await
        } else {
            Ok(self.list_items_unauthorized(auth_rep.refresh_token))
        }
    }

    pub async fn update_items(
        &self,
        req: pb::item_configurator::UpdateReq,
    ) -> Result<pb::item_configurator::UpdateRep, Error> {
        let auth_rep = self
            .authorize(
                &req.name,
                &req.refresh_token,
                pb::item_configurator::AuthKind::Write,
                pb::item_configurator::AuthScope::Items,
            )
            .await?;
        if auth_rep.valid {
            self.update_items_authorized(req, auth_rep.refresh_token)
                .await
        } else {
            Ok(self.update_items_unauthorized(auth_rep.refresh_token))
        }
    }

    pub async fn list_characters(
        &self,
        req: pb::item_configurator::ListCharactersReq,
    ) -> Result<pb::item_configurator::ListCharactersRep, Error> {
        let auth_rep = self
            .authorize(
                &req.name,
                &req.refresh_token,
                pb::item_configurator::AuthKind::Read,
                pb::item_configurator::AuthScope::Characters,
            )
            .await?;
        if auth_rep.valid {
            self.list_characters_authorized(req, auth_rep.refresh_token)
                .await
        } else {
            Ok(self.list_characters_unauthorized(auth_rep.refresh_token))
        }
    }

    pub async fn del_characters(
        &self,
        req: pb::item_configurator::DelCharactersReq,
    ) -> Result<pb::item_configurator::DelCharactersRep, Error> {
        let auth_rep = self
            .authorize(
                &req.name,
                &req.refresh_token,
                pb::item_configurator::AuthKind::Write,
                pb::item_configurator::AuthScope::Characters,
            )
            .await?;
        if auth_rep.valid {
            self.del_characters_authorized(req, auth_rep.refresh_token)
                .await
        } else {
            Ok(self.del_characters_unauthorized(auth_rep.refresh_token))
        }
    }

    pub async fn add_characters(
        &self,
        req: pb::item_configurator::AddCharactersReq,
    ) -> Result<pb::item_configurator::AddCharactersRep, Error> {
        let auth_rep = self
            .authorize(
                &req.name,
                &req.refresh_token,
                pb::item_configurator::AuthKind::Write,
                pb::item_configurator::AuthScope::Characters,
            )
            .await?;
        if auth_rep.valid {
            self.add_characters_authorized(req, auth_rep.refresh_token)
                .await
        } else {
            Ok(self.add_characters_unauthorized(auth_rep.refresh_token))
        }
    }

    pub async fn list_bb_contracts(
        &self,
        req: pb::item_configurator::BuybackContractsReq,
    ) -> Result<pb::item_configurator::BuybackContractsRep, Error> {
        let auth_rep = self
            .authorize(
                "buyback",
                &req.refresh_token,
                pb::item_configurator::AuthKind::Read,
                pb::item_configurator::AuthScope::Contracts,
            )
            .await?;
        if auth_rep.valid {
            self.list_bb_contracts_authorized(req, auth_rep.refresh_token)
                .await
        } else {
            Ok(self.list_bb_contracts_unauthorized(auth_rep.refresh_token))
        }
    }
}

// Format the name to match the authorization endpoint
pub fn get_auth_name(
    name: &str,
    kind: pb::item_configurator::AuthKind,
    scope: pb::item_configurator::AuthScope,
) -> String {
    format!("{}_{}_{}", name, kind.as_str(), scope.as_str())
}
