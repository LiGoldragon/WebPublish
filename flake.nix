{
  description = "Mycelia Spore Rust daemon";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.lib.${system};
        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          pname = "mycelia-spore";
          version = "0.1.0";
          inherit src;
          nativeBuildInputs = [ pkgs.capnproto ];
        };

        myceliaSpore = craneLib.buildPackage commonArgs;
      in {
        packages = {
          default = myceliaSpore;
          mycelia-spore = myceliaSpore;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = myceliaSpore;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.capnproto
            pkgs.cargo
            pkgs.rustc
            pkgs.rust-analyzer
          ];
          shellHook = ''
            export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
          '';
        };
      });
}
