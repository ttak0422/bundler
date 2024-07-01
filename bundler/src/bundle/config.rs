use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PluginConfig {
    /// incremental id (e.g. 1, 2, 3, ...).
    pub id: i32,
    /// pname.
    pub packages: Vec<String>,
    pub startup_config: Option<String>,
    pub pre_config: Option<String>,
    pub post_config: Option<String>,
    pub depends: Vec<i32>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LoadConfig {
    pub on_modules: HashMap<String, Vec<i32>>,
    pub on_events: HashMap<String, Vec<i32>>,
    pub on_userevents: HashMap<String, Vec<i32>>,
    pub on_filetypes: HashMap<String, Vec<i32>>,
    pub on_commands: HashMap<String, Vec<i32>>,
    pub startup_plugins: Vec<i32>,
    pub startup_config_plugins: Vec<i32>,
    pub denops_clients: Vec<i32>,
    // [(id, [path])]
    pub rtp: HashMap<i32, Vec<String>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Config {
    pub plugins: Vec<PluginConfig>,
    pub load_config: LoadConfig,
    pub after: HashMap<String, HashMap<String, String>>,
}
