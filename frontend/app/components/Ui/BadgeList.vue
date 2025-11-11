<template>
  <div v-if="tools" class="flex flex-row gap-1 flex-wrap">
    <UBadge v-for="tool in tools" :key="tool" size="md" variant="solid">
      {{ tool }}
    </UBadge>
  </div>
</template>

<script setup lang="ts">
const { tools } = defineProps<{
  tools: string[];
}>();
</script>
