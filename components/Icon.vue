<template>
  <i :class="iconClass"></i>
</template>

<script lang="ts">
export default {
  props: {
    icon: String,
  },
  data() {
    return {
      iconClass: "icon phunic-",
    };
  },
  mounted() {
    this.iconClass += this.icon;
  },
};
</script>
