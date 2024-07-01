use super::config::Config;
use super::lua::LuaPortable;
use super::LoadConfig;
use super::PluginConfig;
use crate::bundle::constant;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::io::Write;
use std::{
    fs::{self, File, OpenOptions},
    path::Path,
};

fn create_or_open_file(path: impl AsRef<Path>) -> Result<File> {
    if let Some(parent_dir) = path.as_ref().parent() {
        fs::create_dir_all(parent_dir)?;
    }
    OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .read(true)
        .open(&path)
        .context("failed to create or open file")
}

fn export_plugin_config(root: &str, cfg: PluginConfig) -> Result<()> {
    let id = cfg.id.to_string();

    let packages_path = [root, constant::dir::PACKAGES, &id].join("/");
    let mut packages_file = create_or_open_file(packages_path)?;
    write!(packages_file, "return {}", cfg.packages.into_lua())?;

    let startup_config_path = [root, constant::dir::STARTUP_CONFIGS, &id].join("/");
    let mut startup_config_file = create_or_open_file(startup_config_path)?;
    if let Some(startup_config) = cfg.startup_config {
        write!(startup_config_file, "{}", startup_config)?;
    }

    let pre_config_path = [root, constant::dir::PRE_CONFIGS, &id].join("/");
    let mut pre_config_file = create_or_open_file(pre_config_path)?;
    if let Some(pre_config) = cfg.pre_config {
        write!(pre_config_file, "{}", pre_config)?;
    } else {
        write!(pre_config_file, "")?;
    }

    let post_config_path = [root, constant::dir::POST_CONFIGS, &id].join("/");
    let mut post_config_file = create_or_open_file(post_config_path)?;
    if let Some(post_config) = cfg.post_config {
        write!(post_config_file, "{}", post_config)?;
    } else {
        write!(post_config_file, "")?;
    }

    let depends_path = [root, constant::dir::DEPENDS, &id].join("/");
    let mut depends_file = create_or_open_file(depends_path)?;
    write!(depends_file, "return {}", cfg.depends.into_lua())?;

    Ok(())
}

fn export_load_config(root: &str, cfg: LoadConfig) -> Result<()> {
    // modules
    let module_keys = cfg.on_modules.keys().collect::<Vec<_>>();
    let module_keys_path = [root, constant::file::MODULE_KEYS].join("/");
    let mut module_keys_file = create_or_open_file(module_keys_path)?;
    write!(module_keys_file, "return {}", module_keys.into_lua())?;
    for (m, ps) in cfg.on_modules {
        let module_path = [root, constant::dir::MODULES, m.as_ref()].join("/");
        let mut module_file = create_or_open_file(module_path)?;
        write!(module_file, "return {}", ps.into_lua())?;
    }

    // events
    let event_keys = cfg.on_events.keys().collect::<Vec<_>>();
    let event_keys_path = [root, constant::file::EVENT_KEYS].join("/");
    let mut event_keys_file = create_or_open_file(event_keys_path)?;
    write!(event_keys_file, "return {}", event_keys.into_lua())?;
    for (ev, ps) in cfg.on_events {
        let event_path = [root, constant::dir::EVENTS, ev.as_ref()].join("/");
        let mut event_file = create_or_open_file(event_path)?;
        write!(event_file, "return {}", ps.into_lua())?;
    }

    // user events
    let user_event_keys = cfg.on_userevents.keys().collect::<Vec<_>>();
    let user_event_keys_path = [root, constant::file::USER_EVENT_KEYS].join("/");
    let mut user_event_keys_file = create_or_open_file(user_event_keys_path)?;
    write!(
        user_event_keys_file,
        "return {}",
        user_event_keys.into_lua()
    )?;
    for (ev, ps) in cfg.on_userevents {
        let user_event_path = [root, constant::dir::USER_EVENTS, ev.as_ref()].join("/");
        let mut user_event_file = create_or_open_file(user_event_path)?;
        write!(user_event_file, "return {}", ps.into_lua())?;
    }

    // commands
    let command_keys = cfg.on_commands.keys().collect::<Vec<_>>();
    let command_keys_path = [root, constant::file::COMMAND_KEYS].join("/");
    let mut command_keys_file = create_or_open_file(command_keys_path)?;
    write!(command_keys_file, "return {}", command_keys.into_lua())?;
    for (cmd, ps) in cfg.on_commands {
        let command_path = [root, constant::dir::COMMANDS, cmd.as_ref()].join("/");
        let mut command_file = create_or_open_file(command_path)?;
        write!(command_file, "return {}", ps.into_lua())?;
    }

    // filetypes
    for (ft, ps) in cfg.on_filetypes {
        let ft_plugin_path = [
            root,
            constant::dir::AFTER,
            constant::dir::FTPLUGIN,
            (ft.clone() + ".lua").as_ref(),
        ]
        .join("/");
        let mut ft_plugin_file = create_or_open_file(ft_plugin_path)?;
        write!(
            ft_plugin_file,
            "
            if not vim.g.bundler_ft_plugins_{} then
                vim.g.bundler_ft_plugins_{} = true
                require('bundler').load_plugins({})
            end
            ",
            &ft,
            &ft,
            ps.into_lua()
        )?;
    }

    // startup
    let startup_config_keys = cfg.startup_config_plugins.into_lua();
    let startup_config_keys_path = [root, constant::file::STARTUP_CONFIG_KEYS].join("/");
    let mut startup_config_keys_file = create_or_open_file(startup_config_keys_path)?;
    write!(startup_config_keys_file, "return {}", startup_config_keys)?;

    // denops
    let denops_keys = cfg.denops_clients.into_lua();
    let denops_keys_path = [root, constant::file::DENOPS_KEYS].join("/");
    let mut denops_keys_file = create_or_open_file(denops_keys_path)?;
    write!(denops_keys_file, "return {}", denops_keys)?;

    // rtp
    for (id, paths) in cfg.rtp {
        let rtp_path = [root, constant::dir::RTP, id.to_string().as_ref()].join("/");
        let mut rtp_file = create_or_open_file(rtp_path)?;
        write!(rtp_file, "return {}", paths.into_lua())?;
    }

    Ok(())
}

fn export_after_config(root: &str, after: HashMap<String, HashMap<String, String>>) -> Result<()> {
    // x: (e.g. ftplugin, ftdetect, plugin, ...)
    for (x, kvp) in after {
        for (key, value) in kvp {
            // key: (e.g. rust, someplugin, ...)
            // value: lua config
            let path = [
                root,
                constant::dir::AFTER,
                x.as_ref(),
                (key + ".lua").as_ref(),
            ]
            .join("/");
            let mut file = create_or_open_file(path)?;
            write!(file, "{}", value)?;
        }
    }
    Ok(())
}

pub fn export(root: &str, cfg: Config) -> Result<()> {
    for p in cfg.plugins {
        export_plugin_config(root, p)?;
    }
    export_load_config(root, cfg.load_config)?;
    export_after_config(root, cfg.after)?;
    Ok(())
}
