<template>
  <img :srcset="srcset" :sizes="sizes" :alt="props.alt" :src="props.source" />
</template>

<script setup lang="ts">
const props = defineProps<{
  source: string;
  size: number;
  preview: string;
  previewWidth: number;
  previewThreshold?: number;
  alt?: string;
}>();

const srcset = [
  `${props.preview} ${props.previewWidth}w`,
  `${props.source} ${props.size}w`,
].join(", ");
const sizes = [
  `(max-width: ${props.previewThreshold || props.previewWidth}px) ${
    props.previewWidth
  }px`,
  `${props.size}px`,
].join(", ");
</script>
