{ pkgs }: {
  bundler-vim = {
    package = pkgs.vimUtils.buildVimPlugin {
      pname = "bundler-vim";
      version = "2.2.1";
      src = ./../bundler-vim;
    };
  };
}
