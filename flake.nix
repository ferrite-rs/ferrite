{
  description = "Session types EDSL for Rust";

  inputs = {
    flake-utils.url = github:numtide/flake-utils;
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { nixpkgs, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";

      ferrite = naersk-lib.buildPackage {
        pname = "ferrite-session";
        root = ./.;
      };

      ferrite-demo = naersk-lib.buildPackage {
        pname = "ferrite-demo";
        root = ./.;
      };

      hello-app = {
        type = "app";
        program = ferrite-demo + "/bin/hello";
      };
    in {
      packages = { inherit ferrite; };
      defaultPackage = ferrite;

      apps = {
        hello = hello-app;
      };

      defaultApp = hello-app;

      devShell = pkgs.mkShell {
        nativeBuildInputs = [
          ferrite
          pkgs.rustc
          pkgs.cargo
        ];
      };
    })
  ;
}
