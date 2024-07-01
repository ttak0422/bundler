pub mod config;
pub mod constant;
pub mod export;
pub mod incremental_id_generator;
pub mod lua;

use std::collections::HashMap;

use crate::content;
use anyhow::Result;
use config::{Config, LoadConfig, PluginConfig};
use incremental_id_generator::IncrementalIdGenerator;

pub fn bundle(root: &str, content: content::Content) -> Result<()> {
    // [(path, pname)]
    let package_names = content.meta.package_names;
    // [(id, [path])]
    let mut rtp = HashMap::<i32, Vec<String>>::new();
    let id_generator = IncrementalIdGenerator::<String>::new();
    let mut plugins: Vec<PluginConfig> = vec![];
    let mut load_config = LoadConfig::default();

    for p in content.plugin_configs {
        let id = id_generator.get_id(p.id);
        let mut rtp_paths = Vec::<String>::new();
        for path in p.packages.iter() {
            rtp_paths.push(path.clone());
        }
        rtp.insert(id, rtp_paths);
        let packages = p
            .packages
            .into_iter()
            .map(|p| package_names.get(&p).unwrap().to_string())
            .collect::<Vec<_>>();
        let depends = p
            .depends
            .into_iter()
            .map(|p| id_generator.get_id(p))
            .collect::<Vec<_>>();
        let startup_config = if p.startup_config.is_empty() {
            None
        } else {
            Some(p.startup_config)
        };
        if startup_config.is_some() {
            load_config.startup_config_plugins.push(id);
        }
        let pre_config = if p.pre_config.is_empty() {
            None
        } else {
            Some(p.pre_config)
        };
        let post_config = if p.post_config.is_empty() {
            None
        } else {
            Some(p.post_config)
        };

        plugins.push(PluginConfig {
            id,
            packages,
            depends,
            startup_config,
            pre_config,
            post_config,
        });

        for m in p.on_modules {
            load_config
                .on_modules
                .entry(m)
                .or_insert_with(Vec::new)
                .push(id);
        }
        for ev in p.on_events {
            load_config
                .on_events
                .entry(ev)
                .or_insert_with(Vec::new)
                .push(id);
        }
        for ev in p.on_userevents {
            load_config
                .on_userevents
                .entry(ev)
                .or_insert_with(Vec::new)
                .push(id);
        }
        for cmd in p.on_commands {
            load_config
                .on_commands
                .entry(cmd)
                .or_insert_with(Vec::new)
                .push(id);
        }
        if !p.is_opt {
            load_config.startup_plugins.push(id);
        }
        if p.is_denops_client {
            load_config.denops_clients.push(id);
        }

        for ft in p.on_filetypes {
            load_config
                .on_filetypes
                .entry(ft)
                .or_insert_with(Vec::new)
                .push(id);
        }
    }

    export::export(
        root,
        Config {
            plugins,
            load_config,
            after: content.vim.after,
        },
    )
}
