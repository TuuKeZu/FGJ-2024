{
  description = "FGC-2024 flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
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
    in {
      checks = {};
      packages.default = fgc_2024;

      apps.default = flake-utils.lib.mkApp {
        drv = fgc_2024;
      };

      devShells.default = craneLib.devShell {
        # Inherit inputs from checks.
        checks = self.checks.${system};

        # Extra inputs can be added here; cargo and rustc are provided by default.
        packages = with pkgs; [
          pre-commit
          trunk
          cargo-watch
        ];

        # gona
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
          vulkan-loader
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          alsa-lib
          udev
          libxkbcommon
          wayland
        ]);
      };
    });
}
