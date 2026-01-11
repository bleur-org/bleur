flake: {pkgs, ...}: let
  system = pkgs.hostPlatform.system;
  base = flake.packages.${system}.default;
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.mkShell {
    name = "${manifest.name}-dev";

    inputsFrom = [base];

    nativeBuildInputs = with pkgs; [
      nixd
      statix
      deadnix
      alejandra

      rustfmt
      clippy
      rust-analyzer
    ];

    RUST_BACKTRACE = "full";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    shellHook = ''
      rm -rf ./test && mkdir -p ./test
    '';
  }
