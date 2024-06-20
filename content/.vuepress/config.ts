import { defaultTheme } from '@vuepress/theme-default';
import { viteBundler } from '@vuepress/bundler-vite';
import { defineUserConfig } from 'vuepress';
import { searchProPlugin } from 'vuepress-plugin-search-pro';
import { umamiAnalyticsPlugin } from '@vuepress/plugin-umami-analytics';

import { head } from './head';
import { locales, searchLocales } from './locales';
import { themeLocales } from './themeLocales';

const isProd = process.env.NODE_ENV === 'production';

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
    isProd
      ? umamiAnalyticsPlugin({
          id: '67166941-8c83-4a19-bc8c-139e44b7f7aa',
          link: 'https://umami.phundrak.com/script.js',
        })
      : [],
  ],
  locales: locales,
  theme: defaultTheme({
    contributors: false,
    locales: themeLocales,
    repo: 'https://labs.phundrak.com/phundrak/phundrak.com',
    themePlugins: {
      copyCode: false,
      prismjs: false,
    },
  }),
});
