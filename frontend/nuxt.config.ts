// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: {
    enabled: true,
    vueDevTools: true,
    telemetry: false,
  },

  modules: [
    '@nuxt/eslint',
    '@nuxt/image',
    '@nuxt/test-utils',
    '@nuxt/ui',
    '@nuxt/content',
    '@nuxtjs/i18n',
    '@nuxtjs/turnstile',
    '@nuxtjs/device',
    '@nuxt/icon',
    '@nuxt/fonts',
    '@nuxtjs/color-mode',
    '@nuxtjs/tailwindcss',
  ],

  css: ['~/assets/css/main.css'],
  content: {
    database: {
      type: 'sqlite',
      filename: '.data/content/contents.sqlite',
    },
  },
  i18n: {
    locales: [
      { code: 'en', name: 'English', language: 'en-UK', file: 'en.json' },
      { code: 'fr', name: 'Français', language: 'fr-FR', file: 'fr.json' },
      // { code: 'lfn', name: 'Elefen', language: 'lfn', file: 'lfn.json' },
      // { code: 'ei', name: 'Eittlandic', language: 'ei-ST', file: 'ei.json' },
    ],
    strategy: 'no_prefix',
    defaultLocale: 'en',
  },
  fonts: {
    provider: 'google',
    processCSSVariables: true,
    defaults: {
      weights: [400, 700],
      styles: ['normal', 'italic'],
    },
    families: [
      { name: 'Noto Sans', provider: 'google' },
      { name: 'Wittgenstein', provider: 'google' }
    ]
  },
  icon: {
    serverBundle: {
      collections: ['material-symbols', 'mdi']
    },
    clientBundle: {
      scan: true,
    }
  },
  postcss: {
    plugins: {
      '@tailwindcss/postcss': {},
      'autoprefixer': {}
    }
  },
  turnstile: {
    siteKey: '', // Overridden by NUXT_PUBLIC_TURNSTILE_SITE_KEY
    addValidateEndpoint: true
  },
  runtimeConfig: {
    turnstile: {
      secretKey: '', // Overriden by NUXT_TURNSTILE_SECRET_KEY
    },
    public: {
      apiBase: process.env.NUXT_PUBLIC_API_BASE || 'http://localhost:3100/api',
    }
  },
});
