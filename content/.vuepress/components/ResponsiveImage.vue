<template>
  <img :srcset="srcset" :sizes="sizes" :alt="props.alt" :src="props.src" />
</template>

<script setup lang="ts">
const props = defineProps<{
  src: string;
  width: number;
  preview: string;
  previewWidth: number;
  previewThreshold?: number;
  alt?: string;
}>();

const srcset = [
  `${props.preview} ${props.previewWidth}w`,
  `${props.src} ${props.width}w`,
].join(', ');
const sizes = [
  `(max-width: ${props.previewThreshold || props.previewWidth}px) ${
    props.previewWidth
  }px`,
  `${props.width}px`,
].join(', ');
</script>
