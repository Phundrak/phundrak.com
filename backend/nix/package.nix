{
  rust-overlay,
  inputs,
  system,
  ...
}: let
  rust = import ./rust-version.nix { inherit rust-overlay inputs system; };
  pkgs = rust.pkgs;
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rust.version;
    rustc = rust.version;
  };
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
  name = cargoToml.package.name;
  version = cargoToml.package.version;
  rustBuild = rustPlatform.buildRustPackage {
    pname = name;
    inherit version;
    src = ../.;
    cargoLock.lockFile = ../Cargo.lock;
  };
  settingsDir = pkgs.runCommand "settings" {} ''
    mkdir -p $out/settings
    cp ${../settings}/*.yaml $out/settings/
  '';
  makeDockerImage = tag:
    pkgs.dockerTools.buildLayeredImage {
      name = "phundrak/${name}";
      inherit tag;
      created = "now";
      config = {
        Entrypoint = ["${rustBuild}/bin/${name}"];
        WorkingDir = "/";
        Env = [
          "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
        ];
        ExposedPorts = {
          "3100/tcp" = {};
        };
        Labels = {
          "org.opencontainers.image.title" = name;
          "org.opencontainers.image.version" = version;
          "org.opencontainers.image.description" = "REST API backend for phundrak.com";
          "org.opencontainers.image.authors" = "Lucien Cartier-Tilet <lucien@phundrak.com>";
          "org.opencontainers.image.licenses" = "AGPL-3.0-only";
          "org.opencontainers.image.source" = "https://labs.phundrak.com/phundrak/phundrak.com";
          "org.opencontainers.image.url" = "https://labs.phundrak.com/phundrak/phundrak.com";
          "org.opencontainers.image.documentation" = "https://labs.phundrak.com/phundrak/phundrak.com";
          "org.opencontainers.image.vendor" = "Phundrak";
        };
      };
      contents = [rustBuild pkgs.cacert settingsDir];
    };
  dockerImageLatest = makeDockerImage "latest";
  dockerImageVersioned = makeDockerImage version;
in {
  backend = rustBuild;
  backendDocker = dockerImageVersioned;
  backendDockerLatest = dockerImageLatest;
}
