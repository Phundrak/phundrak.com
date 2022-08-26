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
@use 'sass:color';
@import 'node_modules/nord/src/sass/nord.scss';
footer {
  width: 100vw;
  height: 4vh;
  background-color: $nord1;
  text-align: center;
  line-height: 4vh;
  z-index: 10;

  [color-scheme='light'] & {
    background-color: $nord4;
  }
}
</style>
