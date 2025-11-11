<template>
  <NuxtLayout v-if="page" :name="page.meta?.layout ?? 'default'">
    <ContentRenderer :value="page" />
  </NuxtLayout>
  <div v-else>
    <h1>Page not found</h1>
    <p>This page doesn&apos;t exist in {{ locale }} language.</p>
  </div>
</template>

<script setup lang="ts">
const { getPageContent } = useDataJson('page');
const page = await getPageContent();

// Pre-fetch JSON data for MDC components to avoid hydration issues
const { getJsonData } = useDataJson('page-data');
const pageData = await getJsonData();
// Provide data to child MDC components
provide('pageData', pageData);

useMeta({ title: page.value?.title, description: page.value?.description });
</script>
