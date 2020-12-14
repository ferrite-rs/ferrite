{ nixpkgs }:
let
  rustPkgs = nixpkgs.rustBuilder.makePackageSet' {
    rustChannel = "1.46.0";
    packageFun = import ../../ferrite-session/Cargo.nix;
  };
in
rustPkgs.workspace.ferrite-session
