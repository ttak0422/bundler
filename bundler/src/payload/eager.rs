use serde::Deserialize;

use crate::payload::config::Config;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum VimPluginPackage {
    SimplePackage(String),
    ConfiguredPackage(PluginConfig),
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct PluginConfig {
    pub plugin: String,
    pub startup_config: Config,
    pub extra_packages: Vec<String>,
}
