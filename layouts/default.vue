<template>
  <Header />
  <slot />
  <Footer />
</template>

<style lang="scss">
@import 'node_modules/nord/src/sass/nord.scss';

body {
  margin: 0;
  font-size: 16px;
  background-color: $nord2;
  color: $nord5;
}
body::-webkit-scrollbar {
  width: 0.25rem;
}

body::-webkit-scrollbar-track {
  background: #1e1e24;
}

body::-webkit-scrollbar-thumb {
  background: #6649b8;
}

main {
  margin: 5rem;
  padding: 0 2rem;
}

a {
  text-decoration: none;
  box-shadow: inset 0 -3px 0 $nord7;
  transition: box-shadow 150ms ease;

  &:hover {
    box-shadow: inset 0 (-1em * 1.1) 0 $nord7;
  }
}
</style>
