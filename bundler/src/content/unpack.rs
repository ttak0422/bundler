use crate::constants::Language;
use crate::content::config;
use crate::payload;
use serde_json::Value;
use std::collections::HashMap;

fn mapping_language(lang: &payload::Language) -> Language {
    match lang {
        payload::Language::Vim => Language::Vim,
        payload::Language::Lua => Language::Lua,
    }
}

/// key is nix package (e.g. `/nix/store/...`), value is plugin_id which corresponds to the name of vim plugin.
fn make_id_map<'a>(meta: &'a payload::Meta) -> HashMap<&'a str, &'a str> {
    meta.id_map
        .iter()
        .map(|p| (p.package.as_str(), p.plugin_id.as_str()))
        .collect()
}

pub trait Unpackable<'a, Out> {
    /// pauload to `Out`.
    fn unpack(&'a self, id_map: &HashMap<&'a str, &'a str>) -> Out;

    /// nix package (e.g. /nix/store/....) to plugin id (e.g. bundler-nvim).
    fn package_to_plugin_id(id_map: &HashMap<&'a str, &'a str>, package: &'a str) -> &'a str {
        // the value is guaranteed to exist.
        id_map.get(package).unwrap()
    }
}

impl<'a> Unpackable<'a, config::EagerPlugin<'a>> for payload::VimStartPlugin {
    fn unpack(&'a self, id_map: &HashMap<&'a str, &'a str>) -> config::EagerPlugin<'a> {
        match self {
            payload::VimStartPlugin::SimplePackage(pkg) => config::EagerPlugin {
                plugin_id: Self::package_to_plugin_id(id_map, pkg.as_str()),
                ..Default::default()
            },
            payload::VimStartPlugin::ConfiguredPackage(cfg) => config::EagerPlugin {
                plugin_id: Self::package_to_plugin_id(id_map, cfg.plugin.as_str()),
                startup_config: match &cfg.startup_config {
                    payload::Config::Simple(code) => config::PluginConfig {
                        language: Language::Lua,
                        code: &code,
                        args: &Value::Null,
                    },
                    payload::Config::Detail(dtl) => config::PluginConfig {
                        language: mapping_language(&dtl.language),
                        code: &dtl.code,
                        args: &dtl.args,
                    },
                },
            },
        }
    }
}

impl<'a> Unpackable<'a, Vec<config::LazyPlugin<'a>>> for payload::VimOptPlugin {
    fn unpack(&'a self, id_map: &HashMap<&'a str, &'a str>) -> Vec<config::LazyPlugin<'a>> {
        match self {
            payload::VimOptPlugin::SimplePackage(pkg) => {
                let id = id_map.get(pkg.as_str()).unwrap();
                vec![config::LazyPlugin {
                    plugin_id: id,
                    ..Default::default()
                }]
            }
            payload::VimOptPlugin::ConfiguredPackage(cfg) => {
                let plugin_id = id_map.get(cfg.plugin.as_str()).unwrap();
                let depends = cfg
                    .depend_plugins
                    .iter()
                    .flat_map(|p| p.unpack(&id_map))
                    .collect::<Vec<_>>();
                let startup_config = match &cfg.startup_config {
                    payload::Config::Simple(code) => config::PluginConfig {
                        language: Language::Lua,
                        code: &code,
                        args: &Value::Null,
                    },
                    payload::Config::Detail(dtl) => config::PluginConfig {
                        language: mapping_language(&dtl.language),
                        code: &dtl.code,
                        args: &dtl.args,
                    },
                };
                let pre_config = match &cfg.pre_config {
                    payload::Config::Simple(code) => config::PluginConfig {
                        language: Language::Lua,
                        code: &code,
                        args: &Value::Null,
                    },
                    payload::Config::Detail(dtl) => config::PluginConfig {
                        language: mapping_language(&dtl.language),
                        code: &dtl.code,
                        args: &dtl.args,
                    },
                };
                let config = match &cfg.post_config {
                    payload::Config::Simple(code) => config::PluginConfig {
                        language: Language::Lua,
                        code: &code,
                        args: &Value::Null,
                    },
                    payload::Config::Detail(dtl) => config::PluginConfig {
                        language: mapping_language(&dtl.language),
                        code: &dtl.code,
                        args: &dtl.args,
                    },
                };
                vec![
                    vec![config::LazyPlugin {
                        plugin_id,
                        startup_config,
                        pre_config,
                        config,
                    }],
                    depends,
                ]
                .concat()
            }
        }
    }
}

impl<'a> Unpackable<'a, config::LazyGroup<'a>> for payload::LazyGroup {
    fn unpack(&'a self, id_map: &HashMap<&'a str, &'a str>) -> config::LazyGroup<'a> {
        let plugins = self
            .plugins
            .iter()
            .map(|p| match p {
                payload::VimOptPlugin::SimplePackage(pkg) => *id_map.get(pkg.as_str()).unwrap(),
                payload::VimOptPlugin::ConfiguredPackage(cfg) => {
                    *id_map.get(cfg.plugin.as_str()).unwrap()
                }
            })
            .collect::<Vec<_>>();
        let startup_config = match &self.startup_config {
            payload::Config::Simple(code) => config::PluginConfig {
                language: Language::Lua,
                code: &code,
                args: &Value::Null,
            },
            payload::Config::Detail(dtl) => config::PluginConfig {
                language: mapping_language(&dtl.language),
                code: &dtl.code,
                args: &dtl.args,
            },
        };
        let pre_config = match &self.pre_config {
            payload::Config::Simple(code) => config::PluginConfig {
                language: Language::Lua,
                code: &code,
                args: &Value::Null,
            },
            payload::Config::Detail(dtl) => config::PluginConfig {
                language: mapping_language(&dtl.language),
                code: &dtl.code,
                args: &dtl.args,
            },
        };
        let post_config = match &self.post_config {
            payload::Config::Simple(code) => config::PluginConfig {
                language: Language::Lua,
                code: &code,
                args: &Value::Null,
            },
            payload::Config::Detail(dtl) => config::PluginConfig {
                language: mapping_language(&dtl.language),
                code: &dtl.code,
                args: &dtl.args,
            },
        };
        config::LazyGroup {
            group_id: self.name.as_str(),
            plugin_ids: plugins,
            startup_config,
            pre_config,
            post_config,
        }
    }
}

fn unpack_opt_plugin_load_options<'a>(
    load_opt: config::LoadOption<'a>,
    id_map: &HashMap<&'a str, &'a str>,
    opt_plugins: &Vec<&'a payload::VimOptPlugin>,
) -> config::LoadOption<'a> {
    opt_plugins
        .iter()
        .fold(load_opt, |mut load_opt_acc, plugin| match plugin {
            payload::VimOptPlugin::SimplePackage(pkg) => {
                let id = id_map.get(pkg.as_str()).unwrap();
                load_opt_acc.depend_plugins.entry(id).or_default();
                load_opt_acc.depend_groups.entry(id).or_default();
                load_opt_acc
            }
            payload::VimOptPlugin::ConfiguredPackage(cfg) => {
                let id = id_map.get(cfg.plugin.as_str()).unwrap();
                load_opt_acc.depend_plugins.entry(id).or_default();
                load_opt_acc.depend_groups.entry(id).or_default();
                let depends: HashMap<&str, Vec<&str>> = if cfg.depend_plugins.is_empty() {
                    load_opt_acc.depend_plugins
                } else {
                    cfg.depend_plugins
                        .iter()
                        .fold(load_opt_acc.depend_plugins, |mut acc, p| {
                            let dependent_id = id_map
                                .get(match p {
                                    payload::VimOptPlugin::SimplePackage(depend_package) => {
                                        depend_package.as_str()
                                    }
                                    payload::VimOptPlugin::ConfiguredPackage(depend_plugin_cfg) => {
                                        depend_plugin_cfg.plugin.as_str()
                                    }
                                })
                                .unwrap();
                            acc.entry(id).or_default().push(dependent_id);
                            acc
                        })
                };
                let depend_bundles: HashMap<&str, Vec<&str>> = if cfg.depend_groups.is_empty() {
                    load_opt_acc.depend_groups
                } else {
                    cfg.depend_groups.iter().fold(
                        load_opt_acc.depend_groups,
                        |mut acc, bundle_id| {
                            acc.entry(id).or_default().push(bundle_id);
                            acc
                        },
                    )
                };
                let on_modules =
                    cfg.on_modules
                        .iter()
                        .fold(load_opt_acc.on_modules, |mut acc, module| {
                            acc.entry(module).or_default().push(id);
                            acc
                        });
                let on_events =
                    cfg.on_events
                        .iter()
                        .fold(load_opt_acc.on_events, |mut acc, event| {
                            acc.entry(event).or_default().push(id);
                            acc
                        });
                let on_filetypes =
                    cfg.on_filetypes
                        .iter()
                        .fold(load_opt_acc.on_filetypes, |mut acc, filetype| {
                            acc.entry(filetype).or_default().push(id);
                            acc
                        });
                let on_commands =
                    cfg.on_commands
                        .iter()
                        .fold(load_opt_acc.on_commands, |mut acc, command| {
                            acc.entry(command).or_default().push(id);
                            acc
                        });
                if cfg.use_timer {
                    load_opt_acc.timer_clients.push(id);
                }
                if cfg.use_denops {
                    load_opt_acc.denops_clients.push(id);
                }
                unpack_opt_plugin_load_options(
                    config::LoadOption {
                        depend_plugins: depends,
                        depend_groups: depend_bundles,
                        on_modules,
                        on_events,
                        on_filetypes,
                        on_commands,
                        ..load_opt_acc
                    },
                    id_map,
                    &cfg.depend_plugins.iter().collect(),
                )
            }
        })
}

