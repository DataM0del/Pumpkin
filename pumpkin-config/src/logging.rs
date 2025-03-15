#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(default)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub threads: bool,
    pub color: bool,
    pub timestamp: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threads: true,
            color: true,
            timestamp: true,
        }
    }
}
