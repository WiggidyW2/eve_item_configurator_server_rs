#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrder {
    #[prost(int64, tag = "1")]
    pub quantity: i64,
    #[prost(double, tag = "2")]
    pub price: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrdersRep {
    /// no particular order
    #[prost(message, repeated, tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<MarketOrder>,
}
/// Request and Response Types
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrdersReq {
    #[prost(uint64, tag = "1")]
    pub location_id: u64,
    #[prost(uint32, tag = "2")]
    pub type_id: u32,
    #[prost(string, tag = "3")]
    pub token: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub buy: bool,
}
/// uint32 is typeid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdjustedPriceRep {
    #[prost(map = "uint32, double", tag = "1")]
    pub inner: ::std::collections::HashMap<u32, f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdjustedPriceReq {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemIndex {
    #[prost(double, tag = "1")]
    pub manufacturing: f64,
    #[prost(double, tag = "2")]
    pub research_te: f64,
    #[prost(double, tag = "3")]
    pub research_me: f64,
    #[prost(double, tag = "4")]
    pub copying: f64,
    #[prost(double, tag = "5")]
    pub invention: f64,
    #[prost(double, tag = "6")]
    pub reactions: f64,
}
/// uint32 is systemid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemIndexRep {
    #[prost(map = "uint32, message", tag = "1")]
    pub inner: ::std::collections::HashMap<u32, SystemIndex>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemIndexReq {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndustryJob {
    #[prost(uint64, tag = "1")]
    pub location_id: u64,
    #[prost(uint64, tag = "2")]
    pub character_id: u64,
    /// unix timestamp of when it started
    #[prost(uint64, tag = "3")]
    pub start: u64,
    /// unix timestamp of when it finishes
    #[prost(uint64, tag = "4")]
    pub finish: u64,
    /// always 1.0 unless invention
    #[prost(double, tag = "5")]
    pub probability: f64,
    /// 0 if ME/TE research
    #[prost(uint32, tag = "6")]
    pub product_id: u32,
    #[prost(uint32, tag = "7")]
    pub blueprint_id: u32,
    /// blueprint ME
    #[prost(int32, tag = "8")]
    pub material_efficiency: i32,
    /// blueprint TE
    #[prost(int32, tag = "9")]
    pub time_efficiency: i32,
    #[prost(int32, tag = "10")]
    pub activity: i32,
    #[prost(int32, tag = "11")]
    pub runs: i32,
    #[prost(bool, tag = "12")]
    pub is_bpc: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndustryJobsRep {
    #[prost(message, repeated, tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<IndustryJob>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndustryJobsReq {
    #[prost(message, repeated, tag = "1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag = "2")]
    pub corporations: ::prost::alloc::vec::Vec<Entity>,
}
/// assets and blueprints
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    #[prost(uint64, tag = "1")]
    pub entity_id: u64,
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    #[prost(int32, tag = "3")]
    pub runs: i32,
    #[prost(int32, tag = "4")]
    pub material_efficiency: i32,
    #[prost(int32, tag = "5")]
    pub time_efficiency: i32,
    /// may include BPC or Singleton // NEVERMIND
    #[prost(string, repeated, tag = "6")]
    pub flags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeAssets {
    #[prost(message, repeated, tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<Asset>,
}
/// uint32 is typeid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationAssets {
    #[prost(map = "uint32, message", tag = "1")]
    pub inner: ::std::collections::HashMap<u32, TypeAssets>,
}
/// only includes station assets, uint64 is locationid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsRep {
    #[prost(map = "uint64, message", tag = "1")]
    pub inner: ::std::collections::HashMap<u64, LocationAssets>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsReq {
    #[prost(message, repeated, tag = "1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag = "2")]
    pub corporations: ::prost::alloc::vec::Vec<Entity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Skills {
    #[prost(map = "uint32, uint32", tag = "1")]
    pub inner: ::std::collections::HashMap<u32, u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkillsRep {
    #[prost(map = "uint64, message", tag = "1")]
    pub inner: ::std::collections::HashMap<u64, Skills>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkillsReq {
    #[prost(message, repeated, tag = "1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveOrder {
    #[prost(bool, tag = "1")]
    pub buy: bool,
    #[prost(double, tag = "2")]
    pub price: f64,
    /// bool buy = 1;
    /// uint64 location_id = 2;
    /// uint32 type_id = 3;
    /// double price = 4;
    /// int64 quantity = 5;
    #[prost(int64, tag = "3")]
    pub quantity: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeActiveOrders {
    #[prost(message, repeated, tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<ActiveOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationActiveOrders {
    #[prost(map = "uint32, message", tag = "1")]
    pub inner: ::std::collections::HashMap<u32, TypeActiveOrders>,
}
/// shouldn't include duplicates, uint64 is locationid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveOrdersRep {
    #[prost(map = "uint64, message", tag = "1")]
    pub inner: ::std::collections::HashMap<u64, LocationActiveOrders>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveOrdersReq {
    #[prost(message, repeated, tag = "1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag = "2")]
    pub corporations: ::prost::alloc::vec::Vec<Entity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(bool, tag = "1")]
    pub buy: bool,
    #[prost(double, tag = "2")]
    pub price: f64,
    #[prost(int64, tag = "3")]
    pub quantity: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeTransactions {
    #[prost(message, repeated, tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationTransactions {
    #[prost(map = "uint32, message", tag = "1")]
    pub inner: ::std::collections::HashMap<u32, TypeTransactions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsRep {
    #[prost(map = "uint64, message", tag = "1")]
    pub inner: ::std::collections::HashMap<u64, LocationTransactions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsReq {
    #[prost(message, repeated, tag = "1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag = "2")]
    pub corporations: ::prost::alloc::vec::Vec<Entity>,
    #[prost(uint64, tag = "3")]
    pub since: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeContractItem {
    #[prost(uint32, tag = "1")]
    pub type_id: u32,
    #[prost(int64, tag = "2")]
    pub quantity: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeContract {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<ExchangeContractItem>,
    #[prost(uint64, tag = "2")]
    pub location_id: u64,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(double, tag = "4")]
    pub price: f64,
    #[prost(double, tag = "5")]
    pub reward: f64,
    #[prost(uint64, tag = "6")]
    pub expires: u64,
    #[prost(uint64, tag = "7")]
    pub issued: u64,
    #[prost(double, tag = "8")]
    pub volume: f64,
    #[prost(uint32, tag = "9")]
    pub char_id: u32,
    #[prost(uint32, tag = "10")]
    pub corp_id: u32,
    #[prost(bool, tag = "11")]
    pub is_corp: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeContractsRep {
    #[prost(message, repeated, tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<ExchangeContract>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeContractsReq {
    #[prost(message, repeated, tag = "1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag = "2")]
    pub corporations: ::prost::alloc::vec::Vec<Entity>,
    #[prost(bool, tag = "3")]
    pub active_only: bool,
}
pub use ::prost_twirp::ServiceRequest;
pub use ::prost_twirp::PTRes;
pub trait WeveEsi: Send + Sync + 'static {
    fn active_orders(
        &self,
        request: ::prost_twirp::ServiceRequest<ActiveOrdersReq>,
    ) -> ::prost_twirp::PTRes<ActiveOrdersRep>;
    fn adjusted_price(
        &self,
        request: ::prost_twirp::ServiceRequest<AdjustedPriceReq>,
    ) -> ::prost_twirp::PTRes<AdjustedPriceRep>;
    fn assets(
        &self,
        request: ::prost_twirp::ServiceRequest<AssetsReq>,
    ) -> ::prost_twirp::PTRes<AssetsRep>;
    fn industry_jobs(
        &self,
        request: ::prost_twirp::ServiceRequest<IndustryJobsReq>,
    ) -> ::prost_twirp::PTRes<IndustryJobsRep>;
    fn market_orders(
        &self,
        request: ::prost_twirp::ServiceRequest<MarketOrdersReq>,
    ) -> ::prost_twirp::PTRes<MarketOrdersRep>;
    fn skills(
        &self,
        request: ::prost_twirp::ServiceRequest<SkillsReq>,
    ) -> ::prost_twirp::PTRes<SkillsRep>;
    fn system_index(
        &self,
        request: ::prost_twirp::ServiceRequest<SystemIndexReq>,
    ) -> ::prost_twirp::PTRes<SystemIndexRep>;
    fn transactions(
        &self,
        request: ::prost_twirp::ServiceRequest<TransactionsReq>,
    ) -> ::prost_twirp::PTRes<TransactionsRep>;
    fn exchange_contracts(
        &self,
        request: ::prost_twirp::ServiceRequest<ExchangeContractsReq>,
    ) -> ::prost_twirp::PTRes<ExchangeContractsRep>;
}
impl dyn WeveEsi {
    /// Construct a new client stub for the service.
    ///
    /// The client's implementation of the trait methods will make HTTP requests to the
    /// server addressed by `client`.
    #[allow(dead_code)]
    pub fn new_client(
        client: ::hyper::Client<::hyper::client::HttpConnector, ::hyper::Body>,
        root_url: &str,
    ) -> Box<dyn WeveEsi> {
        Box::new(WeveEsiClient(::prost_twirp::HyperClient::new(client, root_url)))
    }
    /// Make a new server for the service.
    ///
    /// Method calls are forwarded to the implementation in `v`.
    ///
    /// Due to <https://github.com/hyperium/hyper/issues/2051> this can't be directly
    /// passed to `Service::serve`.
    #[allow(dead_code)]
    #[allow(clippy::type_complexity)]
    pub fn new_server<T: WeveEsi>(
        v: T,
    ) -> Box<
        dyn (::hyper::service::Service<
            ::hyper::Request<::hyper::Body>,
            Response = ::hyper::Response<::hyper::Body>,
            Error = ::hyper::Error,
            Future = ::std::pin::Pin<
                Box<
                    dyn (::futures::Future<
                        Output = Result<::hyper::Response<::hyper::Body>, ::hyper::Error>,
                    >) + Send,
                >,
            >,
        >) + Send + Sync,
    > {
        Box::new(
            ::prost_twirp::HyperServer::new(WeveEsiServer(::std::sync::Arc::new(v))),
        )
    }
}
pub struct WeveEsiClient(pub ::prost_twirp::HyperClient);
impl WeveEsi for WeveEsiClient {
    fn active_orders(
        &self,
        request: ::prost_twirp::ServiceRequest<ActiveOrdersReq>,
    ) -> ::prost_twirp::PTRes<ActiveOrdersRep> {
        self.0.go("/twirp/weve_esi_proto.WeveEsi/ActiveOrders", request)
    }
    fn adjusted_price(
        &self,
        request: ::prost_twirp::ServiceRequest<AdjustedPriceReq>,
    ) -> ::prost_twirp::PTRes<AdjustedPriceRep> {
        self.0.go("/twirp/weve_esi_proto.WeveEsi/AdjustedPrice", request)
    }
    fn assets(
        &self,
        request: ::prost_twirp::ServiceRequest<AssetsReq>,
    ) -> ::prost_twirp::PTRes<AssetsRep> {
        self.0.go("/twirp/weve_esi_proto.WeveEsi/Assets", request)
    }
    fn industry_jobs(
        &self,
        request: ::prost_twirp::ServiceRequest<IndustryJobsReq>,
    ) -> ::prost_twirp::PTRes<IndustryJobsRep> {
        self.0.go("/twirp/weve_esi_proto.WeveEsi/IndustryJobs", request)
    }
    fn market_orders(
        &self,
        request: ::prost_twirp::ServiceRequest<MarketOrdersReq>,
    ) -> ::prost_twirp::PTRes<MarketOrdersRep> {
        self.0.go("/twirp/weve_esi_proto.WeveEsi/MarketOrders", request)
    }
    fn skills(
        &self,
        request: ::prost_twirp::ServiceRequest<SkillsReq>,
    ) -> ::prost_twirp::PTRes<SkillsRep> {
        self.0.go("/twirp/weve_esi_proto.WeveEsi/Skills", request)
    }
    fn system_index(
        &self,
        request: ::prost_twirp::ServiceRequest<SystemIndexReq>,
    ) -> ::prost_twirp::PTRes<SystemIndexRep> {
        self.0.go("/twirp/weve_esi_proto.WeveEsi/SystemIndex", request)
    }
    fn transactions(
        &self,
        request: ::prost_twirp::ServiceRequest<TransactionsReq>,
    ) -> ::prost_twirp::PTRes<TransactionsRep> {
        self.0.go("/twirp/weve_esi_proto.WeveEsi/Transactions", request)
    }
    fn exchange_contracts(
        &self,
        request: ::prost_twirp::ServiceRequest<ExchangeContractsReq>,
    ) -> ::prost_twirp::PTRes<ExchangeContractsRep> {
        self.0.go("/twirp/weve_esi_proto.WeveEsi/ExchangeContracts", request)
    }
}
pub struct WeveEsiServer<T: WeveEsi>(::std::sync::Arc<T>);
impl<T: WeveEsi> ::prost_twirp::HyperService for WeveEsiServer<T> {
    fn handle(
        &self,
        req: ::hyper::Request<::hyper::Body>,
    ) -> ::std::pin::Pin<
        Box<
            dyn ::futures::Future<
                Output = Result<
                    ::hyper::Response<::hyper::Body>,
                    ::prost_twirp::ProstTwirpError,
                >,
            > + Send + 'static,
        >,
    > {
        let static_service = ::std::sync::Arc::clone(&self.0);
        match req.uri().path() {
            "/twirp/weve_esi_proto.WeveEsi/ActiveOrders" => {
                Box::pin(async move {
                    let req = ::prost_twirp::ServiceRequest::from_hyper_request(req)
                        .await?;
                    static_service.active_orders(req).await?.to_hyper_response()
                })
            }
            "/twirp/weve_esi_proto.WeveEsi/AdjustedPrice" => {
                Box::pin(async move {
                    let req = ::prost_twirp::ServiceRequest::from_hyper_request(req)
                        .await?;
                    static_service.adjusted_price(req).await?.to_hyper_response()
                })
            }
            "/twirp/weve_esi_proto.WeveEsi/Assets" => {
                Box::pin(async move {
                    let req = ::prost_twirp::ServiceRequest::from_hyper_request(req)
                        .await?;
                    static_service.assets(req).await?.to_hyper_response()
                })
            }
            "/twirp/weve_esi_proto.WeveEsi/IndustryJobs" => {
                Box::pin(async move {
                    let req = ::prost_twirp::ServiceRequest::from_hyper_request(req)
                        .await?;
                    static_service.industry_jobs(req).await?.to_hyper_response()
                })
            }
            "/twirp/weve_esi_proto.WeveEsi/MarketOrders" => {
                Box::pin(async move {
                    let req = ::prost_twirp::ServiceRequest::from_hyper_request(req)
                        .await?;
                    static_service.market_orders(req).await?.to_hyper_response()
                })
            }
            "/twirp/weve_esi_proto.WeveEsi/Skills" => {
                Box::pin(async move {
                    let req = ::prost_twirp::ServiceRequest::from_hyper_request(req)
                        .await?;
                    static_service.skills(req).await?.to_hyper_response()
                })
            }
            "/twirp/weve_esi_proto.WeveEsi/SystemIndex" => {
                Box::pin(async move {
                    let req = ::prost_twirp::ServiceRequest::from_hyper_request(req)
                        .await?;
                    static_service.system_index(req).await?.to_hyper_response()
                })
            }
            "/twirp/weve_esi_proto.WeveEsi/Transactions" => {
                Box::pin(async move {
                    let req = ::prost_twirp::ServiceRequest::from_hyper_request(req)
                        .await?;
                    static_service.transactions(req).await?.to_hyper_response()
                })
            }
            "/twirp/weve_esi_proto.WeveEsi/ExchangeContracts" => {
                Box::pin(async move {
                    let req = ::prost_twirp::ServiceRequest::from_hyper_request(req)
                        .await?;
                    static_service.exchange_contracts(req).await?.to_hyper_response()
                })
            }
            _ => {
                Box::pin(
                    ::futures::future::ok(
                        ::prost_twirp::ProstTwirpError::NotFound
                            .into_hyper_response()
                            .unwrap(),
                    ),
                )
            }
        }
    }
}
