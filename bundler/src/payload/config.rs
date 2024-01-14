use crate::payload::common::{Language, Target};
use crate::payload::eager;
use crate::payload::group;
use crate::payload::lazy;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Config {
    // Just config code.
    Simple(String),
    // Config code with language and args.
    Detail(DetailConfig),
}
impl Default for Config {
    fn default() -> Self {
        Config::Simple(String::default())
    }
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct DetailConfig {
    pub language: Language,
    pub code: String,
    pub args: Value,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct IdMapElement {
    pub plugin_id: String,
    pub package: String,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub target: Target,
    pub extra_packages: Vec<String>,
    pub bundler_bin: String,
    pub id_map: Vec<IdMapElement>,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct AfterOption {
    pub ftplugin: HashMap<String, String>,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct BundlerConfig {
    pub eager_plugins: Vec<eager::VimPluginPackage>,
    pub lazy_plugins: Vec<lazy::VimPluginPackage>,
    pub lazy_groups: Vec<group::LazyGroup>,
    pub package: String,
    pub with_node_js: bool,
    pub with_python3: bool,
    pub with_ruby: bool,
    pub after: AfterOption,
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub config: BundlerConfig,
    pub meta: Meta,
}
