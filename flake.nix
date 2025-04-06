{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            libxkbcommon

            vulkan-loader
            vulkan-headers
            vulkan-tools

            rustup
            cargo-cross
            cmake
          ];
          
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
              pkgs.libxkbcommon
              pkgs.vulkan-loader
            ]}:$LD_LIBRARY_PATH"

            export LIBXKBCOMMON_PATH="${pkgs.libxkbcommon}/lib"
            export LIBXKBCOMMON_FILE_NAME="$(ls ${pkgs.libxkbcommon}/lib/libxkbcommon.so* | head -n 1)"
            export LD_PRELOAD="$LIBXKBCOMMON_FILE_NAME:$LD_PRELOAD"

            export VK_LAYER_PATH="${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d"
          '';
        };
      }
    );
}
