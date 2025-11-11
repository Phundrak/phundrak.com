<template>
  <UFooter class="bg-background-200">
    <template #left>
      <div class="flex flex-col gap-2">
        <p class="text-text-800 text-sm">Copyright &copy; {{ new Date().getFullYear() }}</p>
        <p class="text-text-800 text-sm">{{ $t('footer.versions.frontend') }}: {{ version }}</p>
        <p class="text-text-800 text-sm">{{ $t('footer.versions.backend') }}: {{ meta?.version }}</p>
      </div>
    </template>

    <UNavigationMenu :items="items" variant="link" :orientation="orientation" />

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
import { version } from '../../package.json';

const { isMobile } = useDevice();
const orientation = computed(() => (isMobile ? 'vertical' : 'horizontal'));
const { getMeta } = useBackend();
const meta = await getMeta();
const items = computed<NavigationMenuItem[]>(() => [
  {
    label: $t('footer.links.source'),
    to: 'https://labs.phundrak.com/phundrak/phundrak.com',
  },
  {
    label: $t('footer.links.nuxt'),
    to: 'https://nuxt.com/',
  },
  {
    label: $t('footer.links.rust'),
    to: 'https://rust-lang.org/',
  },
]);
</script>
