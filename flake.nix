{
  description = "A simple raytracer based on <https://rs118.uwcs.co.uk/raytracer.html>";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
      perSystem = {system, ...}: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        rust-toolchain = pkgs.rust-bin.stable.latest.default;

        naersk = pkgs.callPackage inputs.naersk {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };
      in rec {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [rust-toolchain];
        };

        packages = rec {
          default = raytracer;

          raytracer = naersk.buildPackage {
            src = ./.;
          };
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
