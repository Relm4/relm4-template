{
  description = "nixed gtk-rust-template";

  inputs = {
    # Fresh and new for testing
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        # Nix script formatter
        formatter = pkgs.nixfmt-tree;

        # Output package
        packages.default =
          let
            getLibFolder = pkg: "${pkg}/lib";
            manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
          in
          pkgs.stdenv.mkDerivation {
            pname = manifest.name;
            version = manifest.version;

            src = pkgs.lib.cleanSource ./.;

            cargoDeps = pkgs.rustPlatform.importCargoLock {
              lockFile = ./Cargo.lock;
              # Use this if you have dependencies from git instead
              # of crates.io in your Cargo.toml
              # outputHashes = {
              #   # Sha256 of the git repository, doesn't matter if it's monorepo
              #   "example-0.1.0" = "sha256-80EwvwMPY+rYyti8DMG4hGEpz/8Pya5TGjsbOBF0P0c=";
              # };
            };

            # Compile time dependencies
            nativeBuildInputs = with pkgs; [
              git
              rustc
              cargo
              ninja
              meson
              clippy
              gettext
              pkg-config
              rust-analyzer
              wrapGAppsHook4
              appstream
              desktop-file-utils
              rustPlatform.cargoSetupHook
            ];

            # Runtime dependencies which will be shipped
            # with nix package
            buildInputs = with pkgs; [
              gtk4
              openssl
              libadwaita
              gnome-desktop
              adwaita-icon-theme
              desktop-file-utils
              rustPlatform.bindgenHook
            ];

            # Compiler LD variables
            NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)}";
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.gcc
              pkgs.libiconv
              pkgs.llvmPackages.llvm
            ];
          };

        devShells.default = pkgs.mkShell {

          # Compile time dependencies
          packages = with pkgs; [
            # Hail the Nix
            nixd
            statix
            deadnix
            nixfmt

            # Rust
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
            cargo-watch

            openssl
            # Gnome related
            gtk4
            meson
            ninja
            gettext
            appstream
            pkg-config
            libadwaita
            gnome-desktop
            wrapGAppsHook4
            desktop-file-utils
            rustPlatform.bindgenHook
          ];

          # Set Environment Variables
          RUST_BACKTRACE = "full";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
