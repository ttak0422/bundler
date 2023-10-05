use crate::constants::Language;
use crate::payload::{
    expand_all_opt_plugins, BundleConfig as PayloadBundleConfig, ConfigLang, Meta,
    OptVimPlugin as PayloadOptVimPlugin, Payload, PluginConfig as PayloadPluginConfig,
    StartVimPlugin as PayloadStartVimPlugin,
};
use serde_json::Value;
use std::collections::HashMap;

fn mapping_lang(lang: &ConfigLang) -> Language {
    match lang {
        ConfigLang::Vim => Language::Vim,
        ConfigLang::Lua => Language::Lua,
        ConfigLang::Fennel => Language::Fennel,
    }
}

/// key is package (e.g. `/nix/store/...`), value is id which corresponds to the name of vim plugin.
fn mk_id_map<'a>(meta: &'a Meta) -> HashMap<&'a str, &'a str> {
    meta.id_map
        .iter()
        .map(|p| (p.package.as_str(), p.id.as_str()))
        .collect()
}

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
}

#[derive(Debug)]
pub struct Pack<'a> {
    pub start_plugins: Vec<StartPlugin<'a>>,
    pub opt_plugins: Vec<OptPlugin<'a>>,
    pub bundles: Vec<Bundle<'a>>,
    pub load_opt: LoadingOptions<'a>,
}

fn unpack_start_plugin<'a>(
    id_map: &HashMap<&'a str, &'a str>,
    plugin: &'a PayloadStartVimPlugin,
) -> StartPlugin<'a> {
    match plugin {
        PayloadStartVimPlugin::Package(package) => {
            let id = id_map.get(package.as_str()).unwrap();
            StartPlugin {
                id,
                ..Default::default()
            }
        }
        PayloadStartVimPlugin::StartPlugin(config) => {
            let id = id_map.get(config.plugin.as_str()).unwrap();
            let startup = match &config.startup {
                PayloadPluginConfig::Line(code) => PluginConfig {
                    lang: Language::Lua,
                    code: &code,
                    args: &Value::Null,
                },
                PayloadPluginConfig::Detail(detail) => PluginConfig {
                    lang: mapping_lang(&detail.lang),
                    code: &detail.code,
                    args: &detail.args,
                },
            };
            StartPlugin { id, startup }
        }
    }
}

fn unpack_start_plugins<'a>(
    id_map: &HashMap<&'a str, &'a str>,
    plugins: &'a Vec<PayloadStartVimPlugin>,
) -> Vec<StartPlugin<'a>> {
    plugins
        .iter()
        .map(|p| unpack_start_plugin(id_map, p))
        .collect::<Vec<_>>()
}

fn unpack_opt_plugin<'a>(
    id_map: &HashMap<&'a str, &'a str>,
    plugin: &'a PayloadOptVimPlugin,
) -> Vec<OptPlugin<'a>> {
    match plugin {
        PayloadOptVimPlugin::Package(package) => {
            let id = id_map.get(package.as_str()).unwrap();
            vec![OptPlugin {
                id,
                ..Default::default()
            }]
        }
        PayloadOptVimPlugin::OptPlugin(config) => {
            let id = id_map.get(config.plugin.as_str()).unwrap();
            let depends = unpack_opt_plugins(id_map, &config.depends.iter().collect());
            let startup = match &config.startup {
                PayloadPluginConfig::Line(code) => PluginConfig {
                    lang: Language::Lua,
                    code: &code,
                    args: &Value::Null,
                },
                PayloadPluginConfig::Detail(detail) => PluginConfig {
                    lang: mapping_lang(&detail.lang),
                    code: &detail.code,
                    args: &detail.args,
                },
            };
            let pre_config = match &config.pre_config {
                PayloadPluginConfig::Line(code) => PluginConfig {
                    lang: Language::Lua,
                    code: &code,
                    args: &Value::Null,
                },
                PayloadPluginConfig::Detail(detail) => PluginConfig {
                    lang: mapping_lang(&detail.lang),
                    code: &detail.code,
                    args: &detail.args,
                },
            };
            let config = match &config.config {
                PayloadPluginConfig::Line(code) => PluginConfig {
                    lang: Language::Lua,
                    code: &code,
                    args: &Value::Null,
                },
                PayloadPluginConfig::Detail(detail) => PluginConfig {
                    lang: mapping_lang(&detail.lang),
                    code: &detail.code,
                    args: &detail.args,
                },
            };
            vec![
                vec![OptPlugin {
                    id,
                    startup,
                    pre_config,
                    config,
                }],
                depends,
            ]
            .concat()
        }
    }
}

fn unpack_opt_plugins<'a>(
    id_map: &HashMap<&'a str, &'a str>,
    plugins: &Vec<&'a PayloadOptVimPlugin>,
) -> Vec<OptPlugin<'a>> {
    plugins
        .iter()
        .flat_map(|p| unpack_opt_plugin(id_map, p))
        .collect::<Vec<_>>()
}

