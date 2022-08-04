import { defineNuxtConfig } from 'nuxt';

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
  autoImports: {
    dirs: [
      'composables/**'
    ]
  },
  modules: ['@nuxt/content'],
  components: {
    dirs: ["~/components"],
    global: true,
  },
  head: {
    titleTemplate: '%s',
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: 'Meta description' },
      { noscript: [{ innerHTML: 'This website requires JavaScript.' }] },
    ],
  },
});
