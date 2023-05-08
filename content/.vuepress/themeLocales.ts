const pages: string[] = [
  '/index.md',
  '/find-me.md',
  '/resume.md',
  '/projects.md',
  '/conlanging.md',
  '/vocal-synthesis.md',
  '/about.md',
  '/privacy.md',
];

const localePages = (languagePrefix: string) => {
  return pages.map((page: string) => `/${languagePrefix}${page}`);
};

export const themeLocales = {
  '/': {
    selectLanguageName: 'Français',
    tip: 'nota bene',
    warning: 'attention',
    sidebar: pages,
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
    lastUpdatedText: 'Dernière mise à jour',
  },
  '/lfn/': {
    selectLanguageName: 'Elefen',
    tip: 'avisa',
    warning: 'averti',
    danger: 'peril',
    sidebar: localePages('lfn'),
    notFound: [
      'Ce? Se no ave no cosa asi',
      'A do vade tu?',
      'Era 404',
      'La lia no es coreta',
    ],
    backToHome: 'reversa a la paja prima',
    openInNewWindow: 'abri en un nova fenetra',
    toggleColorMode: 'cambia la colores',
    toggleSidebar: 'bara ladal',
    lastUpdatedText: 'Ultima refresci',
  },
  '/en/': {
    selectLanguageName: 'English',
    sidebar: localePages('en'),
  },
};
