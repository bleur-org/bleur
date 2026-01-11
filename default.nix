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
  inherit (pkgs) lib;
  getLibFolder = pkg: "${pkg}/lib";

  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    inherit (manifest) version;

    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;

    # Compile time dependencies
    nativeBuildInputs = with pkgs; [
      pkg-config
      openssl
    ];

    # Runtime dependencies
    buildInputs = with pkgs; [
      openssl
    ];

    # Set Environment Variables
    RUST_BACKTRACE = 1;
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    meta = with lib; {
      homepage = manifest.homepage;
      description = manifest.description;
      license = with lib.licenses; [asl20 mit];
      platforms = with platforms; linux ++ darwin;
      maintainers = with lib.maintainers; [orzklv];
    };
  }