fn unpack_opt_bundle<'a>(
    id_map: &HashMap<&'a str, &'a str>,
    bundle: &'a PayloadBundleConfig,
) -> Bundle<'a> {
    // let id = id_map.get(bundle.name.as_str()).unwrap();
    let plugins = bundle
        .plugins
        .iter()
        .map(|p| match p {
            PayloadOptVimPlugin::Package(package) => *id_map.get(package.as_str()).unwrap(),
            PayloadOptVimPlugin::OptPlugin(config) => *id_map.get(config.plugin.as_str()).unwrap(),
        })
        .collect::<Vec<_>>();
    let startup = match &bundle.startup {
        PayloadPluginConfig::Line(code) => PluginConfig {
            lang: Language::Lua,
            code: &code,
            args: &Value::Null,
        },
        PayloadPluginConfig::Detail(detail) => PluginConfig {
            lang: mapping_lang(&detail.lang),
            code: &detail.code,
            args: &detail.args,
        },
    };
    let pre_config = match &bundle.pre_config {
        PayloadPluginConfig::Line(code) => PluginConfig {
            lang: Language::Lua,
            code: &code,
            args: &Value::Null,
        },
        PayloadPluginConfig::Detail(detail) => PluginConfig {
            lang: mapping_lang(&detail.lang),
            code: &detail.code,
            args: &detail.args,
        },
    };
    let config = match &bundle.config {
        PayloadPluginConfig::Line(code) => PluginConfig {
            lang: Language::Lua,
            code: &code,
            args: &Value::Null,
        },
        PayloadPluginConfig::Detail(detail) => PluginConfig {
            lang: mapping_lang(&detail.lang),
            code: &detail.code,
            args: &detail.args,
        },
    };
    Bundle {
        id: bundle.name.as_str(),
        plugins,
        startup,
        pre_config,
        config,
    }
}

fn unpack_opt_bundles<'a>(
    id_map: &HashMap<&'a str, &'a str>,
    bundles: &'a Vec<PayloadBundleConfig>,
) -> Vec<Bundle<'a>> {
    bundles
        .iter()
        .map(|b| unpack_opt_bundle(id_map, b))
        .collect::<Vec<_>>()
}

fn unpack_opt_plugin_load_options<'a>(
    load_opt: LoadingOptions<'a>,
    id_map: &HashMap<&'a str, &'a str>,
    opt_plugins: &Vec<&'a PayloadOptVimPlugin>,
) -> LoadingOptions<'a> {
    opt_plugins
        .iter()
        .fold(load_opt, |mut load_opt_acc, plugin| match plugin {
            PayloadOptVimPlugin::Package(p) => {
                let id = id_map.get(p.as_str()).unwrap();
                load_opt_acc.depends.entry(id).or_insert(Vec::new());
                load_opt_acc.depend_bundles.entry(id).or_insert(Vec::new());
                LoadingOptions {
                    depends: load_opt_acc.depends,
                    depend_bundles: load_opt_acc.depend_bundles,
                    ..load_opt_acc
                }
            }
            PayloadOptVimPlugin::OptPlugin(cfg) => {
                let id = id_map.get(cfg.plugin.as_str()).unwrap();
                let depends: HashMap<&str, Vec<&str>> = if cfg.depends.is_empty() {
                    load_opt_acc.depends.entry(id).or_insert(Vec::new());
                    load_opt_acc.depends
                } else {
                    cfg.depends.iter().fold(load_opt_acc.depends, |mut acc, p| {
                        acc.entry(id).or_insert_with(Vec::new).push(match p {
                            PayloadOptVimPlugin::Package(depend_package) => {
                                id_map.get(depend_package.as_str()).unwrap()
                            }
                            PayloadOptVimPlugin::OptPlugin(depend_plugin_cfg) => {
                                id_map.get(depend_plugin_cfg.plugin.as_str()).unwrap()
                            }
                        });
                        acc
                    })
                };
                let depend_bundles: HashMap<&str, Vec<&str>> = if cfg.depend_bundles.is_empty() {
                    load_opt_acc.depend_bundles.entry(id).or_insert(Vec::new());
                    load_opt_acc.depend_bundles
                } else {
                    cfg.depend_bundles.iter().fold(
                        load_opt_acc.depend_bundles,
                        |mut acc, bundle_id| {
                            acc.entry(id).or_insert_with(Vec::new).push(bundle_id);
                            acc
                        },
                    )
                };
                let modules = cfg
                    .modules
                    .iter()
                    .fold(load_opt_acc.modules, |mut acc, module| {
                        acc.entry(module).or_insert_with(Vec::new).push(id);
                        acc
                    });
                let events = cfg
                    .events
                    .iter()
                    .fold(load_opt_acc.events, |mut acc, event| {
                        acc.entry(event).or_insert_with(Vec::new).push(id);
                        acc
                    });
                let filetypes =
                    cfg.filetypes
                        .iter()
                        .fold(load_opt_acc.filetypes, |mut acc, filetype| {
                            acc.entry(filetype).or_insert_with(Vec::new).push(id);
                            acc
                        });
                let commands =
                    cfg.commands
                        .iter()
                        .fold(load_opt_acc.commands, |mut acc, command| {
                            acc.entry(command).or_insert_with(Vec::new).push(id);
                            acc
                        });
                if cfg.lazy {
                    load_opt_acc.lazys.push(id);
                }
                LoadingOptions {
                    depends,
                    depend_bundles,
                    modules,
                    events,
                    filetypes,
                    commands,
                    ..load_opt_acc
                }
            }
        })
}

