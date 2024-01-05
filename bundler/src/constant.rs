pub mod dir {
    pub static PLUGIN: &str = "plugin";
    pub static PLUGINS: &str = "plugins";
    pub static STARTUP: &str = "startup";
    pub static PRE_CONFIG: &str = "pre_config";
    pub static POST_CONFIG: &str = "post_config";
    pub static DEPEND_PLUGINS: &str = "depend_plugins";
    pub static DEPEND_GROUPS: &str = "depend_groups";
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
    pub static TIMER_CLIENTS: &str = "timer_clients";
    pub static DENOPS_CLIENTS: &str = "denops_clients";
}
