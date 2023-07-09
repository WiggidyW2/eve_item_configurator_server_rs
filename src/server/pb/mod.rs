pub mod item_configurator_proto;
pub use item_configurator_proto as item_configurator;
pub use item_configurator_proto::item_configurator_server;

pub mod buyback_proto;
pub use buyback_proto as buyback;
pub use buyback_proto::buyback_client;

pub mod weve_esi_proto;
pub use weve_esi_proto as weve_esi;
pub use weve_esi_proto::WeveEsiClient;

impl item_configurator::AuthKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
        }
    }
}

impl item_configurator::AuthScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Items => "items",
            Self::Characters => "characters",
            Self::Contracts => "contracts",
        }
    }
}