fn unpack_bundle_load_options<'a>(
    load_opt: LoadingOptions<'a>,
    id_map: &HashMap<&'a str, &'a str>,
    bundles: &'a Vec<PayloadBundleConfig>,
) -> LoadingOptions<'a> {
    bundles.iter().fold(load_opt, |mut load_opt_acc, bundle| {
        let id = bundle.name.as_str();
        let depends: HashMap<&str, Vec<&str>> = if bundle.depends.is_empty() {
            load_opt_acc.depends.entry(id).or_insert(Vec::new());
            load_opt_acc.depends
        } else {
            bundle
                .depends
                .iter()
                .fold(load_opt_acc.depends, |mut acc, p| {
                    acc.entry(bundle.name.as_str())
                        .or_insert_with(Vec::new)
                        .push(match p {
                            PayloadOptVimPlugin::Package(depend_package) => {
                                load_opt_acc
                                    .depend_bundles
                                    .entry(bundle.name.as_str())
                                    .or_insert_with(Vec::new)
                                    .push(depend_package.as_str());
                                id_map.get(depend_package.as_str()).unwrap()
                            }
                            PayloadOptVimPlugin::OptPlugin(depend_plugin_cfg) => {
                                load_opt_acc
                                    .depend_bundles
                                    .entry(bundle.name.as_str())
                                    .or_insert_with(Vec::new)
                                    .push(depend_plugin_cfg.plugin.as_str());
                                id_map.get(depend_plugin_cfg.plugin.as_str()).unwrap()
                            }
                        });
                    acc
                })
        };
        let depend_bundles: HashMap<&str, Vec<&str>> = if bundle.depend_bundles.is_empty() {
            load_opt_acc
                .depend_bundles
                .entry(bundle.name.as_str())
                .or_insert(Vec::new());
            load_opt_acc.depend_bundles
        } else {
            bundle
                .depend_bundles
                .iter()
                .fold(load_opt_acc.depend_bundles, |mut acc, bundle_id| {
                    acc.entry(bundle.name.as_str())
                        .or_insert_with(Vec::new)
                        .push(bundle_id);
                    acc
                })
        };
        let modules = bundle
            .modules
            .iter()
            .fold(load_opt_acc.modules, |mut acc, module| {
                acc.entry(module).or_insert_with(Vec::new).push(id);
                acc
            });
        let events = bundle
            .events
            .iter()
            .fold(load_opt_acc.events, |mut acc, event| {
                acc.entry(event).or_insert_with(Vec::new).push(id);
                acc
            });
        let filetypes =
            bundle
                .filetypes
                .iter()
                .fold(load_opt_acc.filetypes, |mut acc, filetype| {
                    acc.entry(filetype).or_insert_with(Vec::new).push(id);
                    acc
                });
        let commands = bundle
            .commands
            .iter()
            .fold(load_opt_acc.commands, |mut acc, command| {
                acc.entry(command).or_insert_with(Vec::new).push(id);
                acc
            });
        LoadingOptions {
            depends,
            depend_bundles,
            modules,
            events,
            filetypes,
            commands,
            ..load_opt_acc
        }
    })
}

pub fn unpack<'a>(payload: &'a Payload) -> Pack<'a> {
    let id_map = mk_id_map(&payload.meta);
    let payload_opt_plugins =
        expand_all_opt_plugins(&payload.cfg.opt_plugins, &payload.cfg.bundles);

    let start_plugins = unpack_start_plugins(&id_map, &payload.cfg.start_plugins);
    let opt_plugins = unpack_opt_plugins(&id_map, &payload_opt_plugins);
    let bundles = unpack_opt_bundles(&id_map, &payload.cfg.bundles);
    let load_opt =
        unpack_opt_plugin_load_options(LoadingOptions::default(), &id_map, &payload_opt_plugins);
    let load_opt = unpack_bundle_load_options(load_opt, &id_map, &payload.cfg.bundles);

    Pack {
        start_plugins,
        opt_plugins,
        bundles,
        load_opt,
    }
}
