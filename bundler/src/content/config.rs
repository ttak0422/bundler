use crate::constants::Language;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginConfig<'a> {
    pub lang: Language,
    pub code: &'a str,
    pub args: &'a Value,
}

impl Default for PluginConfig<'_> {
    fn default() -> Self {
        PluginConfig {
            lang: Language::default(),
            code: "",
            args: &Value::Null,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StartPlugin<'a> {
    pub id: &'a str,
    pub startup: PluginConfig<'a>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OptPlugin<'a> {
    pub id: &'a str,
    pub startup: PluginConfig<'a>,
    pub pre_config: PluginConfig<'a>,
    pub config: PluginConfig<'a>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bundle<'a> {
    pub id: &'a str,
    pub plugins: Vec<&'a str>,
    pub startup: PluginConfig<'a>,
    pub pre_config: PluginConfig<'a>,
    pub config: PluginConfig<'a>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LoadingOptions<'a> {
    pub depends: HashMap<&'a str, Vec<&'a str>>,
    pub depend_bundles: HashMap<&'a str, Vec<&'a str>>,
    pub modules: HashMap<&'a str, Vec<&'a str>>,
    pub events: HashMap<&'a str, Vec<&'a str>>,
    pub filetypes: HashMap<&'a str, Vec<&'a str>>,
    pub commands: HashMap<&'a str, Vec<&'a str>>,
    pub lazys: Vec<&'a str>,
    pub denops_clients: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Specs<'a> {
    /// key is package (e.g. `/nix/store/...`), value is id.
    pub id_map: HashMap<&'a str, &'a str>,
    pub start_plugins: Vec<StartPlugin<'a>>,
    pub opt_plugins: Vec<OptPlugin<'a>>,
    pub bundles: Vec<Bundle<'a>>,
    pub load_opt: LoadingOptions<'a>,
}
