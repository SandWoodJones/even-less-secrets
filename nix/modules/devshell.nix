{ inputs, ... }:
{
  perSystem =
    {
      config,
      self',
      pkgs,
      lib,
      ...
    }:
    {
      devShells.default = pkgs.mkShell {
        name = "even-less-secrets-shell";
        inputsFrom = [ self'.devShells.rust ];
        packages = with pkgs; [
          bacon
          omnix
        ];
      };
    };
}
