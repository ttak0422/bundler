use anyhow::{bail, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::io::Write;

use crate::collection_util::{to_unique_map, to_unique_vector};
use crate::constants::{dir, file, Language};
use crate::content::{
    AfterOptions, Bundle, LoadingOptions, OptPlugin, PluginConfig, Specs, StartPlugin,
};
use crate::lua::{to_lua_flag_table, to_lua_table};

trait Bundleable
where
    Self: std::default::Default + std::cmp::Eq + std::fmt::Debug,
{
    fn id(&self) -> &str;
    fn modified(&self) -> bool;
    fn bundle(self, other: Self) -> Result<Self> {
        if self.id() != "" && other.id() != "" && self.id() != other.id() {
            bail!(
                "Illegal bundle attempted (`{}` with `{}`).",
                self.id(),
                other.id()
            )
        }
        let self_modified = self.modified();
        let other_modified = other.modified();
        if self_modified && other_modified && self != other {
            bail!("Conflicted {}\n{:?}\n{:?}.", self.id(), self, other)
        } else if self_modified {
            Ok(self)
        } else {
            Ok(other)
        }
    }
}

impl<'a> Bundleable for StartPlugin<'a> {
    fn id(&self) -> &str {
        &self.id
    }
    fn modified(&self) -> bool {
        let default = StartPlugin {
            id: self.id(),
            ..Default::default()
        };
        *self != default
    }
}

impl<'a> Bundleable for OptPlugin<'a> {
    fn id(&self) -> &str {
        &self.id
    }
    fn modified(&self) -> bool {
        let default = OptPlugin {
            id: self.id(),
            ..Default::default()
        };
        *self != default
    }
}

impl<'a> Bundleable for Bundle<'a> {
    fn id(&self) -> &str {
        &self.id
    }
    fn modified(&self) -> bool {
        let default = Bundle {
            id: self.id(),
            ..Default::default()
        };
        *self != default
    }
}

fn bundle_vector<T>(xs: Vec<T>) -> Result<Vec<T>>
where
    T: Bundleable,
{
    xs.into_iter()
        .into_group_map_by(|x| x.id().to_string())
        .into_values()
        .map(|v| {
            let def: T = Default::default();
            v.into_iter()
                .try_fold(def, |plugin, other| plugin.bundle(other))
        })
        .collect()
}

fn bundle_specs(specs: Specs) -> Result<Specs> {
    let start_plugins = bundle_vector(specs.start_plugins)?;
    let opt_plugins = bundle_vector(specs.opt_plugins)?;
    let bundles = bundle_vector(specs.bundles)?;
    Ok(Specs {
        id_map: specs.id_map,
        start_plugins,
        opt_plugins,
        bundles,
        load_opt: LoadingOptions {
            depends: to_unique_map(specs.load_opt.depends),
            depend_bundles: to_unique_map(specs.load_opt.depend_bundles),
            modules: to_unique_map(specs.load_opt.modules),
            events: to_unique_map(specs.load_opt.events),
            filetypes: to_unique_map(specs.load_opt.filetypes),
            commands: to_unique_map(specs.load_opt.commands),
            lazys: to_unique_vector(specs.load_opt.lazys),
            denops_clients: to_unique_vector(specs.load_opt.denops_clients),
        },
        after_opt: specs.after_opt,
    })
}

fn mk_args_code(cfg: &PluginConfig) -> Option<String> {
    if match *cfg.args {
        serde_json::Value::Object(_) => false,
        // ignore Null, Bool, Number, String, Array
        _ => true,
    } {
        return None;
    }

    let args_json = serde_json::to_string(&cfg.args).unwrap();
    if args_json == "{}" {
        return None;
    }
    let args_code = match cfg.lang {
        Language::Vim => {
            format!("let s:args = json_decode('{}')", args_json)
        }
        Language::Lua => {
            format!("local args = vim.json.decode([[{}]])", args_json)
        }
        Language::Fennel => {
            panic!("fennel is not supported yet")
        }
    };
    Some(args_code)
}

fn mk_plugin_config_code(cfg: &PluginConfig) -> String {
    match cfg.lang {
        Language::Vim => {
            let args_code = mk_args_code(cfg).unwrap_or("".to_string());
            format!("vim.cmd([[{}\n{}]])", args_code, cfg.code)
        }
        Language::Lua => {
            let args_code = mk_args_code(cfg).unwrap_or("".to_string());
            format!("{}\n{}", args_code, cfg.code)
        }
        Language::Fennel => {
            panic!("fennel is not supported. use other plugin.")
        }
    }
}

fn bundle_setup_dir(root_dir: &str) -> Result<()> {
    for ds in [
        vec![dir::PLUGIN],
        vec![dir::PLUGINS],
        vec![dir::PRE_CONFIG],
        vec![dir::STARTUP],
        vec![dir::CONFIG],
        vec![dir::DEPENDS],
        vec![dir::DEPEND_BUNDLES],
        vec![dir::MODULES],
        vec![dir::EVENTS],
        vec![dir::FILETYPES],
        vec![dir::COMMANDS],
        vec![dir::RTP],
        vec![dir::AFTER],
        vec![dir::AFTER, dir::FTPLUGIN],
    ]
    .iter()
    {
        create_dir(String::from(root_dir) + "/" + &ds.join("/"))?;
    }
    Ok(())
}

fn bundle_plugins(
    root_dir: &str,
    start_plugins: Vec<StartPlugin>,
    opt_plugins: Vec<OptPlugin>,
    bundles: Vec<Bundle>,
) -> Result<()> {
    // plugin
    for plugin in &opt_plugins {
        let mut file = File::create(String::from(root_dir) + "/" + dir::PLUGIN + "/" + &plugin.id)?;
        write!(file, "return \"{}\"", &plugin.id)?;
    }
    for bundle in &bundles {
        let mut file = File::create(String::from(root_dir) + "/" + dir::PLUGIN + "/" + &bundle.id)?;
        write!(file, "return nil")?;
    }

    // plugins
    for plugin in &opt_plugins {
        let mut file =
            File::create(String::from(root_dir) + "/" + dir::PLUGINS + "/" + &plugin.id)?;
        write!(file, "return {}", to_lua_table(&[].as_slice()))?;
    }
    for bundle in &bundles {
        let mut file =
            File::create(String::from(root_dir) + "/" + dir::PLUGINS + "/" + &bundle.id)?;
        write!(file, "return {}", to_lua_table(&bundle.plugins))?;
    }

    // startup
    let startup_configs = start_plugins
        .iter()
        .filter(|p| p.startup.code != "")
        .collect::<Vec<_>>();
    let opt_startup_configs = opt_plugins
        .iter()
        .filter(|p| p.startup.code != "")
        .collect::<Vec<_>>();
    let bundle_startup_configs = bundles
        .iter()
        .filter(|p| p.startup.code != "")
        .collect::<Vec<_>>();
    let startup_config_ids = &startup_configs
        .iter()
        .map(|p| p.id)
        .chain(opt_startup_configs.iter().map(|p| p.id))
        .chain(bundle_startup_configs.iter().map(|p| p.id))
        .collect::<Vec<_>>();
    let mut startup_keys = File::create(String::from(root_dir) + "/" + file::STARTUP_KEYS)?;
    write!(startup_keys, "return {}", to_lua_table(&startup_config_ids))?;
    for cfg in &startup_configs {
        let mut file = File::create(String::from(root_dir) + "/" + dir::STARTUP + "/" + &cfg.id)?;
        write!(file, "{}", mk_plugin_config_code(&cfg.startup))?;
    }
    for cfg in &opt_startup_configs {
        let mut file = File::create(String::from(root_dir) + "/" + dir::STARTUP + "/" + &cfg.id)?;
        write!(file, "{}", mk_plugin_config_code(&cfg.startup))?;
    }
    for cfg in &bundle_startup_configs {
        let mut file = File::create(String::from(root_dir) + "/" + dir::STARTUP + "/" + &cfg.id)?;
        write!(file, "{}", mk_plugin_config_code(&cfg.startup))?;
    }

    // config
    for plugin in &opt_plugins {
        let mut file = File::create(String::from(root_dir) + "/" + dir::CONFIG + "/" + &plugin.id)?;
        write!(file, "{}", mk_plugin_config_code(&plugin.config))?;
    }
    for bundle in &bundles {
        let mut file = File::create(String::from(root_dir) + "/" + dir::CONFIG + "/" + &bundle.id)?;
        write!(file, "{}", mk_plugin_config_code(&bundle.config))?;
    }

    // pre_config
    for plugin in &opt_plugins {
        let mut file =
            File::create(String::from(root_dir) + "/" + dir::PRE_CONFIG + "/" + &plugin.id)?;
        write!(file, "{}", mk_plugin_config_code(&plugin.pre_config))?;
    }
    for bundle in &bundles {
        let mut file =
            File::create(String::from(root_dir) + "/" + dir::PRE_CONFIG + "/" + &bundle.id)?;
        write!(file, "{}", mk_plugin_config_code(&bundle.pre_config))?;
    }

    Ok(())
}

fn bundle_load_options(root_dir: &str, load_opt: LoadingOptions) -> Result<()> {
    // depends
    for (id, plugins) in &load_opt.depends {
        let mut file = File::create(String::from(root_dir) + "/" + dir::DEPENDS + "/" + id)?;
        write!(file, "return {}", to_lua_table(plugins))?;
    }

    // depend_bundles
    for (id, bundles) in &load_opt.depend_bundles {
        let mut file = File::create(String::from(root_dir) + "/" + dir::DEPEND_BUNDLES + "/" + id)?;
        write!(file, "return {}", to_lua_table(bundles))?;
    }

    // modules
    let mut modules = File::create(String::from(root_dir) + "/" + file::MODULE_KEYS)?;
    write!(
        modules,
        "return {}",
        to_lua_table(&load_opt.modules.keys().cloned().collect::<Vec<_>>())
    )?;
    for (module, plugins) in &load_opt.modules {
        let mut file = File::create(String::from(root_dir) + "/" + dir::MODULES + "/" + module)?;
        write!(file, "return {}", to_lua_table(plugins))?;
    }

    // events
    let mut events = File::create(String::from(root_dir) + "/" + file::EVENT_KEYS)?;
    write!(
        events,
        "return {}",
        to_lua_table(&load_opt.events.keys().cloned().collect::<Vec<_>>())
    )?;
    for (event, plugins) in &load_opt.events {
        let mut file = File::create(String::from(root_dir) + "/" + dir::EVENTS + "/" + event)?;
        write!(file, "return {}", to_lua_table(plugins))?;
    }

    // filetypes
    let mut filetypes = File::create(String::from(root_dir) + "/" + file::FILETYPE_KEYS)?;
    write!(
        filetypes,
        "return {}",
        to_lua_table(&load_opt.filetypes.keys().cloned().collect::<Vec<_>>())
    )?;
    for (filetype, plugins) in &load_opt.filetypes {
        let mut file =
            File::create(String::from(root_dir) + "/" + dir::FILETYPES + "/" + filetype)?;
        write!(file, "return {}", to_lua_table(plugins))?;
    }

    // commands
    let mut commands = File::create(String::from(root_dir) + "/" + file::COMMAND_KEYS)?;
    write!(
        commands,
        "return {}",
        to_lua_table(&load_opt.commands.keys().cloned().collect::<Vec<_>>())
    )?;
    for (command, plugins) in &load_opt.commands {
        let mut file = File::create(String::from(root_dir) + "/" + dir::COMMANDS + "/" + command)?;
        write!(file, "return {}", to_lua_table(plugins))?;
    }

    // lazys
    let mut lazy_file = File::create(String::from(root_dir) + "/" + file::LAZYS)?;
    write!(lazy_file, "return {}", to_lua_table(&load_opt.lazys))?;

    // denops
    let mut denops_file = File::create(String::from(root_dir) + "/" + file::DENOPS)?;
    write!(
        denops_file,
        "return {}",
        to_lua_flag_table(&load_opt.denops_clients, true)
    )?;

    Ok(())
}

fn bundle_stats(root_dir: &str, payload_path: &str) -> Result<()> {
    let mut payload = File::create(String::from(root_dir) + "/" + file::PAYLOAD)?;
    write!(payload, "return \"{}\"", payload_path)?;
    Ok(())
}

fn bundle_rtp(root_dir: &str, id_map: &HashMap<&str, &str>) -> Result<()> {
    for (full_path, id) in id_map {
        let mut file = File::create(String::from(root_dir) + "/" + dir::RTP + "/" + id)?;
        write!(file, "return \"{}\"", full_path)?;
    }
    Ok(())
}

fn bundle_after_opt(root_dir: &str, after_opt: &AfterOptions) -> Result<()> {
    // ftplugin
    for (ft, config) in &after_opt.ftplugin {
        let mut file = File::create(
            String::from(root_dir) + "/" + dir::AFTER + "/" + dir::FTPLUGIN + "/" + ft + ".vim",
        )?;
        write!(file, "{}", config)?;
    }
    Ok(())
}

pub fn bundle(root_dir: &str, payload_path: &str, specs: Specs) -> Result<()> {
    let bundled_specks = bundle_specs(specs)?;

    bundle_setup_dir(root_dir)?;
    bundle_plugins(
        root_dir,
        bundled_specks.start_plugins,
        bundled_specks.opt_plugins,
        bundled_specks.bundles,
    )?;
    bundle_load_options(root_dir, bundled_specks.load_opt)?;
    bundle_stats(root_dir, payload_path)?;
    bundle_rtp(root_dir, &bundled_specks.id_map)?;
    bundle_after_opt(root_dir, &bundled_specks.after_opt)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_start() {
        let p1_filled = StartPlugin {
            id: "p1",
            startup: PluginConfig {
                lang: Language::Vim,
                args: &serde_json::Value::String(String::from("p1-args")),
                code: "p1-args",
            },
        };
        let p1_simple = StartPlugin {
            id: "p1",
            ..Default::default()
        };
        let p2_simple = StartPlugin {
            id: "p2",
            ..Default::default()
        };
        let arg_empty = vec![];
        let arg_same_filled_simple = vec![p1_filled.clone(), p1_simple.clone()];
        let arg_same_simple_filled = vec![p1_simple.clone(), p1_filled.clone()];
        let arg_same = vec![p1_filled.clone(), p1_filled.clone()];
        let arg_diff: Vec<StartPlugin<'_>> = vec![p1_simple.clone(), p2_simple.clone()];
        let exp_empty: Vec<StartPlugin<'_>> = vec![];
        let exp_same_filled = vec![p1_filled.clone()];
        let exp_diff = vec![p1_simple.clone(), p2_simple.clone()];

        let act_empty = bundle_vector(arg_empty).unwrap();
        let act_same_filled_simple = bundle_vector(arg_same_filled_simple).unwrap();
        let act_same_simple_filled = bundle_vector(arg_same_simple_filled).unwrap();
        let act_same = bundle_vector(arg_same).unwrap();
        let act_diff = bundle_vector(arg_diff).unwrap();

        // TODO: soft assert, ignore order
        assert_eq!(exp_empty, act_empty);
        assert_eq!(exp_same_filled, act_same_filled_simple);
        assert_eq!(exp_same_filled, act_same_simple_filled);
        assert_eq!(exp_same_filled, act_same);
        let exp_diff_ids = itertools::sorted(exp_diff.iter().map(|b| b.id())).collect::<Vec<_>>();
        let act_diff_ids = itertools::sorted(act_diff.iter().map(|b| b.id())).collect::<Vec<_>>();
        assert_eq!(exp_diff_ids, act_diff_ids);
    }

    #[test]
    fn bundle_opt() {
        let p1_filled = OptPlugin {
            id: "p1",
            startup: PluginConfig {
                lang: Language::Vim,
                args: &serde_json::Value::String(String::from("p1-args")),
                code: "p1-args",
            },
            config: PluginConfig {
                lang: Language::Vim,
                args: &serde_json::Value::String(String::from("p1-args")),
                code: "p1-args",
            },
            pre_config: PluginConfig {
                lang: Language::Vim,
                args: &serde_json::Value::String(String::from("p1-args")),
                code: "p1-args",
            },
        };
        let p1_simple = OptPlugin {
            id: "p1",
            ..Default::default()
        };
        let p2_simple = OptPlugin {
            id: "p2",
            ..Default::default()
        };

        let arg_empty = vec![];
        let arg_same_filled_simple = vec![p1_filled.clone(), p1_simple.clone()];
        let arg_same_simple_filled = vec![p1_simple.clone(), p1_filled.clone()];
        let arg_same = vec![p1_filled.clone(), p1_filled.clone()];
        let arg_diff: Vec<OptPlugin<'_>> = vec![p1_simple.clone(), p2_simple.clone()];
        let exp_empty: Vec<OptPlugin<'_>> = vec![];
        let exp_same_filled = vec![p1_filled.clone()];
        let exp_diff = vec![p1_simple.clone(), p2_simple.clone()];

        let act_empty = bundle_vector(arg_empty).unwrap();
        let act_same_filled_simple = bundle_vector(arg_same_filled_simple).unwrap();
        let act_same_simple_filled = bundle_vector(arg_same_simple_filled).unwrap();
        let act_same = bundle_vector(arg_same).unwrap();
        let act_diff = bundle_vector(arg_diff).unwrap();

        // TODO: soft assert, ignore order
        assert_eq!(exp_empty, act_empty);
        assert_eq!(exp_same_filled, act_same_filled_simple);
        assert_eq!(exp_same_filled, act_same_simple_filled);
        assert_eq!(exp_same_filled, act_same);
        let exp_diff_ids = itertools::sorted(exp_diff.iter().map(|b| b.id())).collect::<Vec<_>>();
        let act_diff_ids = itertools::sorted(act_diff.iter().map(|b| b.id())).collect::<Vec<_>>();
        assert_eq!(exp_diff_ids, act_diff_ids);
    }

    #[test]
    fn bundle_bundle() {
        let b1_filled = Bundle {
            id: "b1",
            plugins: vec!["p1"],
            startup: PluginConfig {
                lang: Language::Vim,
                args: &serde_json::Value::String(String::from("b1-args")),
                code: "b1-args",
            },
            config: PluginConfig {
                lang: Language::Vim,
                args: &serde_json::Value::String(String::from("b1-args")),
                code: "b1-args",
            },
            pre_config: PluginConfig {
                lang: Language::Vim,
                args: &serde_json::Value::String(String::from("b1-args")),
                code: "b1-args",
            },
        };
        let b1_simple = Bundle {
            id: "b1",
            ..Default::default()
        };
        let b2_simple = Bundle {
            id: "b2",
            ..Default::default()
        };
        let arg_empty = vec![];
        let arg_same_filled_simple = vec![b1_filled.clone(), b1_simple.clone()];
        let arg_same_simple_filled = vec![b1_simple.clone(), b1_filled.clone()];
        let arg_same = vec![b1_filled.clone(), b1_filled.clone()];
        let arg_diff: Vec<Bundle<'_>> = vec![b1_simple.clone(), b2_simple.clone()];
        let exp_empty: Vec<Bundle<'_>> = vec![];
        let exp_same_filled = vec![b1_filled.clone()];
        let exp_diff = vec![b1_simple.clone(), b2_simple.clone()];

        let act_empty = bundle_vector(arg_empty).unwrap();
        let act_same_filled_simple = bundle_vector(arg_same_filled_simple).unwrap();
        let act_same_simple_filled = bundle_vector(arg_same_simple_filled).unwrap();
        let act_same = bundle_vector(arg_same).unwrap();
        let act_diff = bundle_vector(arg_diff).unwrap();

        // TODO: soft assert, ignore order
        assert_eq!(exp_empty, act_empty);
        assert_eq!(exp_same_filled, act_same_filled_simple);
        assert_eq!(exp_same_filled, act_same_simple_filled);
        assert_eq!(exp_same_filled, act_same);
        let exp_diff_ids = itertools::sorted(exp_diff.iter().map(|b| b.id())).collect::<Vec<_>>();
        let act_diff_ids = itertools::sorted(act_diff.iter().map(|b| b.id())).collect::<Vec<_>>();
        assert_eq!(exp_diff_ids, act_diff_ids);
    }
}
