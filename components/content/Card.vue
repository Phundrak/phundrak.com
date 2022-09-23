<template>
  <section class="card">
    <div class="inner-card">
      <div class="card-title">
        <slot name="title"></slot>
      </div>
      <div class="card-content">
        <slot name="content"></slot>
      </div>
      <slot name="loading"></slot>
    </div>
  </section>
</template>

<style lang="scss" scoped>
@import "~/assets/mixins.scss";

.card {
  &:nth-child(even) {
    text-align: right;
  }
}

.inner-card {
  /* @include background-theme($nord1, $nord4); */
  @include background-theme($nord3, $nord4);
  @include shadow-theme(0.2rem 0.2rem 0.4rem, $nord0, $nord3);
  max-width: 30rem;
  border-radius: 0.5rem;
  padding: 1rem;

  &:hover {
    @include shadow-theme(0.5rem 0.5rem 1rem, $nord0, $nord3);
  }
}

.card-title {
  @include font-mono;
  font-size: 1.5rem;
}

.card-content {
  text-align: center;
}
</style>
