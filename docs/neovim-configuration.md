# Neovim Configuration

Configurations defined in bundler-nvim are available as packages and apps.
This module assumes that you will use that package as you wish, as follows.

```nix
environment.systemPackages = [
  (pkgs.runCommand "nvim" { } ''
    mkdir -p $out/bin
    ln -s ${
      self.packages.${system}.bundler-nvim
    }/bin/nvim $out/bin/nvim
  '')
];
```

## flakeModule schemes

| name | type | default | description |
| :-: | :-: | :- | :- |
| packageNamePrefix | `types.str` | `"bundler-nvim"` | package name prefix provided by this module |
| appNamePrefix | `types.str` | `"bundler-nvim"` | app name prefix provided by this module |
| package | `types.package` | `pkgs.neovim-unwrapped` | neovim package |
| extraPackages | `with types; listOf package` | `[]` | e.g. lua-language-server |
| extraConfig | `types.lines` | `""` | viml code executed at startup |
| extraLuaConfig | `types.lines` | `""` | lua code executed at startup |
| after.ftPlugin | `with types; attrsOf lines` | `{}` | `after/ftplugin` (viml only) |
| withNodeJs | `types.bool` | `false` | alias for `neovim.withNodeJs` |
| withPython3 | `types.bool` | `false` | alias for `neovim.withPython3` |
| withRuby | `types.bool` | `false` | alias for `neovim.withRuby` |
| eagerPlugins | `with types; listOf (either package eagerPluginConfig)` | `[]` | plugins loaded at startup |
| lazyPlugins | `with types; listOf (either package lazyPluginConfig)` | `[]` | plugins lazy loaded |
| lazyGroups | `types.listOf lazyGroupConfig` | `[]` | plugin groups lazy loaded |
| timer | `types.int` | `100` | time used for loading plugin (msec) |
| logLevel | `types.enum [ "debug" "info" "warn" "error" ]` | `"warn"` |

### pluginConfigDetail

| name | type | default | description |
| :-: | :-: | :-: | :- |
| language | `types.enum [ "vim" "lua" ]` | `vim` | - |
| code | `types.lines` | `""` | setup code |
| args | `types.attrs` | `{}` | bring the values of nix into the code . see [example](./neovim-configuration-example-args.md). |

### eagerPluginConfig

| name | type | default | description |
| :-: | :-: | :-: | :- |
| startupConfig | `with types; either lines pluginConfigDetail` | `""` | setup code executed at startup | 
| extraPackages | `with types; listOf package` | `[]` | nix packages |

### lazyPluginConfig

| name | type | default | description |
| :-: | :-: | :-: | :- |
| startupConfig | `with types; either lines pluginConfigDetail` | `""` | setup code executed at startup |
| extraPackages | `with types; listOf package` | `[]` | nix packages |
| preConfig | `with types; either lines pluginConfigDetail` | `""` | setup code executed before load plugin |
| postConfig | `with types; either lines pluginConfigDetail` | `""` | setup code executed after load plugin |
| dependPlugins | `with types; listOf (either package lazyPluginConfig)` | `[]` | plugins on which this plugin depends |
| dependGroups | `with types; listOf str` | `[]` | groups on which this plugin depends |
| onModules | `with types; listOf str` | `[]` | |
| onEvents | `with types; listOf str` | `[]` | |
| onFiletypes | `with types; listOf str` | `[]` | |
| onCommands | `with types; listOf str` | `[]` | |
| useTimer | `types.bool` | `false` | |
| useDenops | `types.bool` | `false` | |

### lazyGroupConfig

| name | type | default | description |
| :-: | :-: | :-: | :- |
| name | `types.str` | **required** | group name |
| plugins | `with types; listOf` | **required** | group name |
| startupConfig | `with types; either lines pluginConfigDetail` | `""` | setup code executed at startup |
| extraPackages | `with types; listOf package` | `[]` | nix packages |
| preConfig | `with types; either lines pluginConfigDetail` | `""` | setup code executed before load plugin |
| postConfig | `with types; either lines pluginConfigDetail` | `""` | setup code executed after load plugin |
| dependPlugins | `with types; listOf (either package lazyPluginConfig)` | `[]` | plugins on which this plugin depends |
| dependGroups | `with types; listOf str` | `[]` | groups on which this plugin depends |
| onModules | `with types; listOf str` | `[]` | |
| onEvents | `with types; listOf str` | `[]` | |
| onFiletypes | `with types; listOf str` | `[]` | |
| onCommands | `with types; listOf str` | `[]` | |
| useTimer | `types.bool` | `false` | |