fn unpack_bundle_load_options<'a>(
    load_opt: config::LoadOption<'a>,
    id_map: &HashMap<&'a str, &'a str>,
    bundles: &'a Vec<payload::LazyGroup>,
) -> config::LoadOption<'a> {
    bundles.iter().fold(load_opt, |load_opt_acc, bundle| {
        let id = bundle.name.as_str();
        let mut load_opt_acc =
            unpack_opt_plugin_load_options(load_opt_acc, id_map, &bundle.plugins.iter().collect());
        load_opt_acc.depend_plugins.entry(id).or_default();
        load_opt_acc.depend_groups.entry(id).or_default();
        let depends: HashMap<&str, Vec<&str>> = if bundle.depend_plugins.is_empty() {
            load_opt_acc.depend_plugins
        } else {
            bundle
                .depend_plugins
                .iter()
                .fold(load_opt_acc.depend_plugins, |mut acc, p| {
                    let dep_id = id_map
                        .get(match p {
                            payload::VimOptPlugin::SimplePackage(depend_package) => {
                                depend_package.as_str()
                            }
                            payload::VimOptPlugin::ConfiguredPackage(depend_plugin_cfg) => {
                                depend_plugin_cfg.plugin.as_str()
                            }
                        })
                        .unwrap();
                    acc.get_mut(id).unwrap().push(&dep_id);
                    acc
                })
        };
        let depend_bundles: HashMap<&str, Vec<&str>> = if bundle.depend_groups.is_empty() {
            // load_opt_acc.depend_bundles.entry(id).or_default();
            load_opt_acc.depend_groups
        } else {
            bundle
                .depend_groups
                .iter()
                .fold(load_opt_acc.depend_groups, |mut acc, bundle_id| {
                    acc.entry(id).or_default().push(bundle_id);
                    acc
                })
        };
        let on_modules =
            bundle
                .on_modules
                .iter()
                .fold(load_opt_acc.on_modules, |mut acc, module| {
                    acc.entry(module).or_default().push(id);
                    acc
                });
        let on_events = bundle
            .on_events
            .iter()
            .fold(load_opt_acc.on_events, |mut acc, event| {
                acc.entry(event).or_default().push(id);
                acc
            });
        let on_filetypes =
            bundle
                .on_filetypes
                .iter()
                .fold(load_opt_acc.on_filetypes, |mut acc, filetype| {
                    acc.entry(filetype).or_default().push(id);
                    acc
                });
        let on_commands =
            bundle
                .on_commands
                .iter()
                .fold(load_opt_acc.on_commands, |mut acc, command| {
                    acc.entry(command).or_default().push(id);
                    acc
                });
        if bundle.use_timer {
            load_opt_acc.timer_clients.push(id);
        }
        unpack_opt_plugin_load_options(
            config::LoadOption {
                depend_plugins: depends,
                depend_groups: depend_bundles,
                on_modules,
                on_events,
                on_filetypes,
                on_commands,
                ..load_opt_acc
            },
            id_map,
            &bundle.depend_plugins.iter().collect(),
        )
    })
}

