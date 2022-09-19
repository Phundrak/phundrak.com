import { defineNuxtConfig } from 'nuxt';

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
  app: {
    head: {
      titleTemplate: '%s | Lucien Cartier-Tilet',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        {
          hid: 'description',
          name: 'description',
          content: 'Meta description',
        },
        { name: 'msapplication-TileColor', content: '#3b4252' },
        { name: 'msapplication-TileImage', content: '/ms-icon-144x144.png' },
        { name: 'theme-color', content: '#3b4252' },
      ],
      link: [
        {
          rel: 'apple-touch-icon',
          sizes: '57x57',
          href: '/apple-icon-57x57.png',
        },
        {
          rel: 'apple-touch-icon',
          sizes: '60x60',
          href: '/apple-icon-60x60.png',
        },
        {
          rel: 'apple-touch-icon',
          sizes: '72x72',
          href: '/apple-icon-72x72.png',
        },
        {
          rel: 'apple-touch-icon',
          sizes: '76x76',
          href: '/apple-icon-76x76.png',
        },
        {
          rel: 'apple-touch-icon',
          sizes: '114x114',
          href: '/apple-icon-114x114.png',
        },
        {
          rel: 'apple-touch-icon',
          sizes: '120x120',
          href: '/apple-icon-120x120.png',
        },
        {
          rel: 'apple-touch-icon',
          sizes: '144x144',
          href: '/apple-icon-144x144.png',
        },
        {
          rel: 'apple-touch-icon',
          sizes: '152x152',
          href: '/apple-icon-152x152.png',
        },
        {
          rel: 'apple-touch-icon',
          sizes: '180x180',
          href: '/apple-icon-180x180.png',
        },
        {
          rel: 'icon',
          type: 'image/png',
          sizes: '192x192',
          href: '/android-icon-192x192.png',
        },
        {
          rel: 'icon',
          type: 'image/png',
          sizes: '32x32',
          href: '/favicon-32x32.png',
        },
        {
          rel: 'icon',
          type: 'image/png',
          sizes: '96x96',
          href: '/favicon-96x96.png',
        },
        {
          rel: 'icon',
          type: 'image/png',
          sizes: '16x16',
          href: '/favicon-16x16.png',
        },
        { rel: 'manifest', href: '/manifest.json' },
      ],
      noscript: [{ children: 'Javascript is required' }],
    },
  },
  components: {
    dirs: ['~/components'],
    global: true,
  },
  css: ['@/assets/main.scss'],
  imports: {
    global: true,
    dirs: ['~/composables'],
  },
  modules: ['@nuxt/content', '@vueuse/nuxt', '@nuxtjs/i18n'],
  i18n: {
    locales: [
      { code: 'en', iso: 'en-US', name: 'English' },
      { code: 'fr', iso: 'fr-FR', name: 'Français' },
    ],
    defaultLocale: 'fr',
    detectBrowserLanguage: {
      alwaysRedirect: true,
      useCookie: true,
      cookieKey: 'lang',
      redirectOn: 'root',
    },
    strategy: 'prefix_except_default',
    vueI18n: {
      fallbackLocale: 'fr',
      messages: {
        en: {
          sourceCode: 'Source code available on',
        },
        fr: {
          sourceCode: 'Code source disponible sur'
        }
      }
    },
  },
  nitro: {
    prerender: {
      routes: ['/sitemap.xml'],
    },
  },
  telemetry: false,
  vite: {
    define: {
      "__VUE_I18N_FULL_INSTALL__": true,
      "__VUE_I18N_LEGACY_API__": false,
      "__INTLIFY_PROD_DEVTOOLS__": false,
    }
  }
});
