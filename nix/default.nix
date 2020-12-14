let
  sources = import ./sources.nix {};
in
import ./modules/release.nix {
  inherit sources;
}
