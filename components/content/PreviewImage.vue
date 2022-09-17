<template>
  <a class="no-decoration" :href="src">
    <figure class="img-prev" :style="style">
      <img :src="src" />
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
@import "~/assets/mixins.scss";

img {
  height: auto;
  width: 100%;
}

figure {
  float: left;
  margin: 0.5rem;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  max-width: 400px;
}

@include small-screen {
  figure {
    float: none;
  }
}

@media only screen and (max-width: 800px) {
  figure {
    float: none;
  }
}
</style>
