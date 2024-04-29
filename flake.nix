{
  description = "A simple raytracer based on <https://rs118.uwcs.co.uk/raytracer.html>";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-parts.url = "github:hercules-ci/flake-parts";

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

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
      imports = [
        inputs.pre-commit-hooks.flakeModule
      ];

      systems = ["x86_64-linux" "aarch64-linux"];
      perSystem = {
        config,
        system,
        ...
      }: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        rust-toolchain = pkgs.rust-bin.stable.latest.default;

        buildInputs = with pkgs; [
          libxkbcommon
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          wayland
        ];

        naersk = pkgs.callPackage inputs.naersk {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };
      in rec {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs =
            [
              (rust-toolchain.override {
                extensions = ["rust-analyzer" "rust-src" "rust-std"];
              })
            ]
            ++ buildInputs;
          shellHook = ''
            ${config.pre-commit.installationScript}
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath buildInputs}"
          '';
        };

        # See https://flake.parts/options/pre-commit-hooks-nix and
        # https://github.com/cachix/git-hooks.nix/blob/master/modules/hooks.nix
        # for all the available hooks and options
        pre-commit.settings.hooks = {
          check-added-large-files.enable = true;
          check-merge-conflicts.enable = true;
          check-toml.enable = true;
          check-vcs-permalinks.enable = true;
          check-yaml.enable = true;
          end-of-file-fixer.enable = true;
          trim-trailing-whitespace.enable = true;

          rustfmt = {
            enable = true;
            packageOverrides = {
              cargo = rust-toolchain;
              rustfmt = rust-toolchain;
            };
          };
        };

        packages = rec {
          default = raytracer;

          raytracer = naersk.buildPackage {
            src = ./.;
            inherit buildInputs;
          };
        };
      };
    };
}
