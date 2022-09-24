<template>
  <div class="links" id="personal-infos">
    <slot />
  </div>
</template>

<style lang="scss" scoped>
@import "~/assets/mixins.scss";

.links {
  @include flex-col(2rem);
}
</style>
