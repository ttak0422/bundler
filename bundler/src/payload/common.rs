use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Language {
    Vim,
    #[default]
    Lua,
}
