{ nixpkgs, project-src }:
let
  rustPkgs = nixpkgs.rustBuilder.makePackageSet' {
    rustChannel = "stable";
    packageFun = import ../../Cargo.nix;
    workspaceSrc = project-src;
    localPatterns = [];
  };
in
rustPkgs.workspace.ferrite-session
