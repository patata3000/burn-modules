{
  description = "Rust shell";

  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rustaceanvim = {
      url = "github:mrcjkb/rustaceanvim";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    cargo2nix = {
      url = "github:cargo2nix/cargo2nix/release-0.11.0";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

    # flake-utils.lib.eachDefaultSystem (system:
    #   let
    #     pkgs = import nixpkgs {
    #       inherit system;
    #       overlays = [cargo2nix.overlays.default];
    #     };
    #
    #     rustPkgs = pkgs.rustBuilder.makePackageSet {
    #       rustVersion = "1.81.0";
    #       packageFun = import ./Cargo.nix;
    #     };
    #
    #   in rec {
    #     packages = {
    #       # replace hello-world with your package name
    #       hello-world = (rustPkgs.workspace.generic_mod {});
    #       default = packages.hello-world;
    #     };
    #   }
    # );
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    # rust-overlay,
    fenix,
    rustaceanvim,
    cargo2nix,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        # see https://github.com/nix-community/poetry2nix/tree/master#api for more functions and examples.
        # overlays = [(import rust-overlay)];
        overlays = [
          fenix.overlays.default
          rustaceanvim.overlays.default
          cargo2nix.overlays.default
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
        };
        workspaceShell = rustPkgs.workspaceShell {
          packages = [
            cargo2nix.packages."${system}".cargo2nix
            (pkgs.fenix.beta.withComponents [
              # stable/beta/latest
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            pkgs.rustaceanvim
            pkgs.rust-analyzer-nightly
            pkgs.vulkan-loader
            pkgs.openssl
          ];

          buildInputs = [
          ];
          shellHook = ''
            export VIRTUAL_ENV=gen-mods
            export LD_LIBRARY_PATH="${pkgs.vulkan-loader}/lib"
          '';
        };
      in rec {
        devShells = {
          default = workspaceShell;
        };
        packages = {
          burn-modules = rustPkgs.workspace.burn-modules {};
          default = packages.burn-modules;
        };
      }
    );
}
