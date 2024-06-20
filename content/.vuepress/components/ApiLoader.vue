<template>
  <slot v-if="loading" name="loader">
    <LoaderAnimation />
  </slot>
  <slot v-else-if="error" name="error">
    <FetchError :url="props.url" />
  </slot>
  <slot v-else> </slot>
</template>

<script setup lang="ts">
import LoaderAnimation from './LoaderAnimation.vue';
import FetchError from './FetchError.vue';

import { useFetchAndCache } from '../composables/fetchAndCache';

const props = defineProps({
  url: {
    default: '',
    required: true,
    type: String,
  },
  cacheName: {
    required: true,
    type: String,
  },
  alreadyKnownData: Object,
});

const emits = defineEmits(['loaded', 'error', 'loading']);

const { loading, error } = useFetchAndCache(props.url, {
  emits: emits,
  cacheName: props.cacheName,
});
</script>
