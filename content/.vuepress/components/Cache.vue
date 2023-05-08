<template>
  <slot />
</template>

<script setup lang="ts">
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
  alreadyKnownData: {
    default: null,
    type: Object,
  },
});

const emits = defineEmits(['cached']);

const isDataOutdated = (name: string): boolean => {
  const lastUpdated: number = +localStorage.getItem(name + '-timestamp');
  const elapsedTime: number = Date.now() - lastUpdated;
  return elapsedTime > props.lifetime;
};

const storeInCache = (
  callback: Function,
  data: any,
  name: string
): Observable<any> => {
  let response: Observable<any> = data ? of(data) : callback();

  response.subscribe({
    next: (response) => {
      localStorage.setItem(name, JSON.stringify(response));
      localStorage.setItem(name + '-timestamp', `${Date.now()}`);
    },
  });
  return response;
};

if (isDataOutdated(props.name)) {
  emits(
    'cached',
    storeInCache(props.callback, props.alreadyKnownData, props.name)
  );
} else {
  let data = localStorage.getItem(props.name);
  try {
    emits('cached', of(JSON.parse(data)));
  } catch (err) {
    console.error(`Could not parse data found in cache: ${err}`);
    emits(
      'cached',
      storeInCache(props.callback, props.alreadyKnownData, props.name)
    );
  }
}
</script>
