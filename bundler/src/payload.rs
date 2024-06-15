/* Nix friendly vim/neovim config. */
pub use crate::payload::{
    common::{Language, Target},
    config::{AfterOption, BundlerConfig, Config, DetailConfig, IdMapElement, Meta, Payload},
    eager::{PluginConfig as PluginEagerConfig, VimPluginPackage as EagerVimPluginPackage},
    group::LazyGroup,
    lazy::{PluginConfig as PluginLazyConfig, VimPluginPackage as LazyVimPluginPackage},
};

mod common;
mod config;
mod eager;
mod group;
mod lazy;
