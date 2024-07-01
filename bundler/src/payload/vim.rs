use crate::payload::common::Config;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct VimConfig {
    /// e.g.
    /// `after/XXXX/YYYY.lua`
    /// ```lua
    /// ZZZZ
    /// ```
    /// will store as
    /// ```rust
    /// after = hashmap! { "XXXX" => hashmap! { "YYYY" => "ZZZZ" }
    /// ```
    pub after: BTreeMap<String, BTreeMap<String, Config>>,
}
