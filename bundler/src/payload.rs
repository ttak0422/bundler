pub use crate::payload::{
    core::{BundlerConfig, Config, DetailConfig, IdMapElement, Language, Meta, Payload},
    eager::{PluginConfig as PluginStartConfig, VimPluginPackage as VimStartPlugin},
    group::LazyGroup,
    lazy::{PluginConfig as PluginOptConfig, VimPluginPackage as VimOptPlugin},
    operation::expand,
};

mod core;
mod eager;
mod group;
mod lazy;
mod operation;
