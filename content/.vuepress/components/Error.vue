<template>
  <div class="error rounded-corners card-width">
    <p>API call to {{ props.url }} failed</p>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  url: {
    required: true,
    type: String,
  },
});
</script>

<style lang="less">
@import 'node_modules/nord/src/lesscss/nord.less';
@import '../styles/classes.less';

.error {
  display: inline-block;
  padding: 2rem;
  text-align: center;
  background: @nord11;
}
</style>
