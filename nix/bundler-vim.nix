{ pkgs }: {
  bundler-vim = {
    package = pkgs.vimUtils.buildVimPlugin {
      pname = "bundler-vim";
      version = "2.0.0";
      src = ./../bundler-vim;
    };
  };
}