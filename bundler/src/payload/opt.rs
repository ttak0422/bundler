use serde::Deserialize;

use crate::payload::core::Config;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum VimPlugin {
    SimplePackage(String),
    ConfiguredPackage(PluginConfig),
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct PluginConfig {
    pub plugin: String,
    pub startup: Config,
    pub pre_config: Config,
    pub config: Config,
    pub depends: Vec<VimPlugin>,
    pub depend_bundles: Vec<String>,
    pub modules: Vec<String>,
    pub events: Vec<String>,
    pub filetypes: Vec<String>,
    pub commands: Vec<String>,
    pub lazy: bool,
}
