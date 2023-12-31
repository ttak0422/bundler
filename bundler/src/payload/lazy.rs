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
    pub pre_config: Config,
    pub post_config: Config,
    pub depend_plugins: Vec<VimPluginPackage>,
    pub depend_groups: Vec<String>,
    pub on_modules: Vec<String>,
    pub on_events: Vec<String>,
    pub on_filetypes: Vec<String>,
    pub on_commands: Vec<String>,
    pub use_timer: bool,
    pub use_denops: bool,
}
