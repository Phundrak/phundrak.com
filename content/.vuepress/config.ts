import { defaultTheme } from '@vuepress/theme-default';
import { viteBundler } from '@vuepress/bundler-vite';
import { defineUserConfig } from 'vuepress';
import { searchProPlugin } from 'vuepress-plugin-search-pro';

import { head } from './head';
import { locales, searchLocales } from './locales';
import { themeLocales } from './themeLocales';

export default defineUserConfig({
  lang: 'fr-FR',
  title: 'Lucien Cartier-Tilet',
  description: 'Site web personnel de Lucien Cartier-Tilet',
  head: head,
  bundler: viteBundler({}),
  markdown: {
    html: true,
    linkify: true,
    typographer: true,
  },
  plugins: [
    searchProPlugin({
      indexContent: true,
      locales: searchLocales,
    }),
  ],
  locales: locales,
  theme: defaultTheme({
    contributors: false,
    locales: themeLocales,
    repo: 'https://labs.phundrak.com/phundrak/phundrak.com',
  }),
});
