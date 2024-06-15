mod config;
mod export;
mod merge;
pub use crate::bundle::config::{AfterOption, Bundle, Component, Info, LoadOption};
pub use crate::bundle::export::{ExportOption, Exporter};
use crate::bundle::merge::merge_vector;
use crate::content;
use anyhow::Result;
use std::collections::HashMap;

fn mk_component<'a>(
    id_table: &'a content::IdTable,
    package: &'a content::Package,
) -> Component<'a> {
    let id = match package {
        content::Package::EagerPlugin(p) => id_table.get(p),
        content::Package::LazyPlugin(p) => id_table.get(p),
        content::Package::LazyGroup(g) => &g.name,
    };

    let is_plugin = matches!(
        package,
        content::Package::EagerPlugin(_) | content::Package::LazyPlugin(_)
    );

    let startup_config = match package {
        content::Package::EagerPlugin(p) => p.startup_config.as_str(),
        content::Package::LazyPlugin(p) => p.startup_config.as_str(),
        content::Package::LazyGroup(g) => g.startup_config.as_str(),
    };
    let pre_config = match package {
        content::Package::EagerPlugin(_) => "",
        content::Package::LazyPlugin(p) => p.pre_config.as_str(),
        content::Package::LazyGroup(g) => g.pre_config.as_str(),
    };
    let post_config = match package {
        content::Package::EagerPlugin(_) => "",
        content::Package::LazyPlugin(p) => p.post_config.as_str(),
        content::Package::LazyGroup(g) => g.post_config.as_str(),
    };

    let depend_plugins = match package {
        content::Package::EagerPlugin(_) => vec![],
        content::Package::LazyPlugin(p) => {
            let mut ps = p
                .depend_plugin_packages
                .iter()
                .map(|package| id_table.get(package))
                .collect::<Vec<&str>>();
            ps.sort();
            ps.dedup();
            ps
        }
        content::Package::LazyGroup(g) => {
            let mut ps = g
                .depend_plugin_packages
                .iter()
                .map(|package| id_table.get(package))
                .collect::<Vec<&str>>();
            ps.sort();
            ps.dedup();
            ps
        }
    };
    let depend_groups = match package {
        content::Package::EagerPlugin(_) => vec![],
        content::Package::LazyPlugin(p) => {
            let mut ps = p
                .depend_groups
                .iter()
                .map(|p| p.as_str())
                .collect::<Vec<&str>>();
            ps.sort();
            ps.dedup();
            ps
        }
        content::Package::LazyGroup(g) => {
            let mut ps = g
                .depend_groups
                .iter()
                .map(|p| p.as_str())
                .collect::<Vec<&str>>();
            ps.sort();
            ps.dedup();
            ps
        }
    };

    let group_plugins = match package {
        content::Package::EagerPlugin(_) => vec![],
        content::Package::LazyPlugin(_) => vec![],
        content::Package::LazyGroup(g) => {
            let mut ps = g
                .plugins
                .iter()
                .map(|package| id_table.get(package))
                .collect::<Vec<&str>>();
            ps.sort();
            ps.dedup();
            ps
        }
    };

    Component {
        id,
        is_plugin,
        startup_config,
        pre_config,
        post_config,
        depend_plugins,
        depend_groups,
        group_plugins,
    }
}

fn mk_after_option(option: &content::AfterOption) -> AfterOption {
    let mut ftplugin = HashMap::new();
    for (k, v) in &option.ftplugin {
        ftplugin.insert(k.as_str(), v.as_str());
    }
    AfterOption { ftplugin }
}

pub fn bundle(config: &content::Content) -> Bundle {
    let mut components = Vec::new();
    let mut load_option = LoadOption::default();

    for package in &config.packages {
        components.push(mk_component(&config.id_table, package));
        match package {
            content::Package::EagerPlugin(p) => {
                let id = config.id_table.get(p);
                load_option.plugin_paths.insert(id, p.nix_package.as_str());
                if !p.startup_config.is_empty() {
                    load_option.startup_config_plugins.push(id);
                }
            }
            content::Package::LazyPlugin(p) => {
                let id = config.id_table.get(p);

                load_option.plugin_paths.insert(id, p.nix_package.as_str());

                if !p.startup_config.is_empty() {
                    load_option.startup_config_plugins.push(id);
                }

                for module in &p.on_modules {
                    load_option
                        .on_modules
                        .entry(module.as_str())
                        .or_default()
                        .push(id);
                }
                for event in &p.on_events {
                    load_option
                        .on_events
                        .entry(event.as_str())
                        .or_default()
                        .push(id);
                }
                for filetype in &p.on_filetypes {
                    load_option
                        .on_filetypes
                        .entry(filetype.as_str())
                        .or_default()
                        .push(id);
                }
                for command in &p.on_commands {
                    load_option
                        .on_commands
                        .entry(command.as_str())
                        .or_default()
                        .push(id);
                }

                if p.is_timer_client {
                    load_option.timer_clients.push(id);
                }
                if p.is_denops_client {
                    load_option.denops_clients.push(id);
                }
            }
            content::Package::LazyGroup(g) => {
                let id = g.name.as_str();

                if !g.startup_config.is_empty() {
                    load_option.startup_config_plugins.push(id);
                }

                for module in &g.on_modules {
                    load_option
                        .on_modules
                        .entry(module.as_str())
                        .or_default()
                        .push(id);
                }
                for event in &g.on_events {
                    load_option
                        .on_events
                        .entry(event.as_str())
                        .or_default()
                        .push(id);
                }
                for filetype in &g.on_filetypes {
                    load_option
                        .on_filetypes
                        .entry(filetype.as_str())
                        .or_default()
                        .push(id);
                }
                for command in &g.on_commands {
                    load_option
                        .on_commands
                        .entry(command.as_str())
                        .or_default()
                        .push(id);
                }
                if g.is_timer_client {
                    load_option.timer_clients.push(id);
                }
            }
        }
    }

    for plugins in load_option.on_modules.values_mut() {
        plugins.sort();
        plugins.dedup();
    }
    for plugins in load_option.on_events.values_mut() {
        plugins.sort();
        plugins.dedup();
    }
    for plugins in load_option.on_filetypes.values_mut() {
        plugins.sort();
        plugins.dedup();
    }
    for plugins in load_option.on_commands.values_mut() {
        plugins.sort();
        plugins.dedup();
    }
    load_option.startup_config_plugins.sort();
    load_option.startup_config_plugins.dedup();
    load_option.timer_clients.sort();
    load_option.timer_clients.dedup();
    load_option.denops_clients.sort();
    load_option.denops_clients.dedup();

    let components = merge_vector(components).unwrap();

    Bundle {
        components,
        load_option,
        after_option: mk_after_option(&config.after_option),
        info: Info {
            bundler_bin: config.info.bundler_bin.as_str(),
        },
    }
}

pub fn export<'a>(bundle: Bundle<'a>, export_option: ExportOption<'a>) -> Result<()> {
    // components
    for component in bundle.components {
        component.export_file(&export_option)?;
    }

    // load options
    bundle.load_option.export_file(&export_option)?;

    // after options
    bundle.after_option.export_file(&export_option)?;

    // info
    bundle.info.export_file(&export_option)?;

    Ok(())
}
