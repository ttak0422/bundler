use anyhow::{bail, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::{create_dir, File};
use std::hash::Hash;
use std::io::Write;

use crate::constants::{dir, file, Language};
use crate::lua::to_lua_table;
use crate::unpack::{Bundle, LoadingOptions, OptPlugin, Pack, PluginConfig, StartPlugin};

trait Bundleable
where
    Self: std::default::Default + std::cmp::Eq + std::fmt::Debug,
{
    fn id(&self) -> &str;
    fn modified(&self) -> bool;
    fn bundle(self, other: Self) -> Result<Self> {
        let self_modified = self.modified();
        let other_modified = other.modified();
        if self_modified && other_modified && self != other {
            bail!("Confliced {}\n{:?}\n{:?}.", self.id(), self, other)
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

fn to_unique_vector<T: Hash + Eq>(v: Vec<T>) -> Vec<T> {
    v.into_iter().collect::<HashSet<_>>().into_iter().collect()
}

fn to_unique_map<T: Hash + Eq>(m: HashMap<&str, Vec<T>>) -> HashMap<&str, Vec<T>> {
    m.into_iter()
        .map(|(k, v)| (k, to_unique_vector(v)))
        .collect()
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

fn bundle_pack(pack: Pack) -> Result<Pack> {
    let start_plugins = bundle_vector(pack.start_plugins)?;
    let opt_plugins = bundle_vector(pack.opt_plugins)?;
    let bundles = bundle_vector(pack.bundles)?;
    Ok(Pack {
        start_plugins,
        opt_plugins,
        bundles,
        load_opt: LoadingOptions {
            depends: to_unique_map(pack.load_opt.depends),
            depend_bundles: to_unique_map(pack.load_opt.depend_bundles),
            modules: to_unique_map(pack.load_opt.modules),
            events: to_unique_map(pack.load_opt.events),
            filetypes: to_unique_map(pack.load_opt.filetypes),
            commands: to_unique_map(pack.load_opt.commands),
            lazys: to_unique_vector(pack.load_opt.lazys),
        },
    })
}

fn mk_args_code(cfg: &PluginConfig) -> Option<String> {
    if *cfg.args == serde_json::Value::Null {
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
            panic!("fennel is not supported yet")
        }
    }
}

fn bundle_setup_dir(root_dir: &str) -> Result<()> {
    for d in [
        dir::PLUGIN,
        dir::PLUGINS,
        dir::PRE_CONFIG,
        dir::CONFIG,
        dir::DEPENDS,
        dir::DEPEND_BUNDLES,
        dir::MODULES,
        dir::EVENTS,
        dir::FILETYPES,
        dir::COMMANDS,
    ]
    .iter()
    {
        create_dir(String::from(root_dir) + "/" + d)?;
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
    let mut startup = File::create(String::from(root_dir) + "/" + file::STARTUP)?;
    for plugin in &start_plugins {
        write!(startup, "{}", mk_plugin_config_code(&plugin.startup))?;
    }
    for plugin in &opt_plugins {
        write!(startup, "{}", mk_plugin_config_code(&plugin.startup))?;
    }
    for bundle in &bundles {
        write!(startup, "{}", mk_plugin_config_code(&bundle.startup))?;
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
    let mut modules = File::create(String::from(root_dir) + "/" + file::MODULES)?;
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
    let mut events = File::create(String::from(root_dir) + "/" + file::EVENTS)?;
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
    let mut filetypes = File::create(String::from(root_dir) + "/" + file::FILETYPES)?;
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
    let mut commands = File::create(String::from(root_dir) + "/" + file::COMMANDS)?;
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

    Ok(())
}

fn bundle_stats(root_dir: &str, payload_path: &str) -> Result<()> {
    let mut payload = File::create(String::from(root_dir) + "/" + file::PAYLOAD)?;
    write!(payload, "return \"{}\"", payload_path)?;
    Ok(())
}

pub fn bundle(root_dir: &str, payload_path: &str, pack: Pack) -> Result<()> {
    let bundled_pack = bundle_pack(pack)?;

    bundle_setup_dir(root_dir)?;
    bundle_plugins(
        root_dir,
        bundled_pack.start_plugins,
        bundled_pack.opt_plugins,
        bundled_pack.bundles,
    )?;
    bundle_load_options(root_dir, bundled_pack.load_opt)?;
    bundle_stats(root_dir, payload_path)?;

    Ok(())
}
