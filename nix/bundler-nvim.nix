{ pkgs }: {
  bundler-nvim = {
    package = pkgs.vimUtils.buildVimPlugin {
      pname = "bundler-nvim";
      version = "1.1.0";
      src = ./../bundler-nvim;
    };
  };
}
