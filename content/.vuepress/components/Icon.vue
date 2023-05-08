<template>
  <i :class="`icon phunic-${props.name}`" />
</template>

<script setup lang="ts">
const props = defineProps({
  name: {
    default: '',
    required: true,
    type: String,
  },
});
</script>

<style lang="less">
@import '../styles/fonts.less';
</style>
