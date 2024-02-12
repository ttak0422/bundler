{ pkgs }: {
  bundler-vim = {
    package = pkgs.vimUtils.buildVimPlugin {
      pname = "bundler-vim";
      version = "2.1.0";
      src = ./../bundler-vim;
    };
  };
}
