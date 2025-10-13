{
  description = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    neat-flakes.url = "github:wawwior/neat-flakes";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ self, ... }:

    inputs.neat-flakes.lib.eachSystem
      [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ]
      (
        system:
        let

          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
            pkgs: pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)
          );

          src = craneLib.cleanCargoSource ./.;

          commonArgs = {
            inherit src;
            strictDeps = true;

            buildInputs = [ ];
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          crane-package = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );
        in
        {
          checks.${system} = {
            inherit crane-package;

            clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
              }
            );

            doc = craneLib.cargoDoc (
              commonArgs
              // {
                inherit cargoArtifacts;
              }
            );

            fmt = craneLib.cargoFmt {
              inherit src;
            };

            deny = craneLib.cargoDeny {
              inherit src;
            };
          };

          packages.${system} = {
            default = crane-package;
          };

          devShells.${system} = {
            default = craneLib.devShell {
              checks = self.checks.${system};

              packages = [
                pkgs.rust-analyzer
              ];
            };
          };
        }
      );
}
