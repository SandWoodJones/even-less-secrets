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
      devShells.default =
        pkgs.mkShell.override { stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv; }
          {
            name = "even-less-secrets-shell";
            inputsFrom = [ self'.devShells.rust ];
            packages = with pkgs; [
              bacon
              omnix

              nms
            ];

            RUSTFLAGS="-C link-arg=-fuse-ld=mold";
          };
    };
}
