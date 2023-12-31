use crate::constants::Language;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginConfig<'a> {
    pub language: Language,
    pub code: &'a str,
    pub args: &'a Value,
}

impl Default for PluginConfig<'_> {
    fn default() -> Self {
        PluginConfig {
            language: Language::default(),
            code: "",
            args: &Value::Null,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EagerPlugin<'a> {
    pub plugin_id: &'a str,
    pub startup_config: PluginConfig<'a>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LazyPlugin<'a> {
    pub plugin_id: &'a str,
    pub startup_config: PluginConfig<'a>,
    pub pre_config: PluginConfig<'a>,
    pub config: PluginConfig<'a>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LazyGroup<'a> {
    pub group_id: &'a str,
    pub plugin_ids: Vec<&'a str>,
    pub startup_config: PluginConfig<'a>,
    pub pre_config: PluginConfig<'a>,
    pub post_config: PluginConfig<'a>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LoadOption<'a> {
    pub depend_plugins: HashMap<&'a str, Vec<&'a str>>,
    pub depend_groups: HashMap<&'a str, Vec<&'a str>>,
    pub on_modules: HashMap<&'a str, Vec<&'a str>>,
    pub on_events: HashMap<&'a str, Vec<&'a str>>,
    pub on_filetypes: HashMap<&'a str, Vec<&'a str>>,
    pub on_commands: HashMap<&'a str, Vec<&'a str>>,
    pub timer_clients: Vec<&'a str>,
    pub denops_clients: Vec<&'a str>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AfterOption<'a> {
    pub ftplugin: HashMap<&'a str, &'a str>,
}

#[derive(Debug)]
pub struct Specs<'a> {
    /// key is package (e.g. `/nix/store/...`), value is id.
    pub id_map: HashMap<&'a str, &'a str>,
    pub eager_plugins: Vec<EagerPlugin<'a>>,
    pub lazy_plugins: Vec<LazyPlugin<'a>>,
    pub lazy_groups: Vec<LazyGroup<'a>>,
    pub load_option: LoadOption<'a>,
    pub after_option: AfterOption<'a>,
}
