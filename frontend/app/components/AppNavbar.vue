<template>
  <UHeader toggle-side="right" mode="drawer">
    <template #title> Phundrak </template>
    <UNavigationMenu :items="items" />
    <template #right>
      <NavbarLanguageSwitcher />
      <NavbarThemeSwitcher />
    </template>
    <template #body>
      <UNavigationMenu :items="items" orientation="vertical" class="-mx-2.5" />
    </template>
  </UHeader>
</template>

<script setup lang="ts">
const route = useRoute();
const items = computed<NavigationMenuItem[]>(() => [
  {
    label: $t('pages.home.name'),
    to: '/',
    active: route.path == '/',
  },
  ...['resume', 'vocal-synthesis', 'languages', 'contact'].map((page) => ({
    label: $t(`pages.${page}.name`),
    to: `/${page}`,
    active: route.path.startsWith(`/${page}`),
  })),
]);
</script>
