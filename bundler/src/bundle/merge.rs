use anyhow::{bail, Result};
use itertools::Itertools;

use crate::bundle::config;

pub trait Mergeable
where
    Self: std::default::Default + std::cmp::Eq + std::fmt::Debug,
{
    fn id(&self) -> &str;
    fn modified(&self) -> bool;
    fn merge(self, other: Self) -> Result<Self> {
        if self.id() != "" && other.id() != "" && self.id() != other.id() {
            bail!(
                "Illegal merge attempted (`{}` with `{}`).",
                self.id(),
                other.id()
            )
        }
        let self_modified = self.modified();
        let other_modified = other.modified();
        if self_modified && other_modified && self != other {
            bail!("Conflicted {}\n{:?}\n{:?}.", self.id(), self, other)
        } else if self_modified {
            Ok(self)
        } else {
            Ok(other)
        }
    }
}

impl<'a> Mergeable for config::Component<'a> {
    fn id(&self) -> &str {
        self.id
    }

    fn modified(&self) -> bool {
        let base = config::Component {
            id: self.id,
            is_plugin: self.is_plugin.clone(),
            // compare other fields
            ..Default::default()
        };
        self != &base
    }
}

pub fn merge_vector<T: Mergeable>(xs: Vec<T>) -> Result<Vec<T>> {
    xs.into_iter()
        .into_group_map_by(|x| x.id().to_string())
        .into_values()
        .map(|v| {
            let def: T = Default::default();
            v.into_iter()
                .try_fold(def, |plugin, other| plugin.merge(other))
        })
        .collect()
}
