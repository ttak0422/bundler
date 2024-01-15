use crate::payload::{config::Config, lazy};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct LazyGroup {
    pub name: String,
    pub plugins: Vec<lazy::VimPluginPackage>,
    pub startup_config: Config,
    pub extra_packages: Vec<String>,
    pub pre_config: Config,
    pub post_config: Config,
    pub depend_plugins: Vec<lazy::VimPluginPackage>,
    pub depend_groups: Vec<String>,
    pub on_modules: Vec<String>,
    pub on_events: Vec<String>,
    pub on_filetypes: Vec<String>,
    pub on_commands: Vec<String>,
    pub use_timer: bool,
}
