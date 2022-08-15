<template>
  <footer>
    Source code available
    <NuxtLink to="https://labs.phundrak.com/phundrak/phundrak.com">on Gitea</NuxtLink
    >.
  </footer>
</template>

<style lang="scss">
@import 'node_modules/nord/src/sass/nord.scss';

footer {
  position: fixed;
  bottom: 0;
  background-color: $nord1;
  z-index: 1;
  width: 100vw;
  height: 2.5rem;
  line-height: 2.5rem;
  text-align: center;
  vertical-align: middle;
}
</style>