pub fn unpack<'a>(payload: &'a payload::Payload) -> config::Specs<'a> {
    let id_map = make_id_map(&payload.meta);
    let payload_opt_plugins = [
        payload::expand(&payload.config.lazy_plugins),
        payload::expand(&payload.config.lazy_groups),
    ]
    .concat();

    let start_plugins = payload
        .config
        .eager_plugins
        .iter()
        .map(|p| p.unpack(&id_map))
        .collect::<Vec<_>>();
    let opt_plugins = payload_opt_plugins
        .iter()
        .flat_map(|p| p.unpack(&id_map))
        .collect::<Vec<_>>();
    let bundles = payload
        .config
        .lazy_groups
        .iter()
        .map(|b| b.unpack(&id_map))
        .collect::<Vec<_>>();
    let load_opt = unpack_opt_plugin_load_options(
        config::LoadOption::default(),
        &id_map,
        &payload_opt_plugins,
    );
    let load_opt = unpack_bundle_load_options(load_opt, &id_map, &payload.config.lazy_groups);
    let ftplugin: HashMap<&str, &str> = payload
        .config
        .after
        .ftplugin
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let after_opt = config::AfterOption { ftplugin };

    config::Specs {
        id_map,
        eager_plugins: start_plugins,
        lazy_plugins: opt_plugins,
        lazy_groups: bundles,
        load_option: load_opt,
        after_option: after_opt,
    }
}

