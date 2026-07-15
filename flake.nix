{
  description = "zcash-eta workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    dry-flakes.url = "github:nejucomo/dry-flakes";
  };

  outputs = { self, nixpkgs, flake-utils, dry-flakes }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        checks.default = pkgs.stdenv.mkDerivation {
          pname = "zcash-eta-check";
          version = "0.1.0";
          src = self;
          nativeBuildInputs = [ pkgs.cargo pkgs.rustc ];
          buildPhase = ''
            cargo test --workspace --all-targets
          '';
          installPhase = "mkdir -p $out";
        };
      }) // {
      inherit dry-flakes;
    };
}
