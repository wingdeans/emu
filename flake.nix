{
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  outputs = { self, nixpkgs, rust-overlay }:
    let
      forAllSystems = f: nixpkgs.lib.genAttrs
        [ "x86_64-linux" ]
        (system: f (import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        }));
    in {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
          ];
        };
      });
    };
}
