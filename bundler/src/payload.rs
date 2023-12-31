pub use crate::payload::{
    common::Language,
    config::{BundlerConfig, Config, DetailConfig, IdMapElement, Meta, Payload},
    eager::{PluginConfig as PluginStartConfig, VimPluginPackage as VimStartPlugin},
    group::LazyGroup,
    lazy::{PluginConfig as PluginOptConfig, VimPluginPackage as VimOptPlugin},
    operation::expand,
};

mod common;
mod config;
mod eager;
mod group;
mod lazy;
mod operation;
