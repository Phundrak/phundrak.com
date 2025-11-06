<template>
  <UDropdownMenu
    :key="locale"
    :items="availableLocales"
    :content="{ align: 'start' }"
  >
    <UButton color="neutral" variant="outline" icon="material-symbols:globe" />
  </UDropdownMenu>
</template>

<script setup lang="ts">
import type { DropdownMenuItem } from '@nuxt/ui';
const { locale, locales, setLocale } = useI18n();

const availableLocales = computed(() => {
  return locales.value.map(
    (optionLocale) =>
      ({
        label: optionLocale.name,
        code: optionLocale.code,
        type: 'checkbox' as const,
        checked: optionLocale.code === locale.value,
        onUpdateChecked: () => switchLocale(optionLocale.code),
      }) as DropdownMenuItem,
  );
});
const switchLocale = (newLocale: string) => {
  setLocale(newLocale);
};
</script>
