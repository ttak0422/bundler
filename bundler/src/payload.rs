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

#[cfg(test)]
mod tests {
    use super::*;

    mod mother {
        use super::*;

        pub fn opt_simple_package() -> OptVimPlugin {
            OptVimPlugin::Package(String::from("simple-package"))
        }

        pub fn opt_simple_package2() -> OptVimPlugin {
            OptVimPlugin::Package(String::from("simple-package2"))
        }

        pub fn opt_filled_package() -> OptVimPlugin {
            OptVimPlugin::OptPlugin(OptPluginConfig {
                plugin: String::from("filled-plugin"),
                startup: PluginConfig::Line(String::from("filled-startup")),
                pre_config: PluginConfig::Line(String::from("filled-pre-config")),
                config: PluginConfig::Line(String::from("filled-config")),
                depends: vec![opt_simple_package()],
                depend_bundles: vec![String::from("filled-depend-bundle")],
                modules: vec![String::from("filled-module")],
                events: vec![String::from("filled-event")],
                filetypes: vec![String::from("filled-filetype")],
                commands: vec![String::from("filled-command")],
                lazy: true,
            })
        }

        pub fn opt_filled_detail_package() -> OptVimPlugin {
            OptVimPlugin::OptPlugin(OptPluginConfig {
                plugin: String::from("filled-detail-plugin"),
                startup: PluginConfig::Detail(PluginConfigDetail {
                    lang: ConfigLang::Vim,
                    code: String::from("filled-detail-startup-code"),
                    args: Value::String(String::from("filled-detail-startup-args")),
                }),
                pre_config: PluginConfig::Detail(PluginConfigDetail {
                    lang: ConfigLang::Vim,
                    code: String::from("filled-detail-pre_config-code"),
                    args: Value::String(String::from("filled-detail-pre_config-args")),
                }),
                config: PluginConfig::Detail(PluginConfigDetail {
                    lang: ConfigLang::Lua,
                    code: String::from("filled-detail-config-code"),
                    args: Value::String(String::from("filled-detail-config-args")),
                }),
                depends: vec![opt_filled_package()],
                depend_bundles: vec![String::from("filled-detail-depend-bundle")],
                modules: vec![String::from("filled-detail-module")],
                events: vec![String::from("filled-detail-event")],
                filetypes: vec![String::from("filled-detail-filetype")],
                commands: vec![String::from("filled-detail-command")],
                lazy: false,
            })
        }

        pub fn filled_bundle() -> BundleConfig {
            BundleConfig {
                name: String::from("simple-bundle"),
                plugins: vec![opt_simple_package2()],
                startup: PluginConfig::Line(String::from("simple-startup")),
                extra_packages: vec![String::from("simple-extra-package")],
                pre_config: PluginConfig::Line(String::from("simple-pre-config")),
                config: PluginConfig::Line(String::from("simple-config")),
                depends: vec![opt_simple_package()],
                depend_bundles: vec![String::from("simple-depend-bundle")],
                modules: vec![String::from("simple-module")],
                events: vec![String::from("simple-event")],
                filetypes: vec![String::from("simple-filetype")],
                commands: vec![String::from("simple-command")],
                lazy: true,
            }
        }

        pub fn filled_detail_bundle() -> BundleConfig {
            BundleConfig {
                name: String::from("detail-bundle"),
                plugins: vec![opt_simple_package2()],
                startup: PluginConfig::Detail(PluginConfigDetail {
                    lang: ConfigLang::Vim,
                    code: String::from("detail-startup"),
                    args: Value::String(String::from("detail-startup-arg")),
                }),
                extra_packages: vec![String::from("detail-extra-package")],
                pre_config: PluginConfig::Detail(PluginConfigDetail {
                    lang: ConfigLang::Vim,
                    code: String::from("detail-pre-config"),
                    args: Value::String(String::from("detail-pre_config-arg")),
                }),
                config: PluginConfig::Detail(PluginConfigDetail {
                    lang: ConfigLang::Lua,
                    code: String::from("detail-config"),
                    args: Value::String(String::from("detail-config-arg")),
                }),
                depends: vec![opt_filled_detail_package()],
                depend_bundles: vec![String::from("detail-depend-bundle")],
                modules: vec![String::from("detail-module")],
                events: vec![String::from("detail-event")],
                filetypes: vec![String::from("detail-filetype")],
                commands: vec![String::from("detail-command")],
                lazy: false,
            }
        }
    }

    #[test]
    fn expand_empty_plugins() {
        let arg = vec![];
        let exp: Vec<&OptVimPlugin> = vec![];

        let act = expand_opt_plugins(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_simple_plugins() {
        let arg = vec![mother::opt_simple_package()];
        let exp = vec![mother::opt_simple_package()];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = expand_opt_plugins(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_filled_plugins() {
        let arg = vec![mother::opt_filled_package()];
        let exp = vec![mother::opt_filled_package(), mother::opt_simple_package()];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = expand_opt_plugins(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_filled_detail_plugins() {
        let arg = vec![mother::opt_filled_detail_package()];
        let exp = vec![
            mother::opt_filled_detail_package(),
            mother::opt_filled_package(),
            mother::opt_simple_package(),
        ];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = expand_opt_plugins(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_dup_plugins() {
        let arg = vec![mother::opt_simple_package(), mother::opt_simple_package()];
        let exp = vec![mother::opt_simple_package(), mother::opt_simple_package()];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = expand_opt_plugins(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_empty_bundles() {
        let arg = vec![];
        let exp: Vec<&OptVimPlugin> = vec![];

        let act = expand_bundles(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_filled_bundles() {
        let arg = vec![mother::filled_bundle()];
        let exp = vec![mother::opt_simple_package2(), mother::opt_simple_package()];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = expand_bundles(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_filled_detail_bundles() {
        let arg = vec![mother::filled_detail_bundle()];
        let exp = vec![
            mother::opt_simple_package2(),
            mother::opt_filled_detail_package(),
            mother::opt_filled_package(),
            mother::opt_simple_package(),
        ];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = expand_bundles(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_dup_bundles() {
        let arg = vec![mother::filled_bundle(), mother::filled_bundle()];
        let exp = vec![
            mother::opt_simple_package2(),
            mother::opt_simple_package2(),
            mother::opt_simple_package(),
            mother::opt_simple_package(),
        ];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = expand_bundles(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_all_empty() {
        let arg_ps = vec![];
        let arg_bs = vec![];
        let exp: Vec<&OptVimPlugin> = vec![];

        let act = expand_all_opt_plugins(&arg_ps, &arg_bs);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_all_filled() {
        let arg_ps = vec![mother::opt_filled_package()];
        let arg_bs = vec![mother::filled_bundle()];
        let exp = vec![
            mother::opt_filled_package(),
            mother::opt_simple_package(),
            mother::opt_simple_package2(),
            mother::opt_simple_package(),
        ];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = expand_all_opt_plugins(&arg_ps, &arg_bs);

        assert_eq!(exp, act);
    }
}
