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
@import "~/assets/mixins.scss";

footer {
  width: 100vw;
  height: 2rem;
  line-height: 2rem;
  text-align: center;
  z-index: 10;

  @include background-theme($nord1, $nord4);
}
</style>
