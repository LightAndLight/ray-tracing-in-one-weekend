{
  description = "Ray tracing in one weekend";
  inputs = {
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    cargo2nix = {
      url = "github:cargo2nix/cargo2nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        rust-overlay.follows = "rust-overlay";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, cargo2nix, rust-overlay }: 
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let 
        pkgs = import nixpkgs { 
          inherit system; 
          overlays = [ 
            (import "${cargo2nix}/overlay")
            rust-overlay.overlay 
          ]; 
        };
        
        rustPkgs = { release }: pkgs.rustBuilder.makePackageSet' {
          rustChannel = "1.56.1";
          packageFun = import "${self}/Cargo.nix";
          inherit release;
        };
      in rec {
        packages = {
          ray-tracing-in-one-weekend = (rustPkgs { release = true; }).workspace.ray-tracing-in-one-weekend {};
        };
        defaultPackage = packages.ray-tracing-in-one-weekend;
        devShell =
          pkgs.mkShell {
            buildInputs = [
              cargo2nix.defaultPackage.${system}
              (pkgs.rust-bin.nightly.latest.default.override {
                extensions = [
                  "cargo"
                  "clippy"
                  "rustc"
                  "rust-src"
                  "rustfmt"
                ];
              })
            ];
          };
      }
    );
}
