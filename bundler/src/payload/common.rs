use serde::Deserialize;
use std::collections::BTreeMap;
use std::hash::Hash;

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Language {
    Vim,
    #[default]
    Lua,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Config {
    // Just config code.
    Simple(String),
    // Config code with language and args.
    Detail(ConfigDetail),
}
impl Default for Config {
    fn default() -> Self {
        Config::Simple(String::default())
    }
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct ConfigDetail {
    pub language: Language,
    pub code: String,
    pub args: BTreeMap<String, String>,
}
