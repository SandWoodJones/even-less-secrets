{ inputs, ... }:
{
  imports = with inputs; [
    rust-flake.flakeModules.default
    rust-flake.flakeModules.nixpkgs
  ];
  perSystem =
    {
      config,
      self',
      pkgs,
      lib,
      ...
    }:
    {
      rust-project.crates."even-less-secrets".crane.args = {
        buildInputs = lib.optionals pkgs.stdenv.isDarwin (
          with pkgs.darwin.apple_sdk.frameworks;
          [
            IOKit
          ]
        );
      };
      packages.default = self'.packages.even-less-secrets;
    };
}
