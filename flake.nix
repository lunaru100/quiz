{
  description = "quiz";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          devShells.default = pkgs.mkShell {
            LD_LIBRARY_PATH="${
                pkgs.lib.makeLibraryPath
                (with pkgs; [ stdenv.cc.cc openssl pkg-config ])
            }:$LD_LIBRARY_PATH";
            packages = with pkgs; [
              nodejs_20
              typescript
              sqlite-interactive
              sqlcipher
              openssl.dev
              pkg-config
            ];
          };
        }
      );
}
