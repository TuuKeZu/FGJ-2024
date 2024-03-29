{
  description = "FGC-2024 flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    devenv.url = "github:cachix/devenv";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    devenv,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      inherit (pkgs.lib) hasInfix hasSuffix;

      pkgs = nixpkgs.legacyPackages.${system};
      craneLib = crane.lib.${system};

      buildInputs = with pkgs; [
        vulkan-loader
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        alsa-lib
        udev
      ];

      fgc_2024 = craneLib.buildPackage {
        inherit buildInputs;
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (
              hasInfix "/assets/" path
              && (
                hasSuffix ".png" path
                || hasSuffix ".ttf" path
                || hasSuffix ".json" path
                || hasSuffix ".csv" path
              )
            )
            || (craneLib.filterCargoSources path type);
        };

        doNotRemoveReferencesToVendorDir = true;
        strictDeps = true;
        cargoExtraArgs = "--no-default-features";
        installPhaseCommand = ''
          mkdir -p $out/bin
          cp -r assets $out/bin
          installFromCargoBuildLog "$out" "$cargoBuildLog"
        '';

        nativeBuildInputs = [pkgs.pkg-config];
        # Additional environment variables can be set directly
        # MY_CUSTOM_VAR = "some value";
      };

      # Use with `watch_tool nix run -L`
      watch_tool = pkgs.writeShellScriptBin "watch_tool" ''
        sigint_handler()
        {
          kill $PID
          exit
        }

        trap sigint_handler SIGINT

        while true; do
          $@ &
          PID=$!
          ${pkgs.inotify-tools}/bin/inotifywait -q -e modify -e create -e close_write -r src
          kill $PID
        done
      '';
    in {
      checks = {};
      packages.default = fgc_2024;

      apps.default = flake-utils.lib.mkApp {
        drv = fgc_2024;
      };

      devShells.default = devenv.lib.mkShell {
        # Inherit inputs from checks.
        inherit inputs pkgs;

        # Extra inputs can be added here; cargo and rustc are provided by default.
        modules = [
          ({
            pkgs,
            config,
            ...
          }: {
            packages = with pkgs; [
              pre-commit
              trunk
              watch_tool
            ];

            # gona
            env.LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";

            # FIXME
            pre-commit.default_stages = ["manual"];
            # current workaround:
            # 1. remove .pre-commit-config.yaml but don't stage the deletion
            # 2. run `pre-commit run --all-files --hook-stage manual` before every commit

            pre-commit.hooks = {
              # TODO cargo check
              rustfmt.enable = true;
              alejandra.enable = true;
            };

            scripts.cargo-watch.exec = ''
              RED='\033[0;31m'
              GREEN='\033[0;32m'
              BLUE='\033[0;34m'
              NC='\033[0m'

              build () {
                if nix build .# -L; then
                  echo -e "''${GREEN}Build successful''${NC}";
                else
                  echo -e "''${RED}Build failed''${NC}";
                fi
              }

              build
              echo -e "''${BLUE}Watching for changes''${NC}"

              ${pkgs.inotify-tools}/bin/inotifywait -q -e close_write,moved_to,create -r -m ./src |
              while read -r directory events filename; do
                build
              done
            '';
          })
        ];
      };
    });
}
