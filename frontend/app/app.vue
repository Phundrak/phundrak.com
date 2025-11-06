<template>
  <UApp :locale="locales[locale]">
    <AppNavbar />
    <UMain>
      <NuxtLayout>
        <NuxtPage />
      </NuxtLayout>
    </UMain>
    <AppFooter />
  </UApp>
</template>

<script setup lang="ts">
import * as locales from '@nuxt/ui/locale';
const { locale } = useI18n();
const lang = computed(() => locales[locale.value].code);
const dir = computed(() => locales[locale.value].dir);

useHead({
  htmlAttrs: {
    dir,
    lang,
  },
});
</script>
