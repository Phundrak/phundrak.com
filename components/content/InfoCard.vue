<template>
  <Card>
    <template #title>
      <h3>
        <slot name="type"></slot>
      </h3>
    </template>
    <template #content>
      <slot />
    </template>
  </Card>
</template>

<style lang="scss" scoped>
@import "~/assets/mixins.scss";

h3 {
  @include title-ruler($color: $nord9, $left: true, $right: true);
}
</style>
