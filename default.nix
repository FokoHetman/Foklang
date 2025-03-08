{ pkgs ? import <nixpkgs> { }, ... }:
let
  mf = (pkgs.lib.importTOML ./Cargo.toml).package;
  config = import ./configuration.nix;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = mf.name;
    version = mf.version;
    src = pkgs.lib.cleanSource ./.;

    cargoLock.lockFile = ./Cargo.lock;
    
    cargoHash = pkgs.lib.fakeHash;
    #cargoSha256 = nixpkgs.lib.fakeSha256;
}
