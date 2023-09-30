{ pkgs }: {
  bundler-nvim = {
    package = pkgs.vimUtils.buildVimPlugin {
      pname = "bundler-nvim";
      version = "0.1.0";
      src = ./../bundler-nvim;
    };
  };
}
