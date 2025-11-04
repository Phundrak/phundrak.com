{
  inputs,
  pkgs,
  system,
  self,
  rust-overlay,
  ...
}: let
  overlays = [(import rust-overlay)];
  rustPkgs = import inputs.nixpkgs {inherit system overlays;};
  rustVersion = rustPkgs.rust-bin.stable.latest.default;
in
  inputs.devenv.lib.mkShell {
    inherit inputs pkgs;
    modules = [
      {
        devenv.root = let
          devenvRootFileContent = builtins.readFile "${self}/.devenv-root";
        in
          pkgs.lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;
      }
      {
        packages = with rustPkgs; [
          (rustVersion.override {
            extensions = [
              "clippy"
              "rust-src"
              "rust-analyzer"
              "rustfmt"
            ];
          })
          bacon
          cargo-deny
          cargo-shuttle
          cargo-tarpaulin
          cargo-watch
          flyctl
          just
          tombi # TOML lsp server
          vscode-langservers-extracted
        ];

        services.mailpit = {
          enable = true;
          # HTTP interface for viewing emails
          uiListenAddress = "127.0.0.1:8025";
          # SMTP server for receiving emails
          smtpListenAddress = "127.0.0.1:1025";
        };

        processes.run.exec = "cargo watch -x run";

        enterShell = ''
          echo "🦀 Rust backend development environment loaded!"
          echo "📦 Rust version: $(rustc --version)"
          echo "📦 Cargo version: $(cargo --version)"
          echo ""
          echo "Available tools:"
          echo "  - rust-analyzer (LSP)"
          echo "  - clippy (linter)"
          echo "  - rustfmt (formatter)"
          echo "  - bacon (continuous testing/linting)"
          echo "  - cargo-deny (dependency checker)"
          echo "  - cargo-tarpaulin (code coverage)"
          echo ""
          echo "📧 Mailpit service:"
          echo "  - SMTP server: 127.0.0.1:1025"
          echo "  - Web UI: http://127.0.0.1:8025"
          echo ""
          echo "🚀 Quick start:"
          echo "  Run 'devenv up' to launch:"
          echo "    - Mailpit service (email testing)"
          echo "    - Backend with 'cargo watch -x run' (auto-reload)"
        '';
      }
    ];
  }
