use super::super::{accessors::Accessor, pb};
use super::service::Service;
use tonic::async_trait;

#[async_trait]
impl<A: Accessor> pb::item_configurator_server::ItemConfigurator for Service<A> {
    async fn update(
        &self,
        request: tonic::Request<pb::item_configurator::UpdateReq>,
    ) -> Result<tonic::Response<pb::item_configurator::UpdateRep>, tonic::Status> {
        match self.update_items(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
    async fn list(
        &self,
        request: tonic::Request<pb::item_configurator::ListReq>,
    ) -> Result<tonic::Response<pb::item_configurator::ListRep>, tonic::Status> {
        match self.list_items(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
    async fn list_characters(
        &self,
        request: tonic::Request<pb::item_configurator::ListCharactersReq>,
    ) -> Result<tonic::Response<pb::item_configurator::ListCharactersRep>, tonic::Status> {
        match self.list_characters(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
    async fn add_characters(
        &self,
        request: tonic::Request<pb::item_configurator::AddCharactersReq>,
    ) -> Result<tonic::Response<pb::item_configurator::AddCharactersRep>, tonic::Status> {
        match self.add_characters(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
    async fn del_characters(
        &self,
        request: tonic::Request<pb::item_configurator::DelCharactersReq>,
    ) -> Result<tonic::Response<pb::item_configurator::DelCharactersRep>, tonic::Status> {
        match self.del_characters(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
    async fn buyback_contracts(
        &self,
        request: tonic::Request<pb::item_configurator::BuybackContractsReq>,
    ) -> Result<tonic::Response<pb::item_configurator::BuybackContractsRep>, tonic::Status> {
        match self.list_bb_contracts(request.into_inner()).await {
            Ok(rep) => Ok(tonic::Response::new(rep)),
            Err(e) => Err(e.into()),
        }
    }
}
