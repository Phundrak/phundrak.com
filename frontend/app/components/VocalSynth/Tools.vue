<template>
  <UiBadgeListCard v-if="data" :tools="data.tools">{{ $t('pages.vocal-synthesis.tools') }}</UiBadgeListCard>
</template>

<script setup lang="ts">
// Inject data provided by the page to avoid hydration issues with MDC components
const data = inject('pageData');
</script>
