{ config, channels, pkgs, lib, ... }: with pkgs; with lib; let
  input-linux = import ./. { inherit pkgs; };
  inherit (input-linux) checks;
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
      all = {
        inherit (input-linux.lib.crate.package.metadata.docs.rs) features;
      };
      tokio.features = [ "tokio-util" ];
      serde.features = [ "serde" ];
    };
  };

  options = {
    features = mkOption {
      type = with types; listOf str;
      default = [ ];
    };
  };
}
