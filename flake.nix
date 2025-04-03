{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      utils,
      rust-overlay,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            SDL2
          ];

          LD_LIBRARY_PATH =
            with pkgs;
            lib.makeLibraryPath [
              libGL
              libxkbcommon
              wayland
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
            ];
        };

        devShells.cranelift =
          let
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            };
          in
          pkgs.mkShell {
            packages = with pkgs; [
              ((rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)).override {
                extensions = [ "rustc-codegen-cranelift" ];
              })
            ];

            LD_LIBRARY_PATH =
              with pkgs;
              lib.makeLibraryPath [
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                libxkbcommon
                libGL
              ];
          };
      }
    );
}
