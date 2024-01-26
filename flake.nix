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
      pkgs = nixpkgs.legacyPackages.${system};

      craneLib = crane.lib.${system};
      fgc_2024 = craneLib.buildPackage {
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        strictDeps = true;
        cargoExtraArgs = "--no-default-features";

        nativeBuildInputs = [pkgs.pkg-config];
        buildInputs = with pkgs; [
          vulkan-loader
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          alsa-lib
          udev
        ];

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

      devShells.default = let
        deps = with pkgs; [
          vulkan-loader
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          alsa-lib
          udev
          libxkbcommon
          wayland
        ];
      in
        devenv.lib.mkShell {
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

              # gona
              env.LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath deps}";
            })
          ];
        };
    });
}
