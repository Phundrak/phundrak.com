<template>
  <Cache
    :name="props.cacheName"
    :callback="fetchData"
    :data="alreadyKnownData"
    @cached="processCachedData"
  />
  <slot v-if="loading" name="loader">
    <Loader />
  </slot>
  <slot v-else-if="error" name="error"></slot>
  <slot v-else>
    {{ error }}
  </slot>
</template>

<script setup lang="ts">
import Cache from './Cache.vue';
import Loader from './Loader.vue';

import { Ref, ref } from 'vue';
import { Observable, catchError, switchMap, throwError } from 'rxjs';
import { fromFetch } from 'rxjs/fetch';

const props = defineProps({
  url: {
    default: false,
    required: true,
    type: String,
  },
  cacheName: {
    required: true,
    type: String,
  },
  alreadyKnownData: Object,
});
const emits = defineEmits(['dataLoaded', 'dataError', 'loading']);

const error: Ref<Error> = ref(null);
const loading: Ref<boolean> = ref(true);

const fetchData = (): Observable<any> => {
  return fromFetch(props.url).pipe(
    switchMap((response: Response) => response.json()),
    catchError((error: Error) => {
      console.error(error);
      return throwError(() => new Error(error.message));
    })
  );
};

const processCachedData = (data: Observable<any>) => {
  data.subscribe({
    next: (response: any) => {
      loading.value = false;
      emits('dataLoaded', response);
    },
    error: (responseError: Error) => {
      loading.value = false;
      error.value = responseError;
    },
  });
};
</script>

<style lang="less"></style>
