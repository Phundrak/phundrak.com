<template>
  <button>
    <Icon class="nav-icon" :icon="icon" />
    <span class="nav-text">
      <slot />
    </span>
  </button>
</template>

<script lang="ts">
export default {
  props: {
    icon: String,
  },
};
</script>

<style lang="scss" scoped>
button {
  border: none;
  cursor: pointer;
}
</style>
