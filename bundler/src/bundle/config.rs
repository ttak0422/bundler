use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Component<'a> {
    pub id: &'a str,
    pub is_plugin: bool,
    pub startup_config: &'a str,
    pub pre_config: &'a str,
    pub post_config: &'a str,
    pub depend_plugins: Vec<&'a str>,
    pub depend_groups: Vec<&'a str>,
    pub group_plugins: Vec<&'a str>,
}

#[derive(Default)]
pub struct LoadOption<'a> {
    pub plugin_paths: HashMap<PluginId<'a>, PluginPath<'a>>,
    pub startup_config_plugins: Vec<&'a str>,
    pub on_modules: HashMap<&'a str, Vec<&'a str>>,
    pub on_events: HashMap<&'a str, Vec<&'a str>>,
    pub on_filetypes: HashMap<&'a str, Vec<&'a str>>,
    pub on_commands: HashMap<&'a str, Vec<&'a str>>,
    pub timer_clients: Vec<&'a str>,
    pub denops_clients: Vec<&'a str>,
}

pub struct AfterOption<'a> {
    pub ftplugin: HashMap<&'a str, &'a str>,
}

pub struct Info<'a> {
    pub bundler_bin: &'a str,
}

pub type PluginId<'a> = &'a str;

pub type PluginPath<'a> = &'a str;

pub struct Bundle<'a> {
    pub components: Vec<Component<'a>>,
    pub load_option: LoadOption<'a>,
    pub after_option: AfterOption<'a>,
    pub info: Info<'a>,
}
