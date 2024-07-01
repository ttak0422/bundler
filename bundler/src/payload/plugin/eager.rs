use crate::payload::common::Config;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub package: Option<String>,
    pub packages: Vec<String>,
    pub extra_packages: Vec<String>,
    pub startup_config: Config,
}
