{
  description = "A simple raytracer based on <https://rs118.uwcs.co.uk/raytracer.html>";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-parts.url = "github:hercules-ci/flake-parts";

    cargo2nix = {
      url = "github:cargo2nix/cargo2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
      perSystem = {system, ...}: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.cargo2nix.overlays.default];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageFun = import ./Cargo.nix;
          rustVersion = "1.75.0";
          packageOverrides = pkgs: pkgs.rustBuilder.overrides.all;
        };
      in rec {
        devShells.default = rustPkgs.workspaceShell {
          nativeBuildInputs = [inputs.cargo2nix.packages.${system}.default];
        };

        packages = rec {
          default = raytracer;

          raytracer = rustPkgs.workspace.raytracer {};
        };

        apps = rec {
          default = raytracer;

          raytracer = {
            type = "app";
            program = "${packages.raytracer}/bin/raytracer";
          };
        };
      };
    };
}
