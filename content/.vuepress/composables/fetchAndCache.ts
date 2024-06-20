import { ref, Ref } from 'vue';
import { useCache } from './cache';

type FetchAndCacheEmitter = (
  event: 'loaded' | 'error' | 'loading',
  ...args: any[]
) => void;

interface UseFetchAndCacheOptions {
  cacheLifetime?: number;
  cacheName?: string;
  emits?: FetchAndCacheEmitter;
}

const dummyEmits = (
  _event: 'loaded' | 'error' | 'loading',
  ..._args: any[]
) => {};

export const useFetchAndCache = <T, E>(
  url: string,
  options?: UseFetchAndCacheOptions,
) => {
  const data = ref<T | null>(null) as Ref<T>;
  const error = ref<E | null>(null) as Ref<E>;
  const loading = ref<boolean>(true);
  const cacheLifetime: number = options?.cacheLifetime || 1000 * 60 * 60; // one hour
  const cacheName: string = options?.cacheName || url;
  const { isDataOutdated: isCacheOutdated, error: cacheError } = useCache(
    cacheName,
    data,
    {
      lifetime: cacheLifetime,
    },
  );
  const emits: FetchAndCacheEmitter = options?.emits || dummyEmits;

  const loaded = () => {
    loading.value = false;
    emits('loaded', data.value);
  };

  const fetchData = () => {
    loading.value = true;
    emits('loading');
    console.log('Fetching from URL', url);
    fetch(url)
      .then((response) => {
        if (!response.ok) {
          throw new Error(response.statusText);
        }
        return response.json() as Promise<T>;
      })
      .then((responseData) => {
        data.value = responseData;
        loaded();
      })
      .catch((e) => {
        console.warn('Caught error!', e);
        error.value = e;
        emits('error', e);
      })
      .finally(() => (loading.value = false));
  };

  if (isCacheOutdated.value || cacheError.value != null) {
    fetchData();
  } else {
    loaded();
  }
  return { data, loading, error };
};
