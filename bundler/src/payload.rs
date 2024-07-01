pub mod common;
pub mod meta;
pub mod plugin;
pub mod vim;

use plugin::PluginConfig;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub vim_config: vim::VimConfig,
    pub plugin_config: PluginConfig,
    pub meta: meta::Meta,
}
