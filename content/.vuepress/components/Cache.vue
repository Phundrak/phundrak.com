<template>
  <slot />
</template>

<script setup lang="ts">
import { Ref, ref } from 'vue';
import { Observable, of } from 'rxjs';

const props = defineProps({
  name: {
    required: true,
    type: String,
  },
  callback: {
    required: true,
    type: Function,
  },
  lifetime: {
    default: 1000 * 60 * 60, // one hour
    required: false,
    type: Number,
  },
});

const emits = defineEmits(['cached']);

const isDataOutdated = (name: string): boolean => {
  const lastUpdated: number = +localStorage.getItem(name + '-timestamp');
  const elapsedTime: number = Date.now() - lastUpdated;
  return elapsedTime > props.lifetime;
};

const storeInCache = (data: Observable<any>, name: string): Observable<any> => {
  data.subscribe({
    next: (response) => {
      localStorage.setItem(name, JSON.stringify(response));
      localStorage.setItem(name + '-timestamp', `${Date.now()}`);
    },
  });
  return data;
};

const dataFromCache: Ref<Observable<any>> = ref(null);
if (isDataOutdated(props.name)) {
  emits('cached', storeInCache(props.callback(), props.name));
} else {
  console.log('Sending cached data');
  let data = localStorage.getItem(props.name);
  try {
    emits('cached', of(JSON.parse(data)));
  } catch (err) {
    console.error(
      `Could not parse ${JSON.stringify(
        dataFromCache
      )}: ${err}. Fetching again data from API.`
    );
    emits('cached', storeInCache(props.callback(), props.name));
  }
}
</script>
