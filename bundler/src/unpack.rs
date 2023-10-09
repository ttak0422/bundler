// TODO: rename some type, fn.
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

fn unpack_bundle<'a>(
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

fn unpack_bundles<'a>(
    id_map: &HashMap<&'a str, &'a str>,
    bundles: &'a Vec<PayloadBundleConfig>,
) -> Vec<Bundle<'a>> {
    bundles
        .iter()
        .map(|b| unpack_bundle(id_map, b))
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
                load_opt_acc.depends.entry(id).or_default();
                load_opt_acc.depend_bundles.entry(id).or_default();
                load_opt_acc
            }
            PayloadOptVimPlugin::OptPlugin(cfg) => {
                let id = id_map.get(cfg.plugin.as_str()).unwrap();
                load_opt_acc.depends.entry(id).or_default();
                load_opt_acc.depend_bundles.entry(id).or_default();
                let depends: HashMap<&str, Vec<&str>> = if cfg.depends.is_empty() {
                    load_opt_acc.depends
                } else {
                    cfg.depends.iter().fold(load_opt_acc.depends, |mut acc, p| {
                        let dependent_id = id_map
                            .get(match p {
                                PayloadOptVimPlugin::Package(depend_package) => {
                                    depend_package.as_str()
                                }
                                PayloadOptVimPlugin::OptPlugin(depend_plugin_cfg) => {
                                    depend_plugin_cfg.plugin.as_str()
                                }
                            })
                            .unwrap();
                        acc.entry(id).or_default().push(dependent_id);
                        acc
                    })
                };
                let depend_bundles: HashMap<&str, Vec<&str>> = if cfg.depend_bundles.is_empty() {
                    load_opt_acc.depend_bundles
                } else {
                    cfg.depend_bundles.iter().fold(
                        load_opt_acc.depend_bundles,
                        |mut acc, bundle_id| {
                            acc.entry(id).or_default().push(bundle_id);
                            acc
                        },
                    )
                };
                let modules = cfg
                    .modules
                    .iter()
                    .fold(load_opt_acc.modules, |mut acc, module| {
                        acc.entry(module).or_default().push(id);
                        acc
                    });
                let events = cfg
                    .events
                    .iter()
                    .fold(load_opt_acc.events, |mut acc, event| {
                        acc.entry(event).or_default().push(id);
                        acc
                    });
                let filetypes =
                    cfg.filetypes
                        .iter()
                        .fold(load_opt_acc.filetypes, |mut acc, filetype| {
                            acc.entry(filetype).or_default().push(id);
                            acc
                        });
                let commands =
                    cfg.commands
                        .iter()
                        .fold(load_opt_acc.commands, |mut acc, command| {
                            acc.entry(command).or_default().push(id);
                            acc
                        });
                if cfg.lazy {
                    load_opt_acc.lazys.push(id);
                }
                unpack_opt_plugin_load_options(
                    LoadingOptions {
                        depends,
                        depend_bundles,
                        modules,
                        events,
                        filetypes,
                        commands,
                        ..load_opt_acc
                    },
                    id_map,
                    &cfg.depends.iter().collect(),
                )
            }
        })
}

fn unpack_bundle_load_options<'a>(
    load_opt: LoadingOptions<'a>,
    id_map: &HashMap<&'a str, &'a str>,
    bundles: &'a Vec<PayloadBundleConfig>,
) -> LoadingOptions<'a> {
    bundles.iter().fold(load_opt, |load_opt_acc, bundle| {
        let id = bundle.name.as_str();
        let mut load_opt_acc =
            unpack_opt_plugin_load_options(load_opt_acc, id_map, &bundle.plugins.iter().collect());
        load_opt_acc.depends.entry(id).or_default();
        load_opt_acc.depend_bundles.entry(id).or_default();
        let depends: HashMap<&str, Vec<&str>> = if bundle.depends.is_empty() {
            load_opt_acc.depends
        } else {
            bundle
                .depends
                .iter()
                .fold(load_opt_acc.depends, |mut acc, p| {
                    let dep_id = id_map
                        .get(match p {
                            PayloadOptVimPlugin::Package(depend_package) => depend_package.as_str(),
                            PayloadOptVimPlugin::OptPlugin(depend_plugin_cfg) => {
                                depend_plugin_cfg.plugin.as_str()
                            }
                        })
                        .unwrap();
                    acc.get_mut(id).unwrap().push(&dep_id);
                    acc
                })
        };
        let depend_bundles: HashMap<&str, Vec<&str>> = if bundle.depend_bundles.is_empty() {
            // load_opt_acc.depend_bundles.entry(id).or_default();
            load_opt_acc.depend_bundles
        } else {
            bundle
                .depend_bundles
                .iter()
                .fold(load_opt_acc.depend_bundles, |mut acc, bundle_id| {
                    acc.entry(id).or_default().push(bundle_id);
                    acc
                })
        };
        let modules = bundle
            .modules
            .iter()
            .fold(load_opt_acc.modules, |mut acc, module| {
                acc.entry(module).or_default().push(id);
                acc
            });
        let events = bundle
            .events
            .iter()
            .fold(load_opt_acc.events, |mut acc, event| {
                acc.entry(event).or_default().push(id);
                acc
            });
        let filetypes =
            bundle
                .filetypes
                .iter()
                .fold(load_opt_acc.filetypes, |mut acc, filetype| {
                    acc.entry(filetype).or_default().push(id);
                    acc
                });
        let commands = bundle
            .commands
            .iter()
            .fold(load_opt_acc.commands, |mut acc, command| {
                acc.entry(command).or_default().push(id);
                acc
            });
        if bundle.lazy {
            load_opt_acc.lazys.push(id);
        }
        unpack_opt_plugin_load_options(
            LoadingOptions {
                depends,
                depend_bundles,
                modules,
                events,
                filetypes,
                commands,
                ..load_opt_acc
            },
            id_map,
            &bundle.depends.iter().collect(),
        )
    })
}

