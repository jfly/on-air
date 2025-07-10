{ inputs, ... }:
{
  perSystem =
    { pkgs, self', ... }:
    let
      craneLib = inputs.crane.mkLib pkgs;

      commonArgs = {
        pname = "on-air";
        src = craneLib.cleanCargoSource (craneLib.path ../.);
        strictDeps = true;
        nativeBuildInputs = with pkgs; [
          rustPlatform.bindgenHook
        ];
        # Avoid unnecessary rebuilds of the bindgen crate. See
        # https://crane.dev/faq/rebuilds-bindgen.html
        NIX_OUTPATH_USED_AS_RANDOM_SEED = "deadbeef";
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      clippyCheck = craneLib.cargoClippy (
        commonArgs
        // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        }
      );
      myCrate = craneLib.buildPackage (
        commonArgs
        // {
          inherit cargoArtifacts;
        }
      );
    in
    {
      checks = {
        "packages/default" = self'.packages.default;
        clippy = clippyCheck;
      };
      packages = {
        default = myCrate;
      };
      devShells.default = craneLib.devShell {
        packages = with pkgs; [ rust-analyzer ];
      };
    };
}
