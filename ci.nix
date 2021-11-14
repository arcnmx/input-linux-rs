{ config, channels, pkgs, lib, ... }: with pkgs; with lib; let
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
        (impureCommand "cargo-build" "cargo build")
        (impureCommand "cargo-test" "cargo test")
      ];
    };
  };

  options = {
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
