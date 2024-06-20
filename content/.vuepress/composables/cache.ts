import { Ref, computed, ref, watchEffect } from 'vue';

interface CacheOptions {
  lifetime?: number;
  timestampSuffix?: string;
  forceUpdate?: boolean;
}

/**
 * Cache data in local storage.
 *
 * The cache is updated if:
 * - cache data does not exist
 * - cached data is outdated and `data` is not null
 * - or `options.forceUpdate` is true, regardless of the value of `data`
 *
 * Otherwise, data is retrieved from cache.
 *
 * @param {string} name Name of the cached value in local storage
 * @param {Ref<T>} data Data to cache
 * @param {CacheOptions} options Tweaks to the behaviour of the function
 */
export const useCache = <T>(
  name: string,
  data: Ref<T>,
  options: CacheOptions,
) => {
  const error = ref<string>(null);
  const timestampName = name + (options?.timestampSuffix || '-timestamp');
  const lifetime = options?.lifetime || 1000 * 60 * 60; // one hour in milliseconds
  const lastUpdated: number = +localStorage.getItem(timestampName);
  const cacheAge: number = Date.now() - lastUpdated;
  const isDataOutdated = computed(() => {
    return cacheAge > lifetime;
  });
  const shouldUpdate = computed(
    () => options?.forceUpdate || (isDataOutdated.value && data.value != null),
  );

  const setData = () => {
    console.log('Setting data in cache with name', name);
    localStorage.setItem(name, JSON.stringify(data.value));
    localStorage.setItem(timestampName, `${Date.now()}`);
  };

  const getData = () => {
    console.log('Getting data from cache with name', name);
    const cached = localStorage.getItem(name);
    console.log('Value from storage:', cached);
    try {
      data.value = JSON.parse(cached);
    } catch (err) {
      console.error('Failed to parse cached data:', err);
      data.value = null;
      error.value = err;
    }
  };

  getData();
  watchEffect(() => (shouldUpdate.value ? setData() : getData()));
  return { error, isDataOutdated };
};
