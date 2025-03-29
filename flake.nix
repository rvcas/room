{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      treefmt-nix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        inherit (pkgs) mkShell;

        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        buildTarget = "wasm32-wasip1";

        rustPlatform = pkgs.makeRustPlatform {
          rustc = rust;
          cargo = rust;
        };

        room-formatter =
          (treefmt-nix.lib.evalModule pkgs {
            projectRootFile = "flake.nix";

            settings = {
              allow-missing-formatter = true;
              verbose = 0;

              global.excludes = [ "*.lock" ];

              formatter = {
                nixfmt.options = [ "--strict" ];
                rustfmt.package = rust;
              };
            };

            programs = {
              nixfmt.enable = true;
              rustfmt = {
                enable = true;
                package = rust;
              };
              taplo.enable = true;
            };
          }).config.build.wrapper;

        packages.default = rustPlatform.buildRustPackage rec {
          name = "room";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [ perl ];

          buildPhase = ''
            cargo build --release --target=${buildTarget}
          '';

          installPhase = ''
            mkdir -p $out/lib/zellij/plugins
            cp target/${buildTarget}/release/${name}.wasm $out/lib/zellij/plugins
          '';
        };
      in
      {
        inherit packages;

        devShells.default = mkShell {
          name = "room";

          buildInputs = [
            rust
            room-formatter
          ];
        };

        formatter = room-formatter;
        nixosModules.default = import ./. { inherit (packages) default; };
      }
    );
}
