{rust-overlay, inputs, system, ...}: let
overlays = [(import rust-overlay)];
in rec {
  pkgs = import inputs.nixpkgs {inherit system overlays;};
  version = pkgs.rust-bin.stable.latest.default;
}
