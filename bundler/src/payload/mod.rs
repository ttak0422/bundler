pub use crate::payload::{
    bundle::Bundle,
    core::{BundlerConfig, Config, DetailConfig, IdPackage, Lang, Meta, Payload},
    operation::expand,
    opt::{PluginConfig as PluginOptConfig, VimPlugin as VimOptPlugin},
    start::{PluginConfig as PluginStartConfig, VimPlugin as VimStartPlugin},
};

mod bundle;
mod core;
mod operation;
mod opt;
mod start;
