<template>
  <section class="link-list">
    <section class="list">
      <slot name="list" />
    </section>
    <div id="loader">
      <slot name="loader" />
    </div>
  </section>
</template>

<style lang="scss" scoped>
@import "~/assets/mixins.scss";

.link-list {
  @include flex-col(2rem);
}
</style>
