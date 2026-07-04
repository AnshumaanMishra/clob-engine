{
  description = "High-performance Central Limit Order Book";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            gnuplot
            cargo-criterion # For performance benchmarking
            linuxPackages.perf # For profiling cache misses
          ];
          shellHook = ''
          exec fish
          '';
        };
      });
}
