{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs = {
    self,
    systems,
    nixpkgs,
    ...
  } @ inputs: let
    inherit (nixpkgs) lib;
    eachSystem = lib.genAttrs (import systems);
    pkgsFor = eachSystem (system:
      import nixpkgs {
        localSystem = system;
      });
  in {
    devShells = eachSystem (system: {
      default = pkgsFor.${system}.mkShell {
        buildInputs = with pkgsFor.${system};
          [
            just
            cargo
            cargo-watch
            rustc
            clippy
            gcc
            rustfmt
            hyperfine
          ]
          ++ lib.optionals (system == "aarch64-darwin") [
            darwin.libiconv
          ];
          
        RUST_SRC_PATH = "${pkgsFor.${system}.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
  };
}
