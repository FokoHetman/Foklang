# vim:ts=2:sts=2:sw=2:et:ft=nix:
{
  inputs.nixpkgs.url = "github:nixos/nixpkgs";
  outputs =
    { self, nixpkgs }:
    let
      withPkgs = f: nixpkgs.lib.mapAttrs (k: v: f v) nixpkgs.legacyPackages;
    in
    {
      formatter = withPkgs (pkgs: pkgs.nixfmt-rfc-style);
      packages = withPkgs (pkgs: {
        default = derivation {
          builder = pkgs.moreutils + /bin/install;
          args = [
            (derivation {
              name = "foklang";
              inherit (pkgs) system;
              nativeBuildInputs = [
                pkgs.coreutils
                pkgs.mktemp
              ];
              builder = pkgs.rustc + /bin/rustc;
              args = [
                "${./.}/shell.rs"
                "-C"
                "linker=${pkgs.clang}/bin/clang"
                "-o"
                (builtins.placeholder "out")
              ];
            })
            (builtins.placeholder "out" + /bin/foklang)
          ];
        };
      });
    };
}
