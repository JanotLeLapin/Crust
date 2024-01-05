{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    nixpkgs,
    overlay,
    crane,
    ...
  }: let
    eachSystem = fn: nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
    ] (system: (fn (import nixpkgs { inherit system; overlays = [ (import overlay) ]; } )));
  in {
    devShells = eachSystem (pkgs: {
      default = pkgs.mkShell { packages = [ (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) ]; };
    });
    packages = eachSystem (pkgs: let craneLib = crane.lib.${pkgs.system}; in nixpkgs.lib.genAttrs [
      "proxy"
      "server"
    ] (crate: let
      args = {
        src = craneLib.cleanCargoSource (craneLib.path (./. + "/${crate}"));
        strictDeps = true;
        buildInputs = [];
        cargoLock = ./Cargo.lock;
      };
      cargoArtifacts = craneLib.buildDepsOnly args;
    in craneLib.buildPackage (args // { inherit cargoArtifacts; })));
  };
}
