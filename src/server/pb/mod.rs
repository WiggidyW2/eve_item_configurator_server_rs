mod item_configurator_proto;
pub use item_configurator_proto::*;
pub use item_configurator_server as server;

impl AuthKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
        }
    }
}

impl AuthScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Items => "items",
            Self::Characters => "characters",
            Self::Contracts => "contracts",
        }
    }
}
