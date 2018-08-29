let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "hemoglobin";
    buildInputs = [
      # Rust
      nixpkgs.latest.rustChannels.stable.rust
      nixpkgs.latest.rustChannels.stable.cargo

      # Dev tools
      nixpkgs.neovim
      nixpkgs.zsh
      nixpkgs.git
      nixpkgs.openssh
    ] ++ (if system == "x86_64-darwin" then [
      nixpkgs.darwin.apple_sdk.frameworks.Security
    ] else []);
  }