#[cfg(test)]
mod tests {
    use crate::payload::{IdMapElement, Meta};

    use super::*;

    mod mother {
        use crate::payload::{IdMapElement, Meta};

        pub fn package_path(name: &str) -> String {
            String::from("/nix/store/") + name
        }

        pub fn plugin_package(name: &str) -> IdMapElement {
            IdMapElement {
                plugin_id: String::from(name),
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
    fn test_make_id_map() {
        let foo_package = IdMapElement {
            plugin_id: String::from("foo"),
            package: mother::package_path("foo"),
        };
        let bar_package = IdMapElement {
            plugin_id: String::from("bar"),
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

        let act = make_id_map(&meta);

        assert_eq!(exp, act);
    }

    #[test]
    fn test_unpack_start_package() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = make_id_map(&meta);
        let plugin = payload::VimStartPlugin::SimplePackage(mother::package_path(name));
        let plugins = vec![plugin.clone()];
        let exp = config::EagerPlugin {
            plugin_id: name,
            ..Default::default()
        };
        let exp_vec = vec![exp.clone()];

        let act = plugin.unpack(&id_map);
        let act_vec = plugins
            .iter()
            .map(|p| p.unpack(&id_map))
            .collect::<Vec<_>>();

        assert_eq!(exp, act);
        assert_eq!(exp_vec, act_vec);
    }

    #[test]
    fn test_unpack_start_plugin() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = make_id_map(&meta);
        let code = "foo-code";
        let args = serde_json::json!({"foo-arg": 1});
        let plugin = payload::VimStartPlugin::ConfiguredPackage(payload::PluginStartConfig {
            plugin: mother::package_path(name),
            startup_config: payload::Config::Detail(crate::payload::DetailConfig {
                language: crate::payload::Language::Lua,
                code: String::from(code),
                args: args.clone(),
            }),
            extra_packages: vec![],
        });
        let plugins = vec![plugin.clone()];
        let exp = config::EagerPlugin {
            plugin_id: name,
            startup_config: config::PluginConfig {
                language: Language::Lua,
                code: &code,
                args: &args,
            },
        };
        let exp_vec = vec![exp.clone()];

        let act = plugin.unpack(&id_map);
        let act_vec = plugins
            .iter()
            .map(|p| p.unpack(&id_map))
            .collect::<Vec<_>>();

        assert_eq!(exp, act);
        assert_eq!(exp_vec, act_vec);
    }

    #[test]
    fn test_unpack_opt_package() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = make_id_map(&meta);
        let plugin = payload::VimOptPlugin::SimplePackage(mother::package_path(name));
        let plugins = vec![plugin.clone()];
        let exp = vec![config::LazyPlugin {
            plugin_id: name,
            ..Default::default()
        }];
        let exp_vec = exp.clone();

        let act = plugin.unpack(&id_map);
        let act_vec = plugins
            .iter()
            .flat_map(|p| p.unpack(&id_map))
            .collect::<Vec<_>>();

        assert_eq!(exp, act);
        assert_eq!(exp_vec, act_vec);
    }

