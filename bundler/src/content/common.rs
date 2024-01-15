use crate::payload;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Target {
    Vim,
    Neovim,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    #[default]
    Vim,
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

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Target::Vim => write!(f, "vim"),
            Target::Neovim => write!(f, "neovim"),
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

impl From<payload::Target> for Target {
    fn from(value: payload::Target) -> Self {
        match value {
            payload::Target::Vim => Self::Vim,
            payload::Target::Neovim => Self::Neovim,
        }
    }
}
