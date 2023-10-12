use serde::Deserialize;

use crate::payload::{core::Config, opt};

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    pub name: String,
    pub plugins: Vec<opt::VimPlugin>,
    pub startup: Config,
    pub extra_packages: Vec<String>,
    pub pre_config: Config,
    pub config: Config,
    pub depends: Vec<opt::VimPlugin>,
    pub depend_bundles: Vec<String>,
    pub modules: Vec<String>,
    pub events: Vec<String>,
    pub filetypes: Vec<String>,
    pub commands: Vec<String>,
    pub lazy: bool,
}
