use crate::content;
use crate::payload;
use std::collections::HashMap;

pub struct IdTable {
    value: HashMap<String, String>,
}

impl IdTable {
    /// get plugin_id.
    ///
    /// precondition: the key must be the value registered at initialization.
    pub fn get<T: TableKey + ?Sized>(&self, key: &T) -> &str {
        self.value
            .get(key.key())
            .expect("unregistered key was used")
    }
}

pub trait TableKey {
    fn key(&self) -> &str;
}

impl From<Vec<payload::IdMapElement>> for IdTable {
    fn from(value: Vec<payload::IdMapElement>) -> Self {
        let value: HashMap<String, String> = value
            .into_iter()
            .map(|p| (p.package, p.plugin_id))
            .collect();
        Self { value }
    }
}

impl TableKey for str {
    fn key(&self) -> &str {
        self
    }
}

impl TableKey for String {
    fn key(&self) -> &str {
        self.as_str()
    }
}

impl TableKey for content::EagerPlugin {
    fn key(&self) -> &str {
        &self.nix_package
    }
}

impl TableKey for content::LazyPlugin {
    fn key(&self) -> &str {
        &self.nix_package
    }
}

impl TableKey for payload::EagerVimPluginPackage {
    fn key(&self) -> &str {
        match self {
            payload::EagerVimPluginPackage::SimplePackage(pkg) => pkg.as_str(),
            payload::EagerVimPluginPackage::ConfiguredPackage(cfg) => cfg.plugin.as_str(),
        }
    }
}

impl TableKey for payload::LazyVimPluginPackage {
    fn key(&self) -> &str {
        match self {
            payload::LazyVimPluginPackage::SimplePackage(pkg) => pkg.as_str(),
            payload::LazyVimPluginPackage::ConfiguredPackage(cfg) => cfg.plugin.as_str(),
        }
    }
}
