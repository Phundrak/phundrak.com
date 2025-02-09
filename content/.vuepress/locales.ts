import SlimSarchLocaleData from '@vuepress/plugin-slimsearch';

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

export const searchLocaleLfn: SlimSarchLocaleData = {
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
};
