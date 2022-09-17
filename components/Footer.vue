<template>
  <footer>
    Code source disponible sur
    <NuxtLink to="https://labs.phundrak.com/phundrak/phundrak.com">
      on Gitea
    </NuxtLink>
    .
  </footer>
</template>

<style lang="scss" scoped>
@use "sass:color";
@import "~/assets/mixins.scss";

footer {
  width: 100vw;
  height: 2rem;
  background-color: $nord1;
  text-align: center;
  line-height: 2rem;
  z-index: 10;

  [color-scheme="light"] & {
    background-color: $nord4;
  }
}
</style>
