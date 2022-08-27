<template>
  <NuxtLink :to="path">
    <Icon class="nav-icon" :icon="icon"></Icon>
    <span class="nav-text">
      <slot />
    </span>
  </NuxtLink>
</template>

<script lang="ts">
export default {
  props: {
    icon: String,
    path: String,
  },
};
</script>
