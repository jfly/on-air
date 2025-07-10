{
  description = "Detect if an attached webcam is in use";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    self,
    flake-parts,
    crane,
    treefmt-nix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {pkgs, ...}: let
        craneLib = crane.mkLib pkgs;
        treefmt = treefmt-nix.lib.evalModule pkgs ./nix/treefmt.nix;

        flattenTree = import ./nix/flattenTree.nix;

        crates = pkgs.callPackage ./nix/crates.nix {inherit craneLib;};

        packages = crates.packages;

        checks = flattenTree {
          formatting = treefmt.config.build.check self;
          inherit packages;
          "clippy" = crates.clippyCheck;
        };
      in {
        formatter = treefmt.config.build.wrapper;
        inherit checks;
        inherit packages;
        devShells.default = craneLib.devShell {
          inherit checks;
          packages = with pkgs; [rust-analyzer];
        };
      };
    };
}
