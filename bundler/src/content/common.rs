use crate::payload;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    Vim,
    #[default]
    Lua,
}

impl From<payload::common::Language> for Language {
    fn from(language: payload::common::Language) -> Self {
        match language {
            payload::common::Language::Vim => Self::Vim,
            payload::common::Language::Lua => Self::Lua,
        }
    }
}

impl From<payload::common::Config> for String {
    fn from(value: payload::common::Config) -> Self {
        match value {
            payload::common::Config::Simple(code) => code,
            payload::common::Config::Detail(cfg) => match cfg.language {
                payload::common::Language::Vim => {
                    let mut statements = vec![];
                    let args = serde_json::to_string(&cfg.args).unwrap();
                    if args != "{}" {
                        statements.push(format!("let s:args = json_decode('{}')", args));
                    }
                    if !cfg.code.is_empty() {
                        statements.push(cfg.code);
                    }
                    format!("vim.cmd([=[\n{}\n]=])", statements.join("\n"))
                }
                payload::common::Language::Lua => {
                    let mut statements = vec![];
                    let args = serde_json::to_string(&cfg.args).unwrap();
                    if args != "{}" {
                        statements.push(format!("local args = vim.json.decode([[{}]])", args));
                    }
                    if !cfg.code.is_empty() {
                        statements.push(cfg.code);
                    }
                    statements.join("\n")
                }
            },
        }
    }
}