pub fn unpack<'a>(payload: &'a Payload) -> Pack<'a> {
    let id_map = mk_id_map(&payload.meta);
    let payload_opt_plugins =
        expand_all_opt_plugins(&payload.cfg.opt_plugins, &payload.cfg.bundles);

    let start_plugins = unpack_start_plugins(&id_map, &payload.cfg.start_plugins);
    let opt_plugins = unpack_opt_plugins(&id_map, &payload_opt_plugins);
    let bundles = unpack_bundles(&id_map, &payload.cfg.bundles);
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

#[cfg(test)]
mod tests {
    use crate::payload::PluginPackage;

    use super::*;

    mod mother {
        use crate::payload::{Meta, PluginPackage};

        pub fn package_path(name: &str) -> String {
            String::from("/nix/store/") + name
        }

        pub fn plugin_package(name: &str) -> PluginPackage {
            PluginPackage {
                id: String::from(name),
                package: package_path(name),
            }
        }

        pub fn meta(names: Vec<&str>) -> Meta {
            Meta {
                extra_packages: vec![String::from("/nix/store/...")],
                id_map: names.iter().map(|name| plugin_package(name)).collect(),
            }
        }
    }

    #[test]
    fn make_id_map() {
        let foo_package = PluginPackage {
            id: String::from("foo"),
            package: mother::package_path("foo"),
        };
        let bar_package = PluginPackage {
            id: String::from("bar"),
            package: mother::package_path("bar"),
        };
        let meta = Meta {
            extra_packages: vec![String::from("/nix/store/...")],
            id_map: vec![foo_package.clone(), bar_package.clone()],
        };
        let exp = HashMap::<&str, &str>::from([
            (foo_package.package.as_str(), "foo"),
            (&bar_package.package.as_str(), "bar"),
        ]);

        let act = mk_id_map(&meta);

        assert_eq!(exp, act);
    }

    #[test]
    fn unpack_start_package() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = mk_id_map(&meta);
        let plugin = PayloadStartVimPlugin::Package(mother::package_path(name));
        let plugins = vec![plugin.clone()];
        let exp = StartPlugin {
            id: name,
            ..Default::default()
        };
        let exp_vec = vec![exp.clone()];

        let act = super::unpack_start_plugin(&id_map, &plugin);
        let act_vec = super::unpack_start_plugins(&id_map, &plugins);

