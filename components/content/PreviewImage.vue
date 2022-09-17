<template>
  <a class="no-decoration" :href="src">
    <figure class="img-prev">
      <img :src="src" :style="style" />
      <figcaption><slot /></figcaption>
    </figure>
  </a>
</template>

<script lang="ts">
export default {
  props: {
    src: String,
    maxwidth: String,
  },
  data() {
    return {
      style: "",
    };
  },
  mounted() {
    if (this.maxwidth) {
      this.style = `max-width: ${this.maxwidth};`;
    }
  },
};
</script>

<style scoped lang="scss">
img {
  width: 100%;
  max-width: 400px;
}

figure {
  float: left;
  margin: 0.5rem;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}
</style>
