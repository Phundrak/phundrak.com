{
  inputs,
  pkgs,
  self,
  ...
}:
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
      env.PNPM_HOME = "${self}/.pnpm-store";

      packages = with pkgs; [
        # LSP
        marksman
        # nodePackages."@tailwindcss/language-server"
        # nodePackages."@vue/language-server"
        # vscode-langservers-extracted

        rustywind
        nodePackages.prettier
        nodePackages.eslint

        # Node
        nodejs_24
        nodePackages.pnpm

        # typescript
        # nodePackages.typescript-language-server
      ];

      enterShell = ''
        echo "🚀 Nuxt.js development environment loaded!"
        echo "📦 Node.js version: $(node --version)"
        echo "📦 pnpm version: $(pnpm --version)"
        echo ""
        echo "Available LSP servers:"
        echo "  - typescript-language-server (TypeScript)"
        echo "  - vue-language-server (Vue/Volar)"
        echo "  - tailwindcss-language-server (Tailwind CSS)"
        echo "  - vscode-langservers-extracted (HTML, CSS, JSON, ESLint)"
        echo ""
        echo "Run 'pnpm install' to install dependencies"
        echo "Run 'pnpm dev' to start the development server"
      '';
    }
  ];
}
