{ wf-bot }:
{ config, lib, ... }:

{
  options.services.wf-bot = {
    enable = lib.mkEnableOption "enable the wf-bot Discord Bot";
    EnvironmentFile = lib.mkOption {
      type = lib.types.str;
      description = "Path to an environment file with a Discord API Token and Discord Channel ID.";
    };
  };

  config =
    let
      cfg = config.services.wf-bot;
    in
    lib.mkIf cfg.enable {
      systemd.services.wf-bot = {
        description = "Warframe Discord bot";
        after = [ "network-online.target" ];
        wants = [ "network-online.target" ];
        wantedBy = [ "multi-user.target" ];
        serviceConfig = {
          Type = "simple";
          EnvironmentFile = cfg.EnvironmentFile;
          ExecStart = "${wf-bot}/bin/wf-bot";
          Restart = "always";
        };
      };
    };

}
