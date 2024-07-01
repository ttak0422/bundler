pub mod dir {
    pub static PACKAGES: &str = "packages";
    pub static STARTUP_CONFIGS: &str = "startup_configs";
    pub static PRE_CONFIGS: &str = "pre_configs";
    pub static POST_CONFIGS: &str = "post_configs";
    pub static MODULES: &str = "modules";
    pub static EVENTS: &str = "events";
    pub static USER_EVENTS: &str = "user_events";
    pub static COMMANDS: &str = "commands";
    pub static DEPENDS: &str = "depends";
    pub static AFTER: &str = "after";
    pub static FTPLUGIN: &str = "ftplugin";
    pub static RTP: &str = "rtp";
}

pub mod file {
    pub static MODULE_KEYS: &str = "module_keys";
    pub static EVENT_KEYS: &str = "event_keys";
    pub static USER_EVENT_KEYS: &str = "user_event_keys";
    pub static COMMAND_KEYS: &str = "command_keys";
    pub static STARTUP_CONFIG_KEYS: &str = "startup_config_keys";
    pub static DENOPS_KEYS: &str = "denops_keys";
}