        assert_eq!(exp, act);
        assert_eq!(exp_vec, act_vec);
    }

    #[test]
    fn unpack_start_plugin() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = mk_id_map(&meta);
        let code = "foo-code";
        let args = serde_json::json!({"foo-arg": 1});
        let plugin = PayloadStartVimPlugin::StartPlugin(crate::payload::StartPluginConfig {
            plugin: mother::package_path(name),
            startup: crate::payload::PluginConfig::Detail(crate::payload::PluginConfigDetail {
                lang: crate::payload::ConfigLang::Lua,
                code: String::from(code),
                args: args.clone(),
            }),
            extra_packages: vec![],
        });
        let plugins = vec![plugin.clone()];
        let exp = StartPlugin {
            id: name,
            startup: PluginConfig {
                lang: Language::Lua,
                code: &code,
                args: &args,
            },
        };
        let exp_vec = vec![exp.clone()];

        let act = super::unpack_start_plugin(&id_map, &plugin);
        let act_vec = super::unpack_start_plugins(&id_map, &plugins);

        assert_eq!(exp, act);
        assert_eq!(exp_vec, act_vec);
    }

    #[test]
    fn unpack_opt_package() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = mk_id_map(&meta);
        let plugin = PayloadOptVimPlugin::Package(mother::package_path(name));
        let plugins = vec![plugin.clone()];
        let exp = vec![OptPlugin {
            id: name,
            ..Default::default()
        }];
        let exp_vec = exp.clone();

        let act = super::unpack_opt_plugin(&id_map, &plugin);
        let act_vec = super::unpack_opt_plugins(&id_map, &plugins.iter().collect());

        assert_eq!(exp, act);
        assert_eq!(exp_vec, act_vec);
    }

    #[test]
    fn unpack_opt_plugin() {
        let name = "foo-vim";
        let name2 = "bar-vin";
        let meta = mother::meta(vec![name, name2]);
        let id_map = mk_id_map(&meta);
        let startup_code = "foo-startup";
        let config_code = "foo-config";
        let pre_config_code = "foo-pre-config";
        let startup_arg = serde_json::json!({"foo-startup-arg": 1});
        let config_arg = serde_json::json!({"foo-config-arg": 2});
        let pre_config_arg = serde_json::json!({"foo-pre-config-arg": 3});
        let plugin = PayloadOptVimPlugin::OptPlugin(crate::payload::OptPluginConfig {
            plugin: mother::package_path(name),
            startup: crate::payload::PluginConfig::Detail(crate::payload::PluginConfigDetail {
                lang: crate::payload::ConfigLang::Vim,
                code: String::from(startup_code),
                args: startup_arg.clone(),
            }),
            pre_config: crate::payload::PluginConfig::Detail(crate::payload::PluginConfigDetail {
                lang: crate::payload::ConfigLang::Lua,
                code: String::from(pre_config_code),
                args: pre_config_arg.clone(),
            }),
            config: crate::payload::PluginConfig::Detail(crate::payload::PluginConfigDetail {
                lang: crate::payload::ConfigLang::Lua,
                code: String::from(config_code),
                args: config_arg.clone(),
            }),
            depends: vec![PayloadOptVimPlugin::Package(mother::package_path(name2))],
            ..Default::default()
        });
        let plugins = vec![plugin.clone()];
        let exp = vec![
            OptPlugin {
                id: name,
                startup: PluginConfig {
                    lang: Language::Vim,
                    code: &startup_code,
                    args: &startup_arg,
                },
                pre_config: PluginConfig {
                    lang: Language::Lua,
                    code: &pre_config_code,
                    args: &pre_config_arg,
                },
                config: PluginConfig {
                    lang: Language::Lua,
                    code: &config_code,
                    args: &config_arg,
                },
            },
            OptPlugin {
                id: name2,
                ..Default::default()
            },
        ];
        let exp_vec = exp.clone();

        let act = super::unpack_opt_plugin(&id_map, &plugin);
        let act_vec = super::unpack_opt_plugins(&id_map, &plugins.iter().collect());

        // TODO: soft assert, ignore order
        assert_eq!(exp, act);
        assert_eq!(exp_vec, act_vec);
    }

    #[test]
    fn unpack_bundle() {
        let bundle_name = "hoge";
        let depend_bundle_name = "huga";
        let name1 = "foo-vim";
        let name2 = "bar-vim";
        let meta = mother::meta(vec![name1, name2]);
        let id_map = mk_id_map(&meta);
        let startup_code = "foo-startup";
        let config_code = "foo-config";
        let pre_config_code = "foo-pre-config";
        let startup_arg = serde_json::json!({"foo-startup-arg": 1});
        let config_arg = serde_json::json!({"foo-config-arg": 2});
        let pre_config_arg = serde_json::json!({"foo-pre-config-arg": 3});
        let bundle = PayloadBundleConfig {
            name: String::from(bundle_name),
            plugins: vec![PayloadOptVimPlugin::Package(mother::package_path(name1))],
            startup: crate::payload::PluginConfig::Detail(crate::payload::PluginConfigDetail {
                lang: crate::payload::ConfigLang::Vim,
                code: String::from(startup_code),
                args: startup_arg.clone(),
            }),
            extra_packages: vec![],
            pre_config: crate::payload::PluginConfig::Detail(crate::payload::PluginConfigDetail {
                lang: crate::payload::ConfigLang::Lua,
                code: String::from(pre_config_code),
                args: pre_config_arg.clone(),
            }),
            config: crate::payload::PluginConfig::Detail(crate::payload::PluginConfigDetail {
                lang: crate::payload::ConfigLang::Lua,
                code: String::from(config_code),
                args: config_arg.clone(),
            }),
            depends: vec![PayloadOptVimPlugin::Package(mother::package_path(name2))],
            depend_bundles: vec![depend_bundle_name.to_string()],
            ..Default::default()
        };
        let exp = Bundle {
            id: bundle_name,
            plugins: vec![name1],
            startup: PluginConfig {
                lang: Language::Vim,
                code: &startup_code,
                args: &startup_arg,
            },
            pre_config: PluginConfig {
                lang: Language::Lua,
                code: &pre_config_code,
                args: &pre_config_arg,
            },
            config: PluginConfig {
                lang: Language::Lua,
                code: &config_code,
                args: &config_arg,
            },
        };

        let act = super::unpack_bundle(&id_map, &bundle);

        assert_eq!(exp, act);
    }

    #[test]
    fn unpack_opt_package_load_options() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = mk_id_map(&meta);
        let plugin = PayloadOptVimPlugin::Package(mother::package_path(name));
        let plugins = vec![plugin.clone()];
        let exp = LoadingOptions {
            depends: HashMap::from([(name, vec![])]),
            depend_bundles: HashMap::from([(name, vec![])]),
            modules: HashMap::from([]),
            events: HashMap::from([]),
            filetypes: HashMap::from([]),
            commands: HashMap::from([]),
            lazys: vec![],
        };

        let act = super::unpack_opt_plugin_load_options(
            LoadingOptions::default(),
            &id_map,
            &plugins.iter().collect(),
        );

        assert_eq!(exp, act);
    }

    #[test]
    fn unpack_opt_plugin_load_options() {
        let name1 = "foo-vim";
        let name2 = "bar-vim";
        let depend_bundle_name = "hoge";
        let module_name = "module";
        let event_name = "event";
        let filetype_name = "filetype";
        let command_name = "command";
        let meta = mother::meta(vec![name1, name2]);
        let id_map = mk_id_map(&meta);
        let plugin = PayloadOptVimPlugin::OptPlugin(crate::payload::OptPluginConfig {
            plugin: mother::package_path(name1),
            depends: vec![PayloadOptVimPlugin::Package(mother::package_path(name2))],
            depend_bundles: vec![depend_bundle_name.to_string()],
            modules: vec![module_name.to_string()],
            events: vec![event_name.to_string()],
            filetypes: vec![filetype_name.to_string()],
            commands: vec![command_name.to_string()],
            lazy: true,
            ..Default::default()
        });
        let exp = LoadingOptions {
            depends: HashMap::from([(name1, vec![name2]), (name2, vec![])]),
            depend_bundles: HashMap::from([(name1, vec![depend_bundle_name]), (name2, vec![])]),
            modules: HashMap::from([(module_name, vec![name1])]),
            events: HashMap::from([(event_name, vec![name1])]),
            filetypes: HashMap::from([(filetype_name, vec![name1])]),
            commands: HashMap::from([(command_name, vec![name1])]),
            lazys: vec![name1],
        };

        let act = super::unpack_opt_plugin_load_options(
            LoadingOptions::default(),
            &id_map,
            &vec![&plugin],
        );

        assert_eq!(exp, act);
    }

    #[test]
    fn unpack_bundle_load_options() {
        let name1 = "foo-vim";
        let name2 = "bar-vim";
        let bundle_name = "hoge";
        let depend_bundle_name = "huga";
        let module_name = "module";
        let event_name = "event";
        let filetype_name = "filetype";
        let command_name = "command";
        let meta = mother::meta(vec![name1, name2]);
        let id_map = mk_id_map(&meta);
        let bundle = PayloadBundleConfig {
            name: String::from(bundle_name),
            plugins: vec![PayloadOptVimPlugin::Package(mother::package_path(name1))],
            depends: vec![PayloadOptVimPlugin::Package(mother::package_path(name2))],
            depend_bundles: vec![depend_bundle_name.to_string()],
            modules: vec![module_name.to_string()],
            events: vec![event_name.to_string()],
            filetypes: vec![filetype_name.to_string()],
            commands: vec![command_name.to_string()],
            lazy: true,
            ..Default::default()
        };
        let bundles = vec![bundle];
        let exp = LoadingOptions {
            depends: HashMap::from([(bundle_name, vec![name2]), (name1, vec![]), (name2, vec![])]),
            depend_bundles: HashMap::from([
                (bundle_name, vec![depend_bundle_name]),
                (name1, vec![]),
                (name2, vec![]),
            ]),
            modules: HashMap::from([(module_name, vec![bundle_name])]),
            events: HashMap::from([(event_name, vec![bundle_name])]),
            filetypes: HashMap::from([(filetype_name, vec![bundle_name])]),
            commands: HashMap::from([(command_name, vec![bundle_name])]),
            lazys: vec![bundle_name],
        };

        let act = super::unpack_bundle_load_options(LoadingOptions::default(), &id_map, &bundles);

        assert_eq!(exp, act);
    }
}
