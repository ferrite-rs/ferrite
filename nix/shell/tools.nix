{ sources
, nixpkgs
, cargo2nix
}:
let
  niv = (import sources.niv {
    # inherit nixpkgs;
  }).niv;

in
nixpkgs.mkShell {
  buildInputs = [
    # niv
    cargo2nix
  ];
}
