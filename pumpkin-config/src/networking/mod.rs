use auth::AuthenticationConfig;
use proxy::ProxyConfig;
use query::QueryConfig;
use rcon::RCONConfig;
use serde::{Deserialize, Serialize};

use crate::{CompressionConfig, LANBroadcastConfig};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

pub mod auth;
pub mod compression;
pub mod lan_broadcast;
pub mod proxy;
pub mod query;
pub mod rcon;

#[derive(Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct NetworkingConfig {
    pub authentication: AuthenticationConfig,
    pub query: QueryConfig,
    pub rcon: RCONConfig,
    pub proxy: ProxyConfig,
    pub packet_compression: CompressionConfig,
    pub lan_broadcast: LANBroadcastConfig,
}
