import { withLeadingSlash } from 'ufo';
import type { Collections } from '@nuxt/content';

export const useDataJson = (prefix: string) => {
  const route = useRoute();
  const { locale } = useI18n();
  const slug = computed(() => {
    // Use route.params.slug for dynamic routes, or route.path for static routes
    const slugValue = route.params.slug || route.path;
    return withLeadingSlash(String(slugValue));
  });
  const key = computed(() => prefix + '-' + slug.value);

  const getData = async <T>(
    collectionPrefix: string,
    options: {
      useFilter?: boolean;
      fallbackToEnglish?: boolean;
      extractMeta?: boolean;
    } = {},
  ) => {
    const { useFilter = false, fallbackToEnglish = false, extractMeta = false } = options;

    const { data } = await useAsyncData(
      key.value,
      async () => {
        const collection = (collectionPrefix + locale.value) as keyof Collections;

        let content;
        if (useFilter) {
          // For data collections, use .all() and filter
          const allData = await queryCollection(collection).all();
          content = allData.filter((source) => source.meta.path == slug.value)[0];
        } else {
          // For page collections, use .path().first()
          content = await queryCollection(collection).path(slug.value).first();
          if (!content && fallbackToEnglish && locale.value !== 'en') {
            content = await queryCollection('content_en').path(slug.value).first();
          }
        }

        return extractMeta ? content?.meta : content;
      },
      {
        watch: [locale], // Automatically refresh when locale changes
      },
    );
    return data as Ref<T | null>;
  };

  const getJsonData = async (collectionPrefix: string = 'content_data_') => {
    return getData(collectionPrefix, { useFilter: true, extractMeta: true });
  };

  const getPageContent = async (collectionPrefix: string = 'content_', fallbackToEnglish: boolean = true) => {
    return getData(collectionPrefix, { fallbackToEnglish });
  };

  const getCachedData = () => {
    const { data } = useNuxtData(key.value);
    return data;
  };

  return { getJsonData, getPageContent, getCachedData };
};
