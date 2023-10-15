use crate::constants::Language;
use crate::content::config;
use crate::payload;
use serde_json::Value;
use std::collections::HashMap;

fn mapping_lang(lang: &payload::Lang) -> Language {
    match lang {
        payload::Lang::Vim => Language::Vim,
        payload::Lang::Lua => Language::Lua,
        payload::Lang::Fennel => Language::Fennel,
    }
}

/// key is package (e.g. `/nix/store/...`), value is id which corresponds to the name of vim plugin.
fn mk_id_map<'a>(meta: &'a payload::Meta) -> HashMap<&'a str, &'a str> {
    meta.id_map
        .iter()
        .map(|p| (p.package.as_str(), p.id.as_str()))
        .collect()
}

pub trait Unpackable<'a, Out> {
    /// pauload to `Out`.
    fn unpack(&'a self, id_map: &HashMap<&'a str, &'a str>) -> Out;

    /// package (e.g. /nix/store/....) to id (e.g. bundler-nvim).
    fn package_to_id(id_map: &HashMap<&'a str, &'a str>, package: &'a str) -> &'a str {
        // the value is guaranteed to exist.
        id_map.get(package).unwrap()
    }
}

impl<'a> Unpackable<'a, config::StartPlugin<'a>> for payload::VimStartPlugin {
    fn unpack(&'a self, id_map: &HashMap<&'a str, &'a str>) -> config::StartPlugin<'a> {
        match self {
            payload::VimStartPlugin::SimplePackage(pkg) => config::StartPlugin {
                id: Self::package_to_id(id_map, pkg.as_str()),
                ..Default::default()
            },
            payload::VimStartPlugin::ConfiguredPackage(cfg) => config::StartPlugin {
                id: Self::package_to_id(id_map, cfg.plugin.as_str()),
                startup: match &cfg.startup {
                    payload::Config::Simple(code) => config::PluginConfig {
                        lang: Language::Lua,
                        code: &code,
                        args: &Value::Null,
                    },
                    payload::Config::Detail(dtl) => config::PluginConfig {
                        lang: mapping_lang(&dtl.lang),
                        code: &dtl.code,
                        args: &dtl.args,
                    },
                },
            },
        }
    }
}

impl<'a> Unpackable<'a, Vec<config::OptPlugin<'a>>> for payload::VimOptPlugin {
    fn unpack(&'a self, id_map: &HashMap<&'a str, &'a str>) -> Vec<config::OptPlugin<'a>> {
        match self {
            payload::VimOptPlugin::SimplePackage(pkg) => {
                let id = id_map.get(pkg.as_str()).unwrap();
                vec![config::OptPlugin {
                    id,
                    ..Default::default()
                }]
            }
            payload::VimOptPlugin::ConfiguredPackage(cfg) => {
                let id = id_map.get(cfg.plugin.as_str()).unwrap();
                let depends = cfg
                    .depends
                    .iter()
                    .flat_map(|p| p.unpack(&id_map))
                    .collect::<Vec<_>>();
                let startup = match &cfg.startup {
                    payload::Config::Simple(code) => config::PluginConfig {
                        lang: Language::Lua,
                        code: &code,
                        args: &Value::Null,
                    },
                    payload::Config::Detail(dtl) => config::PluginConfig {
                        lang: mapping_lang(&dtl.lang),
                        code: &dtl.code,
                        args: &dtl.args,
                    },
                };
                let pre_config = match &cfg.pre_config {
                    payload::Config::Simple(code) => config::PluginConfig {
                        lang: Language::Lua,
                        code: &code,
                        args: &Value::Null,
                    },
                    payload::Config::Detail(dtl) => config::PluginConfig {
                        lang: mapping_lang(&dtl.lang),
                        code: &dtl.code,
                        args: &dtl.args,
                    },
                };
                let config = match &cfg.config {
                    payload::Config::Simple(code) => config::PluginConfig {
                        lang: Language::Lua,
                        code: &code,
                        args: &Value::Null,
                    },
                    payload::Config::Detail(dtl) => config::PluginConfig {
                        lang: mapping_lang(&dtl.lang),
                        code: &dtl.code,
                        args: &dtl.args,
                    },
                };
                vec![
                    vec![config::OptPlugin {
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
}

impl<'a> Unpackable<'a, config::Bundle<'a>> for payload::Bundle {
    fn unpack(&'a self, id_map: &HashMap<&'a str, &'a str>) -> config::Bundle<'a> {
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
        let startup = match &self.startup {
            payload::Config::Simple(code) => config::PluginConfig {
                lang: Language::Lua,
                code: &code,
                args: &Value::Null,
            },
            payload::Config::Detail(dtl) => config::PluginConfig {
                lang: mapping_lang(&dtl.lang),
                code: &dtl.code,
                args: &dtl.args,
            },
        };
        let pre_config = match &self.pre_config {
            payload::Config::Simple(code) => config::PluginConfig {
                lang: Language::Lua,
                code: &code,
                args: &Value::Null,
            },
            payload::Config::Detail(dtl) => config::PluginConfig {
                lang: mapping_lang(&dtl.lang),
                code: &dtl.code,
                args: &dtl.args,
            },
        };
        let config = match &self.config {
            payload::Config::Simple(code) => config::PluginConfig {
                lang: Language::Lua,
                code: &code,
                args: &Value::Null,
            },
            payload::Config::Detail(dtl) => config::PluginConfig {
                lang: mapping_lang(&dtl.lang),
                code: &dtl.code,
                args: &dtl.args,
            },
        };
        config::Bundle {
            id: self.name.as_str(),
            plugins,
            startup,
            pre_config,
            config,
        }
    }
}

fn unpack_opt_plugin_load_options<'a>(
    load_opt: config::LoadingOptions<'a>,
    id_map: &HashMap<&'a str, &'a str>,
    opt_plugins: &Vec<&'a payload::VimOptPlugin>,
) -> config::LoadingOptions<'a> {
    opt_plugins
        .iter()
        .fold(load_opt, |mut load_opt_acc, plugin| match plugin {
            payload::VimOptPlugin::SimplePackage(pkg) => {
                let id = id_map.get(pkg.as_str()).unwrap();
                load_opt_acc.depends.entry(id).or_default();
                load_opt_acc.depend_bundles.entry(id).or_default();
                load_opt_acc
            }
            payload::VimOptPlugin::ConfiguredPackage(cfg) => {
                let id = id_map.get(cfg.plugin.as_str()).unwrap();
                load_opt_acc.depends.entry(id).or_default();
                load_opt_acc.depend_bundles.entry(id).or_default();
                let depends: HashMap<&str, Vec<&str>> = if cfg.depends.is_empty() {
                    load_opt_acc.depends
                } else {
                    cfg.depends.iter().fold(load_opt_acc.depends, |mut acc, p| {
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
                if cfg.use_denops {
                    load_opt_acc.denops_clients.push(id);
                }
                unpack_opt_plugin_load_options(
                    config::LoadingOptions {
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
    load_opt: config::LoadingOptions<'a>,
    id_map: &HashMap<&'a str, &'a str>,
    bundles: &'a Vec<payload::Bundle>,
) -> config::LoadingOptions<'a> {
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
            config::LoadingOptions {
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

pub fn unpack<'a>(payload: &'a payload::Payload) -> config::Specs<'a> {
    let id_map = mk_id_map(&payload.meta);
    let payload_opt_plugins = [
        payload::expand(&payload.cfg.opt_plugins),
        payload::expand(&payload.cfg.bundles),
    ]
    .concat();

    let start_plugins = payload
        .cfg
        .start_plugins
        .iter()
        .map(|p| p.unpack(&id_map))
        .collect::<Vec<_>>();
    let opt_plugins = payload_opt_plugins
        .iter()
        .flat_map(|p| p.unpack(&id_map))
        .collect::<Vec<_>>();
    let bundles = payload
        .cfg
        .bundles
        .iter()
        .map(|b| b.unpack(&id_map))
        .collect::<Vec<_>>();
    let load_opt = unpack_opt_plugin_load_options(
        config::LoadingOptions::default(),
        &id_map,
        &payload_opt_plugins,
    );
    let load_opt = unpack_bundle_load_options(load_opt, &id_map, &payload.cfg.bundles);

    config::Specs {
        id_map,
        start_plugins,
        opt_plugins,
        bundles,
        load_opt,
    }
}

#[cfg(test)]
mod tests {
    use crate::payload::{IdPackage, Meta};

    use super::*;

    mod mother {
        use crate::payload::{IdPackage, Meta};

        pub fn package_path(name: &str) -> String {
            String::from("/nix/store/") + name
        }

        pub fn plugin_package(name: &str) -> IdPackage {
            IdPackage {
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
        let foo_package = IdPackage {
            id: String::from("foo"),
            package: mother::package_path("foo"),
        };
        let bar_package = IdPackage {
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
        let plugin = payload::VimStartPlugin::SimplePackage(mother::package_path(name));
        let plugins = vec![plugin.clone()];
        let exp = config::StartPlugin {
            id: name,
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
    fn unpack_start_plugin() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = mk_id_map(&meta);
        let code = "foo-code";
        let args = serde_json::json!({"foo-arg": 1});
        let plugin = payload::VimStartPlugin::ConfiguredPackage(payload::PluginStartConfig {
            plugin: mother::package_path(name),
            startup: payload::Config::Detail(crate::payload::DetailConfig {
                lang: crate::payload::Lang::Lua,
                code: String::from(code),
                args: args.clone(),
            }),
            extra_packages: vec![],
        });
        let plugins = vec![plugin.clone()];
        let exp = config::StartPlugin {
            id: name,
            startup: config::PluginConfig {
                lang: Language::Lua,
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
    fn unpack_opt_package() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = mk_id_map(&meta);
        let plugin = payload::VimOptPlugin::SimplePackage(mother::package_path(name));
        let plugins = vec![plugin.clone()];
        let exp = vec![config::OptPlugin {
            id: name,
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
        let plugin = payload::VimOptPlugin::ConfiguredPackage(payload::PluginOptConfig {
            plugin: mother::package_path(name),
            startup: payload::Config::Detail(payload::DetailConfig {
                lang: payload::Lang::Vim,
                code: String::from(startup_code),
                args: startup_arg.clone(),
            }),
            pre_config: payload::Config::Detail(payload::DetailConfig {
                lang: payload::Lang::Lua,
                code: String::from(pre_config_code),
                args: pre_config_arg.clone(),
            }),
            config: payload::Config::Detail(payload::DetailConfig {
                lang: crate::payload::Lang::Lua,
                code: String::from(config_code),
                args: config_arg.clone(),
            }),
            depends: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name2,
            ))],
            ..Default::default()
        });
        let plugins = vec![plugin.clone()];
        let exp = vec![
            config::OptPlugin {
                id: name,
                startup: config::PluginConfig {
                    lang: Language::Vim,
                    code: &startup_code,
                    args: &startup_arg,
                },
                pre_config: config::PluginConfig {
                    lang: Language::Lua,
                    code: &pre_config_code,
                    args: &pre_config_arg,
                },
                config: config::PluginConfig {
                    lang: Language::Lua,
                    code: &config_code,
                    args: &config_arg,
                },
            },
            config::OptPlugin {
                id: name2,
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
        let bundle = payload::Bundle {
            name: String::from(bundle_name),
            plugins: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name1,
            ))],
            startup: payload::Config::Detail(payload::DetailConfig {
                lang: payload::Lang::Vim,
                code: String::from(startup_code),
                args: startup_arg.clone(),
            }),
            extra_packages: vec![],
            pre_config: payload::Config::Detail(payload::DetailConfig {
                lang: payload::Lang::Lua,
                code: String::from(pre_config_code),
                args: pre_config_arg.clone(),
            }),
            config: payload::Config::Detail(payload::DetailConfig {
                lang: payload::Lang::Lua,
                code: String::from(config_code),
                args: config_arg.clone(),
            }),
            depends: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name2,
            ))],
            depend_bundles: vec![depend_bundle_name.to_string()],
            ..Default::default()
        };
        let exp = config::Bundle {
            id: bundle_name,
            plugins: vec![name1],
            startup: config::PluginConfig {
                lang: Language::Vim,
                code: &startup_code,
                args: &startup_arg,
            },
            pre_config: config::PluginConfig {
                lang: Language::Lua,
                code: &pre_config_code,
                args: &pre_config_arg,
            },
            config: config::PluginConfig {
                lang: Language::Lua,
                code: &config_code,
                args: &config_arg,
            },
        };

        let act = bundle.unpack(&id_map);

        assert_eq!(exp, act);
    }

    #[test]
    fn unpack_opt_package_load_options() {
        let name = "foo-vim";
        let meta = mother::meta(vec![name]);
        let id_map = mk_id_map(&meta);
        let plugin = payload::VimOptPlugin::SimplePackage(mother::package_path(name));
        let plugins = vec![plugin.clone()];
        let exp = config::LoadingOptions {
            depends: HashMap::from([(name, vec![])]),
            depend_bundles: HashMap::from([(name, vec![])]),
            modules: HashMap::from([]),
            events: HashMap::from([]),
            filetypes: HashMap::from([]),
            commands: HashMap::from([]),
            lazys: vec![],
            denops_clients: vec![],
        };

        let act = super::unpack_opt_plugin_load_options(
            config::LoadingOptions::default(),
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
        let plugin = payload::VimOptPlugin::ConfiguredPackage(payload::PluginOptConfig {
            plugin: mother::package_path(name1),
            depends: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name2,
            ))],
            depend_bundles: vec![depend_bundle_name.to_string()],
            modules: vec![module_name.to_string()],
            events: vec![event_name.to_string()],
            filetypes: vec![filetype_name.to_string()],
            commands: vec![command_name.to_string()],
            lazy: true,
            use_denops: true,
            ..Default::default()
        });
        let exp = config::LoadingOptions {
            depends: HashMap::from([(name1, vec![name2]), (name2, vec![])]),
            depend_bundles: HashMap::from([(name1, vec![depend_bundle_name]), (name2, vec![])]),
            modules: HashMap::from([(module_name, vec![name1])]),
            events: HashMap::from([(event_name, vec![name1])]),
            filetypes: HashMap::from([(filetype_name, vec![name1])]),
            commands: HashMap::from([(command_name, vec![name1])]),
            lazys: vec![name1],
            denops_clients: vec![name1],
        };

        let act = super::unpack_opt_plugin_load_options(
            config::LoadingOptions::default(),
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
        let bundle = payload::Bundle {
            name: String::from(bundle_name),
            plugins: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name1,
            ))],
            depends: vec![payload::VimOptPlugin::SimplePackage(mother::package_path(
                name2,
            ))],
            depend_bundles: vec![depend_bundle_name.to_string()],
            modules: vec![module_name.to_string()],
            events: vec![event_name.to_string()],
            filetypes: vec![filetype_name.to_string()],
            commands: vec![command_name.to_string()],
            lazy: true,
            ..Default::default()
        };
        let bundles = vec![bundle];
        let exp = config::LoadingOptions {
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
            denops_clients: vec![],
        };

        let act =
            super::unpack_bundle_load_options(config::LoadingOptions::default(), &id_map, &bundles);

        assert_eq!(exp, act);
    }
}
