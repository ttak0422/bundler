/* Rust friendly neovim config. */

mod common;
mod config;
mod from_target;
mod id_table;

use crate::content::common::Target;
pub use crate::content::config::{AfterOption, Content, EagerPlugin, Info, LazyPlugin, Package};
use crate::content::from_target::FromTarget;
// TODO: capsule
pub use crate::content::id_table::IdTable;
use crate::payload;

pub fn unpack(payload: payload::Payload) -> Content {
    let target = Target::from(payload.meta.target);
    let mut packages = payload
        .config
        .eager_plugins
        .into_iter()
        .map(|p| Package::EagerPlugin(EagerPlugin::from_target(p, &target)))
        .collect::<Vec<Package>>();
    payload
        .config
        .lazy_plugins
        .into_iter()
        .flat_map(|p| Vec::from_target(p, &target))
        .for_each(|p| packages.push(p));
    payload
        .config
        .lazy_groups
        .into_iter()
        .flat_map(|p| Vec::from_target(p, &target))
        .for_each(|p| packages.push(p));
    let id_table = IdTable::from(payload.meta.id_map);
    let after_option = AfterOption::from(payload.config.after);

    let info = Info {
        bundler_bin: payload.meta.bundler_bin,
    };

    Content {
        packages,
        id_table,
        after_option,
        info,
    }
}
