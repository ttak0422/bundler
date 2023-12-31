use crate::payload::{group, lazy};

pub trait Expandable<Out> {
    fn expand_item<'a>(&'a self) -> Vec<&'a Out>;
}

impl Expandable<lazy::VimPluginPackage> for lazy::VimPluginPackage {
    fn expand_item<'a>(&'a self) -> Vec<&'a lazy::VimPluginPackage> {
        match self {
            lazy::VimPluginPackage::SimplePackage(_) => vec![self],
            lazy::VimPluginPackage::ConfiguredPackage(cfg) => {
                [vec![self], expand(&cfg.depend_plugins)].concat()
            }
        }
    }
}

impl Expandable<lazy::VimPluginPackage> for group::LazyGroup {
    fn expand_item<'a>(&'a self) -> Vec<&'a lazy::VimPluginPackage> {
        let plugins = self.plugins.iter().flat_map(|x| x.expand_item());
        let depends = self.depend_plugins.iter().flat_map(|x| x.expand_item());
        plugins.chain(depends).collect::<Vec<_>>()
    }
}

pub fn expand<'a, T: Expandable<lazy::VimPluginPackage>>(
    xs: &'a Vec<T>,
) -> Vec<&'a lazy::VimPluginPackage> {
    xs.iter().flat_map(|x| x.expand_item()).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::{payload::LazyGroup, payload::VimOptPlugin};

    mod mother {
        use crate::payload::{
            common::Language,
            core::{Config, DetailConfig},
            lazy::{PluginConfig, VimPluginPackage},
            LazyGroup,
        };
        use serde_json::Value;

        pub fn opt_simple_package() -> VimPluginPackage {
            VimPluginPackage::SimplePackage(String::from("simple-package"))
        }

        pub fn opt_simple_package2() -> VimPluginPackage {
            VimPluginPackage::SimplePackage(String::from("simple-package2"))
        }

        pub fn opt_filled_package() -> VimPluginPackage {
            VimPluginPackage::ConfiguredPackage(PluginConfig {
                plugin: String::from("filled-plugin"),
                startup_config: Config::Simple(String::from("filled-startup")),
                pre_config: Config::Simple(String::from("filled-pre-config")),
                post_config: Config::Simple(String::from("filled-config")),
                depend_plugins: vec![opt_simple_package()],
                depend_groups: vec![String::from("filled-depend-bundle")],
                on_modules: vec![String::from("filled-module")],
                on_events: vec![String::from("filled-event")],
                on_filetypes: vec![String::from("filled-filetype")],
                on_commands: vec![String::from("filled-command")],
                use_timer: true,
                use_denops: true,
            })
        }

        pub fn opt_filled_detail_package() -> VimPluginPackage {
            VimPluginPackage::ConfiguredPackage(PluginConfig {
                plugin: String::from("filled-detail-plugin"),
                startup_config: Config::Detail(DetailConfig {
                    language: Language::Vim,
                    code: String::from("filled-detail-startup-code"),
                    args: Value::String(String::from("filled-detail-startup-args")),
                }),
                pre_config: Config::Detail(DetailConfig {
                    language: Language::Vim,
                    code: String::from("filled-detail-pre_config-code"),
                    args: Value::String(String::from("filled-detail-pre_config-args")),
                }),
                post_config: Config::Detail(DetailConfig {
                    language: Language::Lua,
                    code: String::from("filled-detail-config-code"),
                    args: Value::String(String::from("filled-detail-config-args")),
                }),
                depend_plugins: vec![opt_filled_package()],
                depend_groups: vec![String::from("filled-detail-depend-bundle")],
                on_modules: vec![String::from("filled-detail-module")],
                on_events: vec![String::from("filled-detail-event")],
                on_filetypes: vec![String::from("filled-detail-filetype")],
                on_commands: vec![String::from("filled-detail-command")],
                use_timer: false,
                use_denops: true,
            })
        }

        pub fn filled_bundle() -> LazyGroup {
            LazyGroup {
                name: String::from("simple-bundle"),
                plugins: vec![opt_simple_package2()],
                startup_config: Config::Simple(String::from("simple-startup")),
                extra_packages: vec![String::from("simple-extra-package")],
                pre_config: Config::Simple(String::from("simple-pre-config")),
                post_config: Config::Simple(String::from("simple-config")),
                depend_plugins: vec![opt_simple_package()],
                depend_groups: vec![String::from("simple-depend-bundle")],
                on_modules: vec![String::from("simple-module")],
                on_events: vec![String::from("simple-event")],
                on_filetypes: vec![String::from("simple-filetype")],
                on_commands: vec![String::from("simple-command")],
                use_timer: true,
            }
        }

        pub fn filled_detail_bundle() -> LazyGroup {
            LazyGroup {
                name: String::from("detail-bundle"),
                plugins: vec![opt_simple_package2()],
                startup_config: Config::Detail(DetailConfig {
                    language: Language::Vim,
                    code: String::from("detail-startup"),
                    args: Value::String(String::from("detail-startup-arg")),
                }),
                extra_packages: vec![String::from("detail-extra-package")],
                pre_config: Config::Detail(DetailConfig {
                    language: Language::Vim,
                    code: String::from("detail-pre-config"),
                    args: Value::String(String::from("detail-pre_config-arg")),
                }),
                post_config: Config::Detail(DetailConfig {
                    language: Language::Lua,
                    code: String::from("detail-config"),
                    args: Value::String(String::from("detail-config-arg")),
                }),
                depend_plugins: vec![opt_filled_detail_package()],
                depend_groups: vec![String::from("detail-depend-bundle")],
                on_modules: vec![String::from("detail-module")],
                on_events: vec![String::from("detail-event")],
                on_filetypes: vec![String::from("detail-filetype")],
                on_commands: vec![String::from("detail-command")],
                use_timer: false,
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
        let arg = Vec::<LazyGroup>::new();
        let exp = Vec::<&VimOptPlugin>::new();

        let act = super::expand(&arg);

        assert_eq!(exp, act);
    }

    #[test]
    fn expand_bundle_filled() {
        let arg = vec![mother::filled_bundle()];
        let exp = vec![mother::opt_simple_package2(), mother::opt_simple_package()];
        let exp = exp.iter().collect::<Vec<_>>();

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
