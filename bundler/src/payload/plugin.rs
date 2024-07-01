pub mod eager;
pub mod lazy;
pub use eager::Component as EagerComponent;
pub use lazy::Component as LazyComponent;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct PluginConfig {
    pub eager: HashMap<String, EagerComponent>,
    pub lazy: HashMap<String, LazyComponent>,
}
