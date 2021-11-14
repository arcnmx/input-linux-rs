{ config, channels, pkgs, lib, ... }: with pkgs; with lib; let
  featuresStr = optionalString (config.features != [ ]) (
    "--features " + concatStringsSep "," config.features
  );
  impureCommand = name: command: ci.command {
    inherit name command;
    impure = true;
  };
in {
  config = {
    name = "input-linux";
    ci.gh-actions.enable = true;
    cache.cachix.arc.enable = true;
    channels = {
      rust = "master";
    };
    environment = {
      test = {
        inherit (config.rustChannel.buildChannel) cargo;
      };
    };
    tasks = {
      build.inputs = [
        (impureCommand "cargo-build" "cargo build ${featuresStr}")
        (impureCommand "cargo-test" "cargo test ${featuresStr}")
      ];
    };
    jobs = {
      default = { };
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
    rustChannel = mkOption {
      type = types.unspecified;
      default = channels.rust.stable;
    };
    shell = mkOption {
      type = types.unspecified;
      default = config.rustChannel.mkShell { };
    };
  };
}
