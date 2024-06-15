/* Nix friendly neovim config. */
pub use crate::payload::{
    common::{Language, Target},
    config::{AfterOption, Config, DetailConfig, IdMapElement, Payload},
    eager::VimPluginPackage as EagerVimPluginPackage,
    group::LazyGroup,
    lazy::VimPluginPackage as LazyVimPluginPackage,
};

mod common;
mod config;
mod eager;
mod group;
mod lazy;
