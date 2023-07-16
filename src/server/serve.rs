use super::{
    accessors::{Accessor, AccessorWrapper},
    error::Error,
    pb,
    service::Service,
    validator::Validator,
};
use regex::Regex;
use tonic;
use tonic_web;

pub async fn serve(
    accessor: impl Accessor,
    client_id: String,
    address: std::net::SocketAddr,
    server: &mut tonic::transport::Server,
    http1: bool,
    weve_esi_address: &str,
    buyback_client_address: String,
    buyback_corp: pb::weve_esi::Entity,
    buyback_contract_structure_token: String,
) -> Result<(), Error> {
    let service = pb::item_configurator_server::ItemConfiguratorServer::new(Service {
        accessor: AccessorWrapper(accessor),
        validator: Validator::new(client_id).await?,
        weve_esi_client: pb::WeveEsiClient(prost_twirp::HyperClient::new(
            hyper::Client::new(),
            weve_esi_address,
        )),
        buyback_client: pb::buyback_client::BuybackClient::connect(buyback_client_address).await?,
        buyback_contract_regex: Regex::new(r"[0123456789abcdef]{15,16}").unwrap(),
        buyback_corp: buyback_corp,
        buyback_contract_structure_token: buyback_contract_structure_token,
    });
    let router = match http1 {
        true => server.add_service(tonic_web::enable(service)),
        false => server.add_service(service),
    };
    router.serve(address).await?;
    Ok(())
}
