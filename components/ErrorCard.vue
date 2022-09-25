<template>
  <Card>
    <template #title>
      <slot name="info" />
    </template>
    <template #content>
      <slot name="cause" />
    </template>
  </Card>
</template>

<style lang="scss" scoped>
@import "~/assets/mixins.scss";
.inner-card {
  @include background-theme($nord11, $nord11);
  @include color-theme($nord4, $nord4);
}
</style>
