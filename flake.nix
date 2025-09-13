{
  description = "A Rust program flake for tianyi";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flakelight.url = "github:accelbread/flakelight";
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  inputs.crane.url = "github:ipetkov/crane";

  outputs =
    {
      flakelight,
      fenix,
      crane,
      ...
    }@inputs:
    flakelight ./. rec {
      inherit inputs;
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      # Needed to add rust-analyzer-nightly?
      withOverlays = [ fenix.overlays.default ];
      packages = {
        default =
          { pkgs, ... }:
          let
            rustToolchain = (
              fenix.packages.${pkgs.system}.minimal.withComponents [
                "cargo"
                "rustc"
              ]
            );
            craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
            cargoArtifacts = craneLib.buildDepsOnly {
              src = ./.;
              pname = "tianyi-deps";
              version = "0.1.0";
            };
          in
          craneLib.buildPackage {
            inherit cargoArtifacts;
            src = ./.;
            pname = "tianyi";
            version = "0.1.0";
          };
      };
      # Expose as runnable app
      # Notes: We can grab tianyi package from `pkgs` here, because flakelight automatically added it to
      # overlays.default.  Even for default package, it seems to grab the name from `pname`?
      apps = {
        default =
          {tianyi, ... }:
          {
            type = "app";
            program = "${tianyi}/bin/tianyi";
          };
      };
      devShell = pkgs: {
        packages =
          with pkgs;
          let
            rustToolchain = (
              fenix.packages.${pkgs.system}.complete.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ]
            );
          in
          [
            rustToolchain
            rust-analyzer-nightly
          ];
      };
    };
}
