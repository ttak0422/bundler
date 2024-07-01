use crate::payload;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Vim {
    pub after: HashMap<String, HashMap<String, String>>,
}

impl From<payload::vim::VimConfig> for Vim {
    fn from(vim_config: payload::vim::VimConfig) -> Self {
        let mut after = HashMap::new();
        for (x, tree) in vim_config.after {
            let map = after.entry(x).or_insert(HashMap::new());
            for (y, z) in tree {
                map.insert(y, String::from(z));
            }
        }
        Vim { after }
    }
}
