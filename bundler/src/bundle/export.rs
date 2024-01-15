use crate::bundle::{AfterOption, Component, Info, LoadOption};
use crate::constant::dir::{AFTER, FTPLUGIN, INFO, MODULES};
use crate::constant::file::{
    BUNDLER_BIN, COMMAND_KEYS, DENOPS_CLIENTS, EVENT_KEYS, FILETYPE_KEYS, MODULE_KEYS,
    STARTUP_KEYS, TIMER_CLIENTS,
};
use crate::constant::{self, dir};
use crate::util::file::create_file_with_dirs;
use crate::util::lua::{to_lua_flag_table, to_lua_table};
use anyhow::Result;
use std::io::Write;

pub struct ExportOption<'a> {
    pub root_dir: &'a str,
}

pub trait Exporter {
    fn export_file(self, opt: &ExportOption) -> Result<()>;
}

impl<'a> Exporter for Component<'a> {
    fn export_file(self, export_option: &ExportOption) -> Result<()> {
        // plugin
        let mut plugin_file = create_file_with_dirs(
            String::from(export_option.root_dir) + "/" + dir::PLUGIN + "/" + self.id,
        )?;
        if self.is_plugin {
            write!(plugin_file, "return \"{}\"", self.id)?;
        } else {
            write!(plugin_file, "return nil")?;
        }

        // plugins
        let mut plugins_file = create_file_with_dirs(
            String::from(export_option.root_dir) + "/" + dir::PLUGINS + "/" + self.id,
        )?;
        write!(plugins_file, "return {}", to_lua_table(&self.group_plugins))?;

        // startup
        let mut startup_file = create_file_with_dirs(
            String::from(export_option.root_dir) + "/" + dir::STARTUP + "/" + self.id,
        )?;
        write!(startup_file, "{}", self.startup_config)?;

        // pre_config
        let mut pre_config_file = create_file_with_dirs(
            String::from(export_option.root_dir) + "/" + dir::PRE_CONFIG + "/" + self.id,
        )?;
        write!(pre_config_file, "{}", self.pre_config)?;

        // post_config
        let mut post_config_file = create_file_with_dirs(
            String::from(export_option.root_dir) + "/" + dir::POST_CONFIG + "/" + self.id,
        )?;
        write!(post_config_file, "{}", self.post_config)?;

        // depend plugins
        let mut depend_plugins_file = create_file_with_dirs(
            String::from(export_option.root_dir) + "/" + dir::DEPEND_PLUGINS + "/" + self.id,
        )?;
        write!(
            depend_plugins_file,
            "return {}",
            to_lua_table(&self.depend_plugins)
        )?;

        // depend groups
        let mut depend_groups_file = create_file_with_dirs(
            String::from(export_option.root_dir) + "/" + dir::DEPEND_GROUPS + "/" + self.id,
        )?;
        write!(
            depend_groups_file,
            "return {}",
            to_lua_table(&self.depend_groups)
        )?;

        Ok(())
    }
}

impl<'a> Exporter for LoadOption<'a> {
    fn export_file(self, export_option: &ExportOption) -> Result<()> {
        // plugin paths
        for (plugin_id, path) in self.plugin_paths {
            let mut plugin_path_file = create_file_with_dirs(
                String::from(export_option.root_dir) + "/" + constant::dir::RTP + "/" + plugin_id,
            )?;
            write!(plugin_path_file, "return \"{}\"", path)?;
        }

        // startup plugins
        let mut startup_plugins_file =
            create_file_with_dirs(String::from(export_option.root_dir) + "/" + STARTUP_KEYS)?;
        write!(
            startup_plugins_file,
            "return {}",
            to_lua_table(&self.startup_plugins)
        )?;

        // modules
        let mut modules_file =
            create_file_with_dirs(String::from(export_option.root_dir) + "/" + MODULE_KEYS)?;
        let modules = self.on_modules.keys().cloned().collect::<Vec<_>>();
        write!(modules_file, "return {}", to_lua_table(&modules))?;
        for (module, plugins) in self.on_modules {
            let mut file = create_file_with_dirs(
                String::from(export_option.root_dir) + "/" + MODULES + "/" + module,
            )?;
            write!(file, "return {}", to_lua_table(&plugins))?;
        }

        // events
        let mut events_file =
            create_file_with_dirs(String::from(export_option.root_dir) + "/" + EVENT_KEYS)?;
        let events = self.on_events.keys().cloned().collect::<Vec<_>>();
        write!(events_file, "return {}", to_lua_table(&events))?;
        for (event, plugins) in self.on_events {
            let mut file = create_file_with_dirs(
                String::from(export_option.root_dir) + "/" + constant::dir::EVENTS + "/" + event,
            )?;
            write!(file, "return {}", to_lua_table(&plugins))?;
        }

        // filetypes
        let mut filetypes_file =
            create_file_with_dirs(String::from(export_option.root_dir) + "/" + FILETYPE_KEYS)?;
        let filetypes = self.on_filetypes.keys().cloned().collect::<Vec<_>>();
        write!(filetypes_file, "return {}", to_lua_table(&filetypes))?;
        for (filetype, plugins) in self.on_filetypes {
            let mut file = create_file_with_dirs(
                String::from(export_option.root_dir)
                    + "/"
                    + constant::dir::FILETYPES
                    + "/"
                    + filetype,
            )?;
            write!(file, "return {}", to_lua_table(&plugins))?;
        }

        // commands
        let mut commands_file =
            create_file_with_dirs(String::from(export_option.root_dir) + "/" + COMMAND_KEYS)?;
        let commands = self.on_commands.keys().cloned().collect::<Vec<_>>();
        write!(commands_file, "return {}", to_lua_table(&commands))?;
        for (command, plugins) in self.on_commands {
            let mut file = create_file_with_dirs(
                String::from(export_option.root_dir)
                    + "/"
                    + constant::dir::COMMANDS
                    + "/"
                    + command,
            )?;
            write!(file, "return {}", to_lua_table(&plugins))?;
        }

        // timer clients
        let mut timer_clients_file =
            create_file_with_dirs(String::from(export_option.root_dir) + "/" + TIMER_CLIENTS)?;
        write!(
            timer_clients_file,
            "return {}",
            to_lua_table(&self.timer_clients)
        )?;

        // denops clients
        let mut denops_clients_file =
            create_file_with_dirs(String::from(export_option.root_dir) + "/" + DENOPS_CLIENTS)?;
        write!(
            denops_clients_file,
            "return {}",
            to_lua_flag_table(&self.denops_clients, true)
        )?;

        Ok(())
    }
}

impl<'a> Exporter for AfterOption<'a> {
    fn export_file(self, export_option: &ExportOption) -> Result<()> {
        // filetypes
        for (filetype, code) in self.ftplugin {
            let mut file = create_file_with_dirs(
                String::from(export_option.root_dir)
                    + "/"
                    + AFTER
                    + "/"
                    + FTPLUGIN
                    + "/"
                    + filetype
                    + ".vim",
            )?;
            write!(file, "{}", code)?;
        }

        Ok(())
    }
}

impl<'a> Exporter for Info<'a> {
    fn export_file(self, opt: &ExportOption) -> Result<()> {
        // bundler bin
        let mut bundler_bin_file =
            create_file_with_dirs(String::from(opt.root_dir) + "/" + INFO + "/" + BUNDLER_BIN)?;
        write!(bundler_bin_file, "return \"{}\"", self.bundler_bin)?;

        Ok(())
    }
}
