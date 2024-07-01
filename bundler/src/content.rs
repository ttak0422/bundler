pub mod common;
pub mod hashable;
pub mod meta;
pub mod plugin;
pub mod vim;

use crate::content::hashable::Hashable;
use crate::content::plugin::PluginConfig;
use crate::payload;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Content {
    pub plugin_configs: Vec<PluginConfig>,
    pub vim: vim::Vim,
    pub meta: meta::Meta,
}

impl From<payload::Payload> for Content {
    // plugin configs
    fn from(value: payload::Payload) -> Self {
        let mut eager_plugin_configs: Vec<PluginConfig> = value
            .plugin_config
            .eager
            .into_values()
            .map(PluginConfig::from)
            .collect::<Vec<_>>();
        let mut lazy_plugin_configs: Vec<PluginConfig> = value
            .plugin_config
            .lazy
            .into_values()
            .flat_map(Vec::<PluginConfig>::from)
            .collect::<Vec<_>>();
        let mut plugin_configs = vec![];
        plugin_configs.append(&mut eager_plugin_configs);
        plugin_configs.append(&mut lazy_plugin_configs);
        plugin_configs.dedup();

        // vim configs
        let vim = vim::Vim::from(value.vim_config);

        // other configs
        let meta = meta::Meta::from(value.meta);

        Content {
            plugin_configs,
            vim,
            meta,
        }
    }
}
