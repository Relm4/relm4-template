{
  pkgs ? let
    lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
    nixpkgs = fetchTarball {
      url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
      sha256 = lock.narHash;
    };
  in
    import nixpkgs {overlays = [];},
  ...
}: let
  # Helpful nix function
  getLibFolder = pkg: "${pkg}/lib";

  # Manifest via Cargo.toml
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
      appstream-glib
      desktop-file-utils
      rustPlatform.cargoSetupHook
    ];

    # Runtime dependencies which will be shipped
    # with nix package
    buildInputs = with pkgs; [
      gtk4
      glib
      openssl
      libadwaita
      gdk-pixbuf
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
  }