    #[test]
    fn test_unpack_opt_plugin() {
        let name = "foo-vim";
        let name2 = "bar-vin";
        let meta = mother::meta(vec![name, name2]);
        let id_map = make_id_map(&meta);
        let startup_code = "foo-startup";
        let config_code = "foo-config";
        let pre_config_code = "foo-pre-config";
        let startup_arg = serde_json::json!({"foo-startup-arg": 1});
        let config_arg = serde_json::json!({"foo-config-arg": 2});
        let pre_config_arg = serde_json::json!({"foo-pre-config-arg": 3});
        let plugin = payload::VimOptPlugin::ConfiguredPackage(payload::PluginOptConfig {
            plugin: mother::package_path(name),
            startup_config: payload::Config::Detail(payload::DetailConfig {
                language: payload::Language::Vim,
                code: String::from(startup_code),
                args: startup_arg.clone(),
            }),
            pre_config: payload::Config::Detail(payload::DetailConfig {
                language: payload::Language::Lua,
                code: String::from(pre_config_code),
                args: pre_config_arg.clone(),
            }),
            post_config: payload::Config::Detail(payload::DetailConfig {
                language: crate::payload::Language::Lua,
                code: String::from(config_code),
                args: config_arg.clone(),
            }),
            depend_plugins: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name2,
            ))],
            ..Default::default()
        });
        let plugins = vec![plugin.clone()];
        let exp = vec![
            config::LazyPlugin {
                plugin_id: name,
                startup_config: config::PluginConfig {
                    language: Language::Vim,
                    code: &startup_code,
                    args: &startup_arg,
                },
                pre_config: config::PluginConfig {
                    language: Language::Lua,
                    code: &pre_config_code,
                    args: &pre_config_arg,
                },
                config: config::PluginConfig {
                    language: Language::Lua,
                    code: &config_code,
                    args: &config_arg,
                },
            },
            config::LazyPlugin {
                plugin_id: name2,
                ..Default::default()
            },
        ];
        let exp_vec = exp.clone();

        let act = plugin.unpack(&id_map);
        let act_vec = plugins
            .iter()
            .flat_map(|p| p.unpack(&id_map))
            .collect::<Vec<_>>();

        // TODO: soft assert, ignore order
        assert_eq!(exp, act);
        assert_eq!(exp_vec, act_vec);
    }

    #[test]
    fn test_unpack_bundle() {
        let bundle_name = "hoge";
        let depend_bundle_name = "huga";
        let name1 = "foo-vim";
        let name2 = "bar-vim";
        let meta = mother::meta(vec![name1, name2]);
        let id_map = make_id_map(&meta);
        let startup_code = "foo-startup";
        let config_code = "foo-config";
        let pre_config_code = "foo-pre-config";
        let startup_arg = serde_json::json!({"foo-startup-arg": 1});
        let config_arg = serde_json::json!({"foo-config-arg": 2});
        let pre_config_arg = serde_json::json!({"foo-pre-config-arg": 3});
        let bundle = payload::LazyGroup {
            name: String::from(bundle_name),
            plugins: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name1,
            ))],
            startup_config: payload::Config::Detail(payload::DetailConfig {
                language: payload::Language::Vim,
                code: String::from(startup_code),
                args: startup_arg.clone(),
            }),
            extra_packages: vec![],
            pre_config: payload::Config::Detail(payload::DetailConfig {
                language: payload::Language::Lua,
                code: String::from(pre_config_code),
                args: pre_config_arg.clone(),
            }),
            post_config: payload::Config::Detail(payload::DetailConfig {
                language: payload::Language::Lua,
                code: String::from(config_code),
                args: config_arg.clone(),
            }),
            depend_plugins: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name2,
            ))],
            depend_groups: vec![depend_bundle_name.to_string()],
            ..Default::default()
        };
        let exp = config::LazyGroup {
            group_id: bundle_name,
            plugin_ids: vec![name1],
            startup_config: config::PluginConfig {
                language: Language::Vim,
                code: &startup_code,
                args: &startup_arg,
            },
            pre_config: config::PluginConfig {
                language: Language::Lua,
                code: &pre_config_code,
                args: &pre_config_arg,
            },
            post_config: config::PluginConfig {
                language: Language::Lua,
                code: &config_code,
                args: &config_arg,
            },
        };

        let act = bundle.unpack(&id_map);

        assert_eq!(exp, act);
    }

    #[test]
    fn test_unpack_opt_package_load_options() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = make_id_map(&meta);
        let plugin = payload::VimOptPlugin::SimplePackage(mother::package_path(name));
        let plugins = vec![plugin.clone()];
        let exp = config::LoadOption {
            depend_plugins: HashMap::from([(name, vec![])]),
            depend_groups: HashMap::from([(name, vec![])]),
            on_modules: HashMap::from([]),
            on_events: HashMap::from([]),
            on_filetypes: HashMap::from([]),
            on_commands: HashMap::from([]),
            timer_clients: vec![],
            denops_clients: vec![],
        };

        let act = super::unpack_opt_plugin_load_options(
            config::LoadOption::default(),
            &id_map,
            &plugins.iter().collect(),
        );

        assert_eq!(exp, act);
    }

    #[test]
    fn test_unpack_opt_plugin_load_options() {
        let name1 = "foo-vim";
        let name2 = "bar-vim";
        let depend_bundle_name = "hoge";
        let module_name = "module";
        let event_name = "event";
        let filetype_name = "filetype";
        let command_name = "command";
        let meta = mother::meta(vec![name1, name2]);
        let id_map = make_id_map(&meta);
        let plugin = payload::VimOptPlugin::ConfiguredPackage(payload::PluginOptConfig {
            plugin: mother::package_path(name1),
            depend_plugins: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name2,
            ))],
            depend_groups: vec![depend_bundle_name.to_string()],
            on_modules: vec![module_name.to_string()],
            on_events: vec![event_name.to_string()],
            on_filetypes: vec![filetype_name.to_string()],
            on_commands: vec![command_name.to_string()],
            use_timer: true,
            use_denops: true,
            ..Default::default()
        });
        let exp = config::LoadOption {
            depend_plugins: HashMap::from([(name1, vec![name2]), (name2, vec![])]),
            depend_groups: HashMap::from([(name1, vec![depend_bundle_name]), (name2, vec![])]),
            on_modules: HashMap::from([(module_name, vec![name1])]),
            on_events: HashMap::from([(event_name, vec![name1])]),
            on_filetypes: HashMap::from([(filetype_name, vec![name1])]),
            on_commands: HashMap::from([(command_name, vec![name1])]),
            timer_clients: vec![name1],
            denops_clients: vec![name1],
        };

        let act = super::unpack_opt_plugin_load_options(
            config::LoadOption::default(),
            &id_map,
            &vec![&plugin],
        );

        assert_eq!(exp, act);
    }

    #[test]
    fn test_unpack_bundle_load_options() {
        let name1 = "foo-vim";
        let name2 = "bar-vim";
        let bundle_name = "hoge";
        let depend_bundle_name = "huga";
        let module_name = "module";
        let event_name = "event";
        let filetype_name = "filetype";
        let command_name = "command";
        let meta = mother::meta(vec![name1, name2]);
        let id_map = make_id_map(&meta);
        let bundle = payload::LazyGroup {
            name: String::from(bundle_name),
            plugins: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name1,
            ))],
            depend_plugins: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name2,
            ))],
            depend_groups: vec![depend_bundle_name.to_string()],
            on_modules: vec![module_name.to_string()],
            on_events: vec![event_name.to_string()],
            on_filetypes: vec![filetype_name.to_string()],
            on_commands: vec![command_name.to_string()],
            use_timer: true,
            ..Default::default()
        };
        let bundles = vec![bundle];
        let exp = config::LoadOption {
            depend_plugins: HashMap::from([
                (bundle_name, vec![name2]),
                (name1, vec![]),
                (name2, vec![]),
            ]),
            depend_groups: HashMap::from([
                (bundle_name, vec![depend_bundle_name]),
                (name1, vec![]),
                (name2, vec![]),
            ]),
            on_modules: HashMap::from([(module_name, vec![bundle_name])]),
            on_events: HashMap::from([(event_name, vec![bundle_name])]),
            on_filetypes: HashMap::from([(filetype_name, vec![bundle_name])]),
            on_commands: HashMap::from([(command_name, vec![bundle_name])]),
            timer_clients: vec![bundle_name],
            denops_clients: vec![],
        };

        let act =
            super::unpack_bundle_load_options(config::LoadOption::default(), &id_map, &bundles);

        assert_eq!(exp, act);
    }
}
