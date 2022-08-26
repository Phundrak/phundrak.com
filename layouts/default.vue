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
#container {
  display: flex;
  height: 100vh;
  width: 100vw;
}
</style>
