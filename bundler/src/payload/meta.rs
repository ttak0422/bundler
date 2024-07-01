use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// [String(pname), String(Package)]
    pub package_paths: HashMap<String, String>,
    pub extra_packages: Vec<String>,
}
