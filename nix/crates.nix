{
  craneLib,
  pkgs,
}: let
  inherit
    (pkgs.lib)
    getExe
    ;

  commonArgs = {
    pname = "on-air";
    src = craneLib.cleanCargoSource (craneLib.path ../.);
    strictDeps = true;
    nativeBuildInputs = with pkgs; [
      rustPlatform.bindgenHook
    ];

    preBuild = ''
      # This is a nasty workaround for https://github.com/ipetkov/crane/discussions/518
      # to avoid unnecessarily recompiling bindgen.
      export BINDGEN_EXTRA_CLANG_ARGS=$(echo $BINDGEN_EXTRA_CLANG_ARGS | ${getExe pkgs.gnused} 's/-frandom-seed=[^ ]\+/-frandom-seed=deadbeef/')
    '';
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
  clippyCheck = craneLib.cargoClippy (commonArgs
    // {
      inherit cargoArtifacts;
      cargoClippyExtraArgs = "--all-targets -- --deny warnings";
    });
  myCrate = craneLib.buildPackage (commonArgs
    // {
      inherit cargoArtifacts;
    });
in {
  packages.default = myCrate;
  inherit clippyCheck;
}
