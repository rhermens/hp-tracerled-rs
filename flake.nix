{
  description = "HP Tracer LED controller";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          craneLib = crane.mkLib pkgs;

          hp-tracerled = craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;
            buildInputs = [ pkgs.hidapi pkgs.udev ];
            nativeBuildInputs = [ pkgs.pkg-config ];
          };
        in
        {
          packages.default = hp-tracerled;

          devShells.default = pkgs.mkShell {
            buildInputs = [ pkgs.hidapi pkgs.udev ];
            nativeBuildInputs = with pkgs; [ rustc cargo pkg-config ];
          };
        }
      ) // {
      nixosModules.default = { config, lib, pkgs, ... }:
        with lib;
        let
          cfg = config.services.hp-tracerled;
        in
        {
          options.services.hp-tracerled = {
            enable = mkEnableOption "HP Tracer LED controller";

            color = mkOption {
              type = types.str;
              default = "FD4001";
              description = "LED color as a hex RGB value (without #)";
            };

            mode = mkOption {
              type = types.enum [ "static" "pulse" "flash" "random" ];
              default = "static";
              description = "LED animation mode";
            };

            speed = mkOption {
              type = types.ints.between 1 3;
              default = 1;
              description = "Animation speed (1-3)";
            };
          };

          config = mkIf cfg.enable {
            systemd.services.hp-tracerled = {
              description = "Set HP Tracerled device colors";
              after = [ "multi-user.target" ];
              wantedBy = [ "multi-user.target" ];
              serviceConfig = {
                Type = "oneshot";
                RemainAfterExit = true;
                ExecStart = "${self.packages.${pkgs.stdenv.hostPlatform.system}.default}/bin/hp-tracerled --color=${cfg.color} --mode=${cfg.mode} --speed=${toString cfg.speed}";
              };
            };
          };
        };
    };
}
