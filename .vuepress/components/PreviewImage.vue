<template>
  <a class="no-decoration" :href="src">
    <figure class="img-prev" :style="style">
      <ResponsiveImage
        :source="props.src"
        :size="props.width"
        :preview="props.preview"
        :previewWidth="props.previewWidth"
        :previewTheshold="props.maxwidth"
      />
      <figcaption>
        <slot />
      </figcaption>
    </figure>
  </a>
</template>

<script setup lang="ts">
const props = defineProps<{
  src: string;
  width: number;
  preview: string;
  previewWidth: number;
  maxwidth?: number;
}>();

const style = props.maxwidth ? `max-width: ${props.maxwidth}px` : '';
</script>

<style scoped lang="scss">
img {
  height: auto;
  width: 100%;
}

figure {
  float: left;
  margin: 0.5rem 1rem;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  max-width: 400px;
}

@media only screen and (max-width: 800px) {
  figure {
    float: none;
  }
}
</style>
