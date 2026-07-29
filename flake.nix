{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "comd";
          version = "0.1.0";
          src = ./.;

          cargoHash = "sha256-LHHFMQyEkbqZVTWo6rFXJSgTBTkCHKfqM+Uz9wiwcOo=";

          meta = {
            description = "A TUI command-line application";
            mainProgram = "comd";
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.default ];

          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
          ];
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/comd";
        };
      }
    );
}
