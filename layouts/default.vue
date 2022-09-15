<template>
  <div id="glob-container">
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

#glob-container {
  display: grid;
  grid-template-areas: "header content" "footer footer";
  grid-template-columns: auto 1fr;
  height: 100vh;
  overflow: hidden;
}

#header {
  grid-area: header;
}

#footer {
  grid-area: footer;
}

main {
  grid-area: content;
  overflow-y: auto;
  overflow-x: hidden;
  height: 96vh;
  box-shadow: inset 0.5rem -0.5rem 1rem color.adjust($nord0, $alpha: -0.3);

  [color-scheme="light"] & {
    box-shadow: inset 0.5rem -0.5rem 1rem color.adjust($nord3, $alpha: -0.7);
  }
}
</style>
