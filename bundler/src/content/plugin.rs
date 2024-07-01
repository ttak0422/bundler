use crate::content::Hashable;
use crate::payload::{self};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct PluginConfig {
    pub id: String,
    pub packages: Vec<String>,
    pub startup_config: String,
    pub pre_config: String,
    pub post_config: String,
    pub depends: Vec<String>,
    pub on_modules: Vec<String>,
    pub on_events: Vec<String>,
    pub on_userevents: Vec<String>,
    pub on_filetypes: Vec<String>,
    pub on_commands: Vec<String>,
    pub is_opt: bool,
    pub is_denops_client: bool,
}

fn derive_packages(package: Option<String>, packages: Vec<String>) -> Vec<String> {
    let mut pkgs = packages;
    if let Some(p) = package {
        pkgs.push(p);
    }
    pkgs
}

impl Hashable for payload::plugin::EagerComponent {}

impl From<payload::plugin::EagerComponent> for PluginConfig {
    fn from(value: payload::plugin::EagerComponent) -> Self {
        PluginConfig {
            id: value.get_hash(),
            packages: derive_packages(value.package, value.packages),
            startup_config: String::from(value.startup_config),
            is_opt: false,
            ..Default::default()
        }
    }
}

impl Hashable for payload::plugin::LazyComponent {}

impl From<payload::plugin::LazyComponent> for Vec<PluginConfig> {
    fn from(value: payload::plugin::LazyComponent) -> Self {
        let mut configs = vec![];
        let config = PluginConfig {
            id: value.get_hash(),
            packages: derive_packages(value.package, value.packages),
            startup_config: String::from(value.startup_config),
            pre_config: String::from(value.pre_config),
            post_config: String::from(value.post_config),
            depends: vec![], // set values after all dependencies resolved
            on_modules: value.hooks.modules,
            on_events: value.hooks.events,
            on_userevents: value.hooks.user_events,
            on_filetypes: value.hooks.file_types,
            on_commands: value.hooks.commands,
            is_opt: true,
            is_denops_client: value.use_denops,
        };
        let (depend_packages, mut depend_components): (Vec<String>, Vec<PluginConfig>) = value
            .depends
            .into_iter()
            .fold((vec![], vec![]), |(mut pkgs, mut cmps), p| {
                match p {
                    payload::plugin::lazy::PackageOrComponent::Package(p) => pkgs.push(p),
                    payload::plugin::lazy::PackageOrComponent::Component(c) => {
                        let mut components = Vec::<PluginConfig>::from(c);
                        cmps.append(&mut components);
                    }
                }
                (pkgs, cmps)
            });
        let mut depends = depend_packages;
        for cmp in depend_components.iter() {
            depends.push(cmp.id.clone());
        }
        configs.push(PluginConfig { depends, ..config });
        configs.append(&mut depend_components);
        configs
    }
}
