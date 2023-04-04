{ config, channels, pkgs, lib, ... }: with pkgs; with lib; let
  inherit (import ./. { inherit pkgs; }) checks;
in {
  config = {
    name = "input-linux";
    ci.gh-actions.enable = true;
    cache.cachix = {
      ci.signingKey = "";
      arc.enable = true;
    };
    channels = {
      nixpkgs = "stable";
    };
    tasks = {
      test.inputs = singleton (checks.test.override {
        buildFeatures = config.features;
      });
    };
    system = "x86_64-linux";
    jobs = {
      default = {
        tasks.version.inputs = singleton checks.version;
      };
      all.features = [ "with-serde" "with-tokio" ];
      tokio.features = [ "with-tokio" ];
      serde.features = [ "with-serde" ];
    };
  };

  options = {
    features = mkOption {
      type = with types; listOf str;
      default = [ ];
    };
  };
}
