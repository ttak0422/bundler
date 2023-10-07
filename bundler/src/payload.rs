use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct PluginPackage {
    pub id: String,
    pub package: String,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ConfigLang {
    Vim,
    #[default]
    Lua,
    Fennel,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct PluginConfigDetail {
    pub lang: ConfigLang,
    pub code: String,
    pub args: Value,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum PluginConfig {
    Line(String),
    Detail(PluginConfigDetail),
}

impl Default for PluginConfig {
    fn default() -> Self {
        Value::default();
        PluginConfig::Line(String::from(""))
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum StartVimPlugin {
    Package(String),
    StartPlugin(StartPluginConfig),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum OptVimPlugin {
    Package(String),
    OptPlugin(OptPluginConfig),
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct StartPluginConfig {
    pub plugin: String,
    pub startup: PluginConfig,
    pub extra_packages: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct OptPluginConfig {
    pub plugin: String,
    pub startup: PluginConfig,
    pub pre_config: PluginConfig,
    pub config: PluginConfig,
    pub depends: Vec<OptVimPlugin>,
    pub depend_bundles: Vec<String>,
    pub modules: Vec<String>,
    pub events: Vec<String>,
    pub filetypes: Vec<String>,
    pub commands: Vec<String>,
    pub lazy: bool,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct BundleConfig {
    pub name: String,
    pub plugins: Vec<OptVimPlugin>,
    pub startup: PluginConfig,
    pub extra_packages: Vec<String>,
    pub pre_config: PluginConfig,
    pub config: PluginConfig,
    pub depends: Vec<OptVimPlugin>,
    pub depend_bundles: Vec<String>,
    pub modules: Vec<String>,
    pub events: Vec<String>,
    pub filetypes: Vec<String>,
    pub commands: Vec<String>,
    pub lazy: bool,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct BundlerConfig {
    pub start_plugins: Vec<StartVimPlugin>,
    pub opt_plugins: Vec<OptVimPlugin>,
    pub bundles: Vec<BundleConfig>,
    pub package: String,
    pub with_node_js: bool,
    pub with_python3: bool,
    pub with_ruby: bool,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub extra_packages: Vec<String>,
    pub id_map: Vec<PluginPackage>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub cfg: BundlerConfig,
    pub meta: Meta,
}

fn expand_opt_plugins<'a>(ps: &'a Vec<OptVimPlugin>) -> Vec<&'a OptVimPlugin> {
    ps.iter()
        .flat_map(|p| match p {
            OptVimPlugin::Package(_) => vec![p],
            OptVimPlugin::OptPlugin(cfg) => [vec![p], expand_opt_plugins(&cfg.depends)].concat(),
        })
        .collect::<Vec<_>>()
}

fn expand_bundles<'a>(bs: &'a Vec<BundleConfig>) -> Vec<&'a OptVimPlugin> {
    let bundle_plugins = bs.iter().flat_map(|b| &b.plugins).collect::<Vec<_>>();

    let bundle_depends = bs
        .iter()
        .flat_map(|b| &b.depends)
        .flat_map(|p| match p {
            OptVimPlugin::Package(_) => vec![p],
            OptVimPlugin::OptPlugin(cfg) => {
                let ds = expand_opt_plugins(&cfg.depends);
                [vec![p], ds].concat()
            }
        })
        .collect::<Vec<_>>();
    [bundle_plugins, bundle_depends].concat()
}

/// expand all nested plugins.
pub fn expand_all_opt_plugins<'a>(
    ps: &'a Vec<OptVimPlugin>,
    bs: &'a Vec<BundleConfig>,
) -> Vec<&'a OptVimPlugin> {
    let ps = expand_opt_plugins(ps);
    let bs = expand_bundles(bs);
    [ps, bs].concat()
}
