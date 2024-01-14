use crate::content::common::{Language, Target};
use crate::content::from_target::FromTarget;
use crate::content::id_table::IdTable;
use crate::payload;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EagerPlugin {
    pub nix_package: String,
    pub startup_config: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LazyPlugin {
    pub nix_package: String,
    pub startup_config: String,
    pub pre_config: String,
    pub post_config: String,
    pub depend_plugin_packages: Vec<String>,
    pub depend_groups: Vec<String>,
    pub on_modules: Vec<String>,
    pub on_events: Vec<String>,
    pub on_filetypes: Vec<String>,
    pub on_commands: Vec<String>,
    pub is_timer_client: bool,
    pub is_denops_client: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LazyGroup {
    // TODO: automate
    pub name: String,
    pub plugins: Vec<String>,
    pub startup_config: String,
    pub pre_config: String,
    pub post_config: String,
    pub depend_plugin_packages: Vec<String>,
    pub depend_groups: Vec<String>,
    pub on_modules: Vec<String>,
    pub on_events: Vec<String>,
    pub on_filetypes: Vec<String>,
    pub on_commands: Vec<String>,
    pub is_timer_client: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Package {
    EagerPlugin(EagerPlugin),
    LazyPlugin(LazyPlugin),
    LazyGroup(LazyGroup),
}

pub struct AfterOption {
    pub ftplugin: HashMap<String, String>,
}

pub struct Content {
    pub packages: Vec<Package>,
    pub id_table: IdTable,
    pub after_option: AfterOption,
}

fn mk_args_code(args: serde_json::Value, language: &Language) -> String {
    if match args {
        serde_json::Value::Object(_) => false,
        // ignore Null, Bool, Number, String, Array
        _ => true,
    } {
        String::default()
    } else {
        let args = serde_json::to_string(&args).unwrap();
        if args == "{}" {
            String::default()
        } else {
            match language {
                Language::Vim => {
                    format!("let s:args = json_decode('{}')", args)
                }
                Language::Lua => {
                    format!("local args = vim.json.decode([[{}]])", args)
                }
            }
        }
    }
}

fn mk_simple_code(code: String, target: &Target) -> String {
    let language = Language::default();
    if code == "" {
        String::default()
    } else {
        match (target, language) {
            (Target::Vim, Language::Vim) => code,
            (Target::Neovim, Language::Vim) => format!("vim.cmd([[{}]])", code),
            (Target::Neovim, Language::Lua) => code,
            _ => panic!("invalid target and language combination"),
        }
    }
}

fn mk_detail_code(cfg: payload::DetailConfig, target: &Target) -> String {
    let language = Language::from(cfg.language);
    let args = mk_args_code(cfg.args, &language);
    match (target, language) {
        (Target::Vim, Language::Vim) => format!("{}\n{}", args, cfg.code),
        (Target::Neovim, Language::Vim) => format!("vim.cmd([[\n{}\n{}]])", args, cfg.code),
        (Target::Neovim, Language::Lua) => format!("{}\n{}", args, cfg.code),
        _ => panic!("invalid target and language combination"),
    }
}

impl FromTarget<payload::EagerVimPluginPackage> for EagerPlugin {
    fn from_target(value: payload::EagerVimPluginPackage, target: &Target) -> Self {
        match value {
            payload::EagerVimPluginPackage::SimplePackage(pkg) => EagerPlugin {
                nix_package: pkg,
                ..Default::default()
            },
            payload::EagerVimPluginPackage::ConfiguredPackage(cfg) => {
                let startup_config = match cfg.startup_config {
                    payload::Config::Simple(code) => mk_simple_code(code, target),
                    payload::Config::Detail(cfg) => mk_detail_code(cfg, target),
                };
                EagerPlugin {
                    nix_package: cfg.plugin,
                    startup_config,
                }
            }
        }
    }
}

impl FromTarget<payload::LazyVimPluginPackage> for Vec<Package> {
    fn from_target(value: payload::LazyVimPluginPackage, target: &Target) -> Self {
        match value {
            payload::LazyVimPluginPackage::SimplePackage(pkg) => {
                vec![Package::LazyPlugin(LazyPlugin {
                    nix_package: pkg,
                    ..Default::default()
                })]
            }
            payload::LazyVimPluginPackage::ConfiguredPackage(cfg) => {
                let mut packages = vec![];

                // package
                let startup_config = match cfg.startup_config {
                    payload::Config::Simple(code) => mk_simple_code(code, target),
                    payload::Config::Detail(cfg) => mk_detail_code(cfg, target),
                };
                let pre_config = match cfg.pre_config {
                    payload::Config::Simple(code) => mk_simple_code(code, target),
                    payload::Config::Detail(cfg) => mk_detail_code(cfg, target),
                };
                let post_config = match cfg.post_config {
                    payload::Config::Simple(code) => mk_simple_code(code, target),
                    payload::Config::Detail(cfg) => mk_detail_code(cfg, target),
                };
                let depend_plugins = cfg
                    .depend_plugins
                    .iter()
                    .map(|p| match p {
                        payload::LazyVimPluginPackage::SimplePackage(pkg) => pkg.clone(),
                        payload::LazyVimPluginPackage::ConfiguredPackage(cfg) => cfg.plugin.clone(),
                    })
                    .collect();
                let plugin = LazyPlugin {
                    nix_package: cfg.plugin,
                    startup_config,
                    pre_config,
                    post_config,
                    depend_plugin_packages: depend_plugins,
                    depend_groups: cfg.depend_groups,
                    on_modules: cfg.on_modules,
                    on_events: cfg.on_events,
                    on_filetypes: cfg.on_filetypes,
                    on_commands: cfg.on_commands,
                    is_timer_client: cfg.use_timer,
                    is_denops_client: cfg.use_denops,
                };
                packages.push(Package::LazyPlugin(plugin));

                // depend packages
                let depend_packages = cfg
                    .depend_plugins
                    .into_iter()
                    .map(|p| Vec::from_target(p, target))
                    .flatten()
                    .collect::<Vec<Package>>();
                packages.extend(depend_packages);

                packages
            }
        }
    }
}

impl FromTarget<payload::LazyGroup> for Vec<Package> {
    fn from_target(value: payload::LazyGroup, target: &Target) -> Self {
        let mut packages = vec![];

        // package
        let plugins = value
            .plugins
            .iter()
            .map(|p| match p {
                payload::LazyVimPluginPackage::SimplePackage(pkg) => pkg.clone(),
                payload::LazyVimPluginPackage::ConfiguredPackage(cfg) => cfg.plugin.clone(),
            })
            .collect();
        let startup_config = match value.startup_config {
            payload::Config::Simple(code) => mk_simple_code(code, target),
            payload::Config::Detail(cfg) => mk_detail_code(cfg, target),
        };
        let pre_config = match value.pre_config {
            payload::Config::Simple(code) => mk_simple_code(code, target),
            payload::Config::Detail(cfg) => mk_detail_code(cfg, target),
        };
        let post_config = match value.post_config {
            payload::Config::Simple(code) => mk_simple_code(code, target),
            payload::Config::Detail(cfg) => mk_detail_code(cfg, target),
        };
        let depend_plugins = value
            .depend_plugins
            .iter()
            .map(|p| match p {
                payload::LazyVimPluginPackage::SimplePackage(pkg) => pkg.clone(),
                payload::LazyVimPluginPackage::ConfiguredPackage(cfg) => cfg.plugin.clone(),
            })
            .collect();
        let group = LazyGroup {
            name: value.name,
            plugins,
            startup_config,
            pre_config,
            post_config,
            depend_plugin_packages: depend_plugins,
            depend_groups: value.depend_groups,
            on_modules: value.on_modules,
            on_events: value.on_events,
            on_filetypes: value.on_filetypes,
            on_commands: value.on_commands,
            is_timer_client: value.use_timer,
        };
        packages.push(Package::LazyGroup(group));

        // plugin packages
        let plugin_packages = value
            .plugins
            .into_iter()
            .map(|p| Vec::from_target(p, target))
            .flatten()
            .collect::<Vec<Package>>();
        packages.extend(plugin_packages);

        // depend packages
        let depend_packages = value
            .depend_plugins
            .into_iter()
            .map(|p| Vec::from_target(p, target))
            .flatten()
            .collect::<Vec<Package>>();
        packages.extend(depend_packages);

        packages
    }
}

impl From<payload::AfterOption> for AfterOption {
    fn from(value: payload::AfterOption) -> Self {
        AfterOption {
            ftplugin: value.ftplugin,
        }
    }
}
