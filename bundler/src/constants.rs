use std::fmt::{self};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    Vim,
    #[default]
    Lua,
    Fennel,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Language::Vim => write!(f, "vim"),
            Language::Lua => write!(f, "lua"),
            Language::Fennel => write!(f, "fennel"),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Target {
    Vim,
    #[default]
    Neovim,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Target::Vim => write!(f, "vim"),
            Target::Neovim => write!(f, "neovim"),
        }
    }
}

pub mod dir {
    pub static PLUGIN: &str = "plugin";
    pub static PLUGINS: &str = "plugins";
    pub static STARTUP: &str = "startup";
    pub static PRE_CONFIG: &str = "pre_config";
    pub static CONFIG: &str = "config";
    pub static DEPENDS: &str = "depends";
    pub static DEPEND_BUNDLES: &str = "depend_bundles";
    pub static MODULES: &str = "modules";
    pub static EVENTS: &str = "events";
    pub static FILETYPES: &str = "filetypes";
    pub static COMMANDS: &str = "commands";
    pub static RTP: &str = "rtp";
    pub static AFTER: &str = "after";
    pub static FTPLUGIN: &str = "ftplugin";
}

pub mod file {
    pub static STARTUP_KEYS: &str = "startup_keys";
    pub static MODULE_KEYS: &str = "module_keys";
    pub static EVENT_KEYS: &str = "event_keys";
    pub static FILETYPE_KEYS: &str = "filetype_keys";
    pub static COMMAND_KEYS: &str = "command_keys";
    pub static LAZYS: &str = "lazys";
    pub static DENOPS: &str = "denops";
    pub static PAYLOAD: &str = "payload";
}
