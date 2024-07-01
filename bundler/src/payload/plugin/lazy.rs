use crate::payload::common::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum PackageOrComponent {
    Package(String),
    Component(Component),
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Hook {
    pub modules: Vec<String>,
    pub events: Vec<String>,
    pub user_events: Vec<String>,
    pub file_types: Vec<String>,
    pub commands: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub package: Option<String>,
    pub packages: Vec<String>,
    pub extra_packages: Vec<String>,
    pub depends: Vec<PackageOrComponent>,
    pub startup_config: Config,
    pub pre_config: Config,
    pub post_config: Config,
    pub hooks: Hook,
    pub use_denops: bool,
}
