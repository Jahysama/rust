{ pkgs, lib, inputs, ... }:

let
  craneLib = inputs.crane.mkLib pkgs;
  
  commonArgs = {
    src = craneLib.cleanCargoSource ./.;
    cargoToml = ./Cargo.toml;
    cargoLock = ./Cargo.lock;
    strictDeps = true;
  };

  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
    pname = "rust-project-deps";
  });

in
{
  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  env = {
  };

  packages = [
    (craneLib.buildPackage (commonArgs // {
      inherit cargoArtifacts;
    }))
  ];

  enterShell = ''
    cat << 'EOF'
                _         _                           
 _ __ _   _ ___| |_    __| | _____   _____ _ ____   __
| '__| | | / __| __|  / _` |/ _ \ \ / / _ \ '_ \ \ / /
| |  | |_| \__ \ |_  | (_| |  __/\ V /  __/ | | \ V / 
|_|   \__,_|___/\__|  \__,_|\___| \_/ \___|_| |_|\_/  
EOF
  '';
}
