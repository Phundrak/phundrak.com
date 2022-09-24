<template>
  <CardList class="personal-infos">
    <template #list>
      <slot />
    </template>
  </CardList>
</template>

<style lang="scss" scoped>
@import "~/assets/mixins.scss";

.links {
  @include flex-col(2rem);
}
</style>
