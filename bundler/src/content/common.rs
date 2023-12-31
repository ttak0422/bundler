use std::{convert::From, fmt};

use crate::payload;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    Vim,
    #[default]
    Lua,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Language::Vim => write!(f, "vim"),
            Language::Lua => write!(f, "lua"),
        }
    }
}

impl From<payload::Language> for Language {
    fn from(language: payload::Language) -> Self {
        match language {
            payload::Language::Vim => Self::Vim,
            payload::Language::Lua => Self::Lua,
        }
    }
}
