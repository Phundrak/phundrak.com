<template>
  <div id="container">
    <Header id="header" />
    <main>
      <slot />
    </main>
    <Footer id="footer" />
  </div>
</template>

<style lang="scss" scoped>
@use 'sass:color';
@import 'node_modules/nord/src/sass/nord.scss';

#container {
  display: grid;
  grid-template-areas: 'sidebar content' 'footer footer';
  height: 100vh;
}

main {
  grid-area: content;
  overflow-y: auto;
  height: 96vh;
  box-shadow: inset 0.5rem -0.5rem 1rem color.adjust($nord0, $alpha: -0.3);
  [color-scheme='light'] & {
    box-shadow: inset 0.5rem -0.5rem 1rem color.adjust($nord3, $alpha: -0.7);
  }
}

#header {
  grid-area: sidebar;
}

#footer {
  grid-area: footer;
}
</style>
