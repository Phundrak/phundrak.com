<template>
  <UFooter>
    <template #left>
      <p class="text-300 txt-sm">
        Copyright &copy; {{ new Date().getFullYear() }}
      </p>
    </template>

    <UNavigationMenu :items="items" variant="link" />

    <template #right>
      <UButton
        icon="i-simple-icons-github"
        color="neutral"
        variant="ghost"
        to="https://github.com/Phundrak"
        target="_blank"
        aria-label="GitHub"
      />
    </template>
  </UFooter>
</template>

<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui';

const items = computed<NavigationMenuItem[]>(() => [
  {
    label: $t('footer.links.source'),
    to: 'https://labs.phundrak.com/phundrak/phundrak.com',
  },
]);
</script>
