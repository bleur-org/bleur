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
    # Package related things automatically
    # obtained from Cargo.toml, so you don't
    # have to do everything manually
    pname = manifest.name;
    inherit (manifest) version;

    # Your govnocodes
    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;

    # Compile time dependencies
    nativeBuildInputs = with pkgs; [
      # GCC toolchain
      pkg-config

      # Other compile time dependencies
      # here
      openssl
    ];

    # Runtime dependencies which will be shipped
    # with nix package
    buildInputs = with pkgs; [openssl];

    # Set Environment Variables
    RUST_BACKTRACE = 1;
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    # Compiler LD variables
    NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)}";
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.libiconv
    ];

    meta = with lib; {
      homepage = manifest.homepage;
      description = manifest.description;
      license = with lib.licenses; [asl20 mit];
      platforms = with platforms; linux ++ darwin;
      maintainers = with lib.maintainers; [orzklv];
    };
  }
