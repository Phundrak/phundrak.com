<template>
  <UPageCard class="bg-background-100 my-10">
    <p class="text-xl">
      <slot />
    </p>
    <UiBadgeList :tools="tools" />
  </UPageCard>
</template>

<script setup lang="ts">
const { tools } = defineProps<{
  tools: string[];
}>();
</script>
