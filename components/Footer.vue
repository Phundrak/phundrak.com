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
@import "node_modules/nord/src/sass/nord.scss";
footer {
  position: fixed;
  width: 100vw;
  height: 2rem;
  bottom: 0;
  background-color: $nord1;
  text-align: center;
  line-height: 2rem;
  z-index: 10;
  box-shadow: 0 -0.2rem 0.5rem $nord0;
}
</style>
