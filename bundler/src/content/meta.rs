use crate::payload;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Meta {
    pub package_names: HashMap<String, String>,
    pub extra_packages: Vec<String>,
}

impl From<payload::meta::Meta> for Meta {
    fn from(meta: payload::meta::Meta) -> Self {
        let mut package_names = HashMap::new();
        for (pname, path) in meta.package_paths {
            package_names.insert(path, pname);
        }
        Meta {
            package_names,
            extra_packages: meta.extra_packages,
        }
    }
}
