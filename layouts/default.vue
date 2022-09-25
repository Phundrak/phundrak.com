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
@import "~/assets/mixins.scss";

#glob-container {
  display: grid;
  grid-template-areas: "header content" "footer footer";
  grid-template-columns: auto 1fr;
  grid-template-rows: 1fr auto;
  height: 100vh;
  overflow: hidden;
}

@include small-screen {
  #glob-container {
    @include flex-col;
  }
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
  @include shadow-theme(inset 0.5rem -0.5rem 1rem, $nord0, $nord3);
  box-sizing: content-box;
}

@include small-screen {
  main {
    box-shadow: none !important;
  }

  #header {
    @include shadow-theme(0.5rem 0.5rem 1rem, $nord0, $nord3);
  }
  #footer {
    @include shadow-theme(0.5rem -0.5rem 1rem, $nord0, $nord3);
  }
}
</style>
