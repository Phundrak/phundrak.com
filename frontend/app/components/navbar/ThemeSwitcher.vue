<template>
  <UDropdownMenu :key="colorMode.preference" :items="themes" :content="{ align: 'start' }">
    <UButton color="neutral" variant="outline" :icon="icons[currentColor]" :aria-label="$t('menu.theme')" />
  </UDropdownMenu>
</template>

<script setup lang="ts">
type Theme = 'light' | 'dark' | 'system';
const icons: Dictionary<Theme, string> = {
  light: 'material-symbols:light-mode',
  dark: 'material-symbols:dark-mode',
  system: 'material-symbols:computer-outline',
};
const colorMode = useColorMode();
const currentColor = computed<Theme>(() => colorMode.preference ?? 'system');
const themes = computed<DropdownValue[]>(() =>
  ['light', 'dark', 'system'].map((theme) => ({
    code: theme,
    label: $t(`theme.${theme}`),
    icon: icons[theme],
    type: 'checkbox' as const,
    checked: currentColor.value === theme,
    onUpdateChecked: () => switchColor(theme),
  })),
);
const switchColor = (theme: Theme) => (colorMode.preference = theme);
</script>
