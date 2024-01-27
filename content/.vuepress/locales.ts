import { SearchProLocaleConfig } from 'vuepress-plugin-search-pro';

export const locales = {
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
  '/lfn/': {
    lang: 'lfn',
    title: 'Lucien Cartier-Tilet',
    description: 'loca ueb de Lucien Cartier-Tilet',
  },
};

export const searchLocales: SearchProLocaleConfig = {
  '/fr/': {
    cancel: 'Annuler',
    placeholder: 'Rechercher',
    search: 'Rechercher',
    searching: 'Recherche',
    defaultTitle: 'Documentation',
    select: 'sélectionner',
    navigate: 'naviguer',
    autocomplete: 'auto-complétion',
    exit: 'fermer',
    queryHistory: 'Historique de recherche',
    resultHistory: 'Historique des résultats',
    emptyHistory: "Vider l'historique de recherche",
    emptyResult: 'Aucun résultat trouvé',
    loading: 'Chargement des index de recherche...',
  },
  '/lfn/': {
    cancel: 'Cansela',
    placeholder: 'Xerca',
    search: 'Xerca',
    searching: 'Xercante',
    defaultTitle: 'Documentos',
    select: 'eleje',
    navigate: 'naviga',
    autocomplete: 'auto-completi',
    exit: 'sorti',
    queryHistory: 'Historia de xerca',
    resultHistory: 'Historia de resultas',
    emptyHistory: 'Historia vacua',
    emptyResult: 'Resultas vacua',
    loading: 'Cargante la indise de xerca...',
  },
};
