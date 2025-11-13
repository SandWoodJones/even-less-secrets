{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    systems.url = "github:nix-systems/default";
    rust-flake = {
      url = "github:juspay/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = with builtins; map (fn: ./nix/modules/${fn}) (attrNames (readDir ./nix/modules));

      flake = {
        meta = {
          homepage = "https://github.com/SandWoodJones/even-less-secrets";
          license = "GPL-3.0";
        };
      };
    };
}
