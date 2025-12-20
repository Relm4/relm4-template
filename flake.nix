{
  description = "nixed gtk-rust-template";

  inputs = {
    # Fresh and new for testing
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    # The flake-utils library
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      # Nix script formatter
      formatter = pkgs.alejandra;

      # Development environment
      devShells.default = import ./shell.nix {inherit pkgs;};

      # Output package
      packages.default = pkgs.callPackage ./. {inherit pkgs;};
    });
}