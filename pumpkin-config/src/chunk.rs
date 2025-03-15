use std::str;

use serde::{Deserialize, Serialize};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Default, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(default)]
pub struct ChunkConfig {
    pub compression: ChunkCompression,
    pub format: ChunkFormat,
    pub write_in_place: bool,
}

#[derive(Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ChunkCompression {
    pub algorithm: Compression,
    pub level: u32,
}

impl Default for ChunkCompression {
    fn default() -> Self {
        Self {
            algorithm: Compression::LZ4,
            level: 6,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum Compression {
    /// GZip Compression
    GZip,
    /// ZLib Compression
    ZLib,
    /// LZ4 Compression (since 24w04a)
    LZ4,
    /// Custom compression algorithm (since 24w05a)
    Custom,
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum ChunkFormat {
    #[default]
    Anvil,
    Linear,
}
