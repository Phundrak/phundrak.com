// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: {
    enabled: true,
    vueDevTools: true,
    telemetry: false,
  },

  modules: [
    '@nuxt/content',
    '@nuxt/eslint',
    '@nuxt/image',
    '@nuxt/test-utils',
    '@nuxt/ui',
    '@vueuse/nuxt',
    '@nuxtjs/i18n',
    '@nuxtjs/turnstile',
  ],

  content: {
    database: {
      type: 'sqlite',
      filename: '.data/content/contents.sqlite',
    },
  },
  i18n: {
    locales: [
      { code: 'en', name: 'English', language: 'en-UK' },
      { code: 'fr', name: 'Français', language: 'fr-FR' },
      { code: 'lfn', name: 'Lingua Franca Nova', language: 'lfn' },
    ],
    strategy: 'prefix_except_default',
    defaultLocale: 'en',
  },
  turnstile: {
    siteKey: '', // Overridden by NUXT_PUBLIC_TURNSTILE_SITE_KEY
  },
  runtimeConfig: {
    turnstile: {
      secretKey: '', // Overriden by NUXT_TURNSTILE_SECRET_KEY
    },
  },
});
