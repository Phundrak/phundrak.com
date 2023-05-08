import { defineUserConfig, defaultTheme } from 'vuepress';
import { removeHtmlExtensionPlugin } from 'vuepress-plugin-remove-html-extension';
import head from './head';
import locales from './locales';
import { themeLocales } from './themeLocales';

export default defineUserConfig({
  lang: 'fr-FR',
  title: 'Lucien Cartier-Tilet',
  description: 'Site web personnel de Lucien Cartier-Tilet',
  head: head,
  markdown: {
    html: true,
    linkify: true,
    typographer: true,
  },
  plugins: [removeHtmlExtensionPlugin()],
  locales: locales,
  theme: defaultTheme({
    contributors: false,
    locales: themeLocales,
    repo: 'https://labs.phundrak.com/phundrak/phundrak.com',
  }),
});
