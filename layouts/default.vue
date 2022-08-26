<template>
  <div>
    <Header id="header" />
    <main id="content">
      <slot />
    </main>
    <Footer id="footer" />
  </div>
</template>

<style lang="scss">
@import "node_modules/nord/src/sass/nord.scss";

@import url("https://fonts.googleapis.com/css2?family=Noto+Sans&display=swap");
body {
  margin: 0;
  font-size: 16px;
  font-family: "Noto Sans", sans-serif;
  background-color: $nord2;
  color: $nord5;

  &::-webkit-scrollbar {
    width: 0.25rem;
  }
  &::-webkit-scrollbar-track {
    background: $nord2;
  }
  &::-webkit-scrollbar-thumb {
    background: $nord4;
  }
}

-webkit-scrollbar {
  width: 0.25rem;
}

-webkit-scrollbar-track {
  background: $nord2;
}

-webkit-scrollbar-thumb {
  background: $nord4;
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
