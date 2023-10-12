use serde::Deserialize;
use serde_json::Value;

use crate::payload::bundle;
use crate::payload::opt;
use crate::payload::start;

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
        Config::Simple(String::from(""))
    }
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct DetailConfig {
    pub lang: Lang,
    pub code: String,
    pub args: Value,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Lang {
    Vim,
    #[default]
    Lua,
    Fennel,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct IdPackage {
    pub id: String,
    pub package: String,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub extra_packages: Vec<String>,
    pub id_map: Vec<IdPackage>,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct BundlerConfig {
    pub start_plugins: Vec<start::VimPlugin>,
    pub opt_plugins: Vec<opt::VimPlugin>,
    pub bundles: Vec<bundle::Bundle>,
    pub package: String,
    pub with_node_js: bool,
    pub with_python3: bool,
    pub with_ruby: bool,
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub cfg: BundlerConfig,
    pub meta: Meta,
}
