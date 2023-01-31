import { defineUserConfig, defaultTheme } from 'vuepress';

export default defineUserConfig({
  lang: 'fr-FR',
  title: 'Lucien Cartier-Tilet',
  description: 'Site web personnel de Lucien Cartier-Tilet',
  head: [
    [
      'link',
      {
        rel: 'icon',
        href: 'https://cdn.phundrak.com/img/mahakala-128x128.png',
      },
    ],
    [
      'meta',
      {
        name: 'author',
        content: 'Lucien Cartier-Tilet',
      },
    ],
    [
      'meta',
      {
        property: 'og:image',
        content: 'https://cdn.phundrak.com/img/rich_preview.png',
      },
    ],
    [
      'meta',
      {
        property: 'org:title',
        content: 'Lucien Cartier-Tilet',
      },
    ],
    [
      'meta',
      {
        property: 'og:description',
        content: 'Site web personnel de Lucien Cartier-Tilet',
      },
    ],
    [
      'meta',
      {
        name: 'twitter:card',
        content: 'summary',
      },
    ],
    [
      'meta',
      {
        name: 'twitter:site',
        content: '@phundrak',
      },
    ],
    [
      'meta',
      {
        name: 'twitter:creator',
        content: '@phundrak',
      },
    ],
  ],
  markdown: {
    html: true,
    linkify: true,
    typographer: true,
  },
  locales: {
    '/': {
      lang: 'fr-FR',
      title: 'Lucien Cartier-Tilet',
      description: 'Site web personnel de Lucien Cartier-Tilet',
    },
    '/en/': {
      lang: 'en-US',
      title: 'Lucien Cartier-Tilet',
      description: 'Personal website of Lucien Cartier-Tilet',
    },
  },
  theme: defaultTheme({
    locales: {
      '/': {
        selectLanguageName: 'Français',
        tip: 'nota bene',
        warning: 'attention',
        sidebar: [
          '/README.md',
          '/about.md',
          '/resume.md',
          '/projects.md',
          '/conlanging.md',
          '/vocal-synthesis.md',
        ],
        notFound: [
          'C’est bien vide ici',
          'Pourquoi sommes-nous ici?',
          'Erreur 404',
          'Le lien ne semble pas être correct',
        ],
        backToHome: 'Retour accueil',
        openInNewWindow: 'Ouvrir dans une nouvelle fenêtre',
        toggleColorMode: 'Changer de thème',
        toggleSidebar: 'Barre latérale',
      },
      '/en/': {
        selectLanguageName: 'English',
        selectLanguageText: 'Langues',
        sidebar: [
          '/en/index.md',
          '/en/about.md',
          '/en/resume.md',
          '/en/projects.md',
          '/en/conlanging.md',
          '/en/vocal-synthesis.md',
        ],
      },
    },
  }),
});
