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
| packageNamePrefix | `types.str` | `"bundler-nvim"` | TODO |
| appNamePrefix | `types.str` | `"bundler-nvim"` | TODO |
| package | `types.package` | `pkgs.neovim-unwrapped` | - |
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

