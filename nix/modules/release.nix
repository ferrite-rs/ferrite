{ sources }:
let
  rustOverlay = import "${sources.nixpkgs-mozilla}/rust-overlay.nix";
  cargo2nixOverlay = import "${sources.cargo2nix}/overlay";

  nixpkgs = import sources.nixpkgs {
    overlays = [ cargo2nixOverlay rustOverlay ];
  };

  gitignoreSource = (import sources."gitignore.nix" {
    lib = nixpkgs.lib;
  }).gitignoreSource;

  project-src = gitignoreSource ../..;

  project = import ./project.nix {
    inherit nixpkgs project-src;
  };

  cargo2nix = (import sources.cargo2nix {
    nixpkgs = sources.nixpkgs;
  }).package;

  tools-shell = import ../shell/tools.nix {
    inherit sources nixpkgs cargo2nix;
  };
in
{
  inherit
    sources
    nixpkgs
    project
    tools-shell
  ;
}
