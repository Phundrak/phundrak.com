<template>
  <div id="glob-container">
    <div id="page-container">
      <Header id="header" />
      <main>
        <slot />
      </main>
    </div>
    <Footer id="footer" />
  </div>
</template>

<style lang="scss" scoped>
@use 'sass:color';
@import 'node_modules/nord/src/sass/nord.scss';

#glob-container {
  display: grid;
  grid-template-areas: 'content' 'footer';
  height: 100vh;
}

#page-container {
  grid-area: 'content';
  display: flex;
  flex-direction: row;
}

main {
  overflow-y: auto;
  height: 96vh;
  box-shadow: inset 0.5rem -0.5rem 1rem color.adjust($nord0, $alpha: -0.3);
  [color-scheme='light'] & {
    box-shadow: inset 0.5rem -0.5rem 1rem color.adjust($nord3, $alpha: -0.7);
  }
}

#footer {
  grid-area: footer;
}
</style>
