{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    alejandra = {
      url = "github:kamadorueda/alejandra/4.0.0";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = [
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
      "phundrak-dot-com.cachix.org-1:c02/xlCknJIDoaQPUzEWSJHPoXcmIXYzCa+hVRhbDgE="
    ];
    extra-substituters = [
      "https://devenv.cachix.org"
      "https://phundrak-dot-com.cachix.org"
    ];
  };

  outputs = {
    self,
    nixpkgs,
    devenv,
    systems,
    rust-overlay,
    alejandra,
    ...
  } @ inputs: let
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    formatter = forEachSystem (system: alejandra.defaultPackage.${system});
    packages = forEachSystem (system: import ./backend/nix/package.nix { inherit rust-overlay inputs system; });
    devShells = forEachSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        backend = import ./backend/nix/shell.nix {
          inherit inputs pkgs system self rust-overlay;
        };
        frontend = import ./frontend/shell.nix {
          inherit inputs pkgs self;
        };
      }
    );
  };
}
