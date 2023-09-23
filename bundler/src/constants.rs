#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    Vim,
    #[default]
    Lua,
    Fennel,
}

pub mod dir {
    pub static PLUGIN: &str = "plugin";
    pub static PLUGINS: &str = "plugins";
    pub static PRE_CONFIG: &str = "pre_config";
    pub static CONFIG: &str = "config";
    pub static DEPENDS: &str = "depends";
    pub static DEPEND_BUNDLES: &str = "depend_bundles";
    pub static MODULES: &str = "modules";
    pub static EVENTS: &str = "events";
    pub static FILETYPES: &str = "filetypes";
    pub static COMMANDS: &str = "commands";
}

pub mod file {
    pub static STARTUP: &str = "startup";
    pub static MODULES: &str = "module_keys";
    pub static EVENTS: &str = "event_keys";
    pub static FILETYPES: &str = "filetype_keys";
    pub static COMMANDS: &str = "command_keys";
    pub static LAZYS: &str = "lazys";
}
