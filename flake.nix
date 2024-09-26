# vim:ts=2:sts=2:sw=2:et:ft=nix:
{
  inputs.nixpkgs.url = "github:nixos/nixpkgs";
  outputs =
    { self, nixpkgs }:
    let
      withPkgs = f: nixpkgs.lib.mapAttrs (k: v: f v) nixpkgs.legacyPackages;
      config = import ./configuration.nix;
    in
    {
      formatter = withPkgs (pkgs: pkgs.nixfmt-rfc-style);
      packages = withPkgs (pkgs: {
        default = derivation {
          #separator = config.separator;
          name = "foklang";
          inherit (pkgs) system;
          builder = "${pkgs.coreutils}/bin/install";
          args = [
            "-D"
            (derivation {
              name = "foklang-bin";
              inherit (pkgs) system;
              builder = "${pkgs.rustc}/bin/rustc";
              args = [
                "${./.}/shell.rs"
                "-C"
                "linker=${pkgs.gcc}/bin/gcc"
                "-o"
                (builtins.placeholder "out")
              ];
            })
            "${builtins.placeholder "out"}/bin/fokshell"
          ];
        };
      });
    };
}
