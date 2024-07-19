{ config, pkgs, env, lib, ... }: with pkgs; with lib; let
  input-linux = import ./. { pkgs = null; };
  inherit (input-linux) checks;
in {
  config = {
    name = "input-linux";
    ci.version = "v0.7";
    ci.gh-actions.enable = true;
    cache.cachix = {
      ci.signingKey = "";
      arc.enable = true;
    };
    channels = {
      nixpkgs = mkIf (env.platform != "impure") "stable";
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
      all.features = lib.remove "dox" input-linux.lib.crate.package.metadata.docs.rs.features;
      serde.features = [ "serde" ];
    } // genAttrs input-linux.lib.tokio-util-features (feat: {
      features = singleton feat;
    });
  };

  options = {
    features = mkOption {
      type = with types; listOf str;
      default = [ ];
    };
  };
}
