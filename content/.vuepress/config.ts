import { defaultTheme } from '@vuepress/theme-default';
import { viteBundler } from '@vuepress/bundler-vite';
import { defineUserConfig } from 'vuepress';
import { slimsearchPlugin } from '@vuepress/plugin-slimsearch';
import { umamiAnalyticsPlugin } from '@vuepress/plugin-umami-analytics';

import { head } from './head';
import { locales, searchLocaleLfn } from './locales';
import { themeLocales } from './themeLocales';

const isProd = process.env.NODE_ENV === 'production';

export default defineUserConfig({
  lang: 'fr-FR',
  title: 'Lucien Cartier-Tilet',
  description: 'Site web personnel de Lucien Cartier-Tilet',
  head: head,
  bundler: isProd
    ? viteBundler({})
    : viteBundler({
        viteOptions: {
          server: {
            allowedHosts: true,
          },
        },
      }),
  markdown: {
    html: true,
    linkify: true,
    typographer: true,
  },
  plugins: [
    slimsearchPlugin({
      indexContent: true,
      indexLocaleOptions: {
        '/lfn': searchLocaleLfn,
      },
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
