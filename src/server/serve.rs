use super::{
    accessors::{Accessor, AccessorWrapper},
    error::Error,
    pb,
    service::Service,
    validator::Validator,
};
use tonic;
use tonic_web;

pub async fn serve(
    accessor: impl Accessor,
    client_id: String,
    address: std::net::SocketAddr,
    server: &mut tonic::transport::Server,
    http1: bool,
) -> Result<(), Error> {
    let service = pb::server::ItemConfiguratorServer::new(Service {
        accessor: AccessorWrapper(accessor),
        validator: Validator::new(client_id).await?,
    });
    let router = match http1 {
        true => server.add_service(tonic_web::enable(service)),
        false => server.add_service(service),
    };
    router.serve(address).await?;
    Ok(())
}
