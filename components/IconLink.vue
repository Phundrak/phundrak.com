<template>
  <NuxtLink :to="path">
    <span class="nav-icon">
      <font-awesome-icon :icon="icon" v-if="icon" size="lg" fixed-width />
    </span>
    <span class="nav-text"><slot></slot></span>
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
