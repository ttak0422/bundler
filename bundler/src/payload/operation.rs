use crate::payload::{bundle, opt};

pub trait Expandable<Out> {
    fn expand_item<'a>(&'a self) -> Vec<&'a Out>;
}

impl Expandable<opt::VimPlugin> for opt::VimPlugin {
    fn expand_item<'a>(&'a self) -> Vec<&'a opt::VimPlugin> {
        match self {
            opt::VimPlugin::SimplePackage(_) => vec![self],
            opt::VimPlugin::ConfiguredPackage(cfg) => [vec![self], expand(&cfg.depends)].concat(),
        }
    }
}

impl Expandable<opt::VimPlugin> for bundle::Bundle {
    fn expand_item<'a>(&'a self) -> Vec<&'a opt::VimPlugin> {
        let plugins = self.plugins.iter().flat_map(|x| x.expand_item());
        let depends = self.depends.iter().flat_map(|x| x.expand_item());
        plugins.chain(depends).collect::<Vec<_>>()
    }
}

pub fn expand<'a, T: Expandable<opt::VimPlugin>>(xs: &'a Vec<T>) -> Vec<&'a opt::VimPlugin> {
    xs.iter().flat_map(|x| x.expand_item()).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{payload::Bundle, payload::VimOptPlugin};

    mod mother {
        use crate::payload::{
            bundle::Bundle,
            core::{Config, DetailConfig, Lang},
            opt::{PluginConfig, VimPlugin},
        };
        use serde_json::Value;

        pub fn opt_simple_package() -> VimPlugin {
            VimPlugin::SimplePackage(String::from("simple-package"))
        }

        pub fn opt_simple_package2() -> VimPlugin {
            VimPlugin::SimplePackage(String::from("simple-package2"))
        }

        pub fn opt_filled_package() -> VimPlugin {
            VimPlugin::ConfiguredPackage(PluginConfig {
                plugin: String::from("filled-plugin"),
                startup: Config::Simple(String::from("filled-startup")),
                pre_config: Config::Simple(String::from("filled-pre-config")),
                config: Config::Simple(String::from("filled-config")),
                depends: vec![opt_simple_package()],
                depend_bundles: vec![String::from("filled-depend-bundle")],
                modules: vec![String::from("filled-module")],
                events: vec![String::from("filled-event")],
                filetypes: vec![String::from("filled-filetype")],
                commands: vec![String::from("filled-command")],
                lazy: true,
            })
        }

        pub fn opt_filled_detail_package() -> VimPlugin {
            VimPlugin::ConfiguredPackage(PluginConfig {
                plugin: String::from("filled-detail-plugin"),
                startup: Config::Detail(DetailConfig {
                    lang: Lang::Vim,
                    code: String::from("filled-detail-startup-code"),
                    args: Value::String(String::from("filled-detail-startup-args")),
                }),
                pre_config: Config::Detail(DetailConfig {
                    lang: Lang::Vim,
                    code: String::from("filled-detail-pre_config-code"),
                    args: Value::String(String::from("filled-detail-pre_config-args")),
                }),
                config: Config::Detail(DetailConfig {
                    lang: Lang::Lua,
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

        pub fn filled_bundle() -> Bundle {
            Bundle {
                name: String::from("simple-bundle"),
                plugins: vec![opt_simple_package2()],
                startup: Config::Simple(String::from("simple-startup")),
                extra_packages: vec![String::from("simple-extra-package")],
                pre_config: Config::Simple(String::from("simple-pre-config")),
                config: Config::Simple(String::from("simple-config")),
                depends: vec![opt_simple_package()],
                depend_bundles: vec![String::from("simple-depend-bundle")],
                modules: vec![String::from("simple-module")],
                events: vec![String::from("simple-event")],
                filetypes: vec![String::from("simple-filetype")],
                commands: vec![String::from("simple-command")],
                lazy: true,
            }
        }

        pub fn filled_detail_bundle() -> Bundle {
            Bundle {
                name: String::from("detail-bundle"),
                plugins: vec![opt_simple_package2()],
                startup: Config::Detail(DetailConfig {
                    lang: Lang::Vim,
                    code: String::from("detail-startup"),
                    args: Value::String(String::from("detail-startup-arg")),
                }),
                extra_packages: vec![String::from("detail-extra-package")],
                pre_config: Config::Detail(DetailConfig {
                    lang: Lang::Vim,
                    code: String::from("detail-pre-config"),
                    args: Value::String(String::from("detail-pre_config-arg")),
                }),
                config: Config::Detail(DetailConfig {
                    lang: Lang::Lua,
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
    fn expand_opt_empty() {
        let arg = Vec::<VimOptPlugin>::new();
        let exp = Vec::<&VimOptPlugin>::new();

        let act = super::expand(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_opt_simple() {
        let arg = vec![mother::opt_simple_package()];
        let exp = vec![mother::opt_simple_package()];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = super::expand(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_opt_filled() {
        let arg = vec![mother::opt_filled_package()];
        let exp = vec![mother::opt_filled_package(), mother::opt_simple_package()];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = super::expand(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_opt_filled_detail() {
        let arg = vec![mother::opt_filled_detail_package()];
        let exp = vec![
            mother::opt_filled_detail_package(),
            mother::opt_filled_package(),
            mother::opt_simple_package(),
        ];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = super::expand(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_bundle_empty() {
        let arg = Vec::<Bundle>::new();
        let exp = Vec::<&VimOptPlugin>::new();

        let act = super::expand(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_bundle_filled_detail() {
        let arg = vec![mother::filled_detail_bundle()];
        let exp = vec![
            mother::opt_simple_package2(),
            mother::opt_filled_detail_package(),
            mother::opt_filled_package(),
            mother::opt_simple_package(),
        ];
        let exp = exp.iter().collect::<Vec<_>>();

        let act = super::expand(&arg);

        assert_eq!(exp, act);
    }
}
