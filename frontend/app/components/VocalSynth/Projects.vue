<template>
  <UPageCard class="bg-background-100 my-10">
    <p class="text-xl">
      {{ $t('pages.vocal-synthesis.projects') }}
    </p>
    <div class="flex flex-col max-w gap-10">
      <div v-for="project in data?.projects" :key="project.title" class="flex flex-row max-w gap-5">
        <div>
          <div
            class="bg-primary text-text-50 dark:bg-primary p-1 rounded-md min-w-13 w-13 h-13 min-h-13 flex justify-center my-2"
          >
            <UIcon :name="project.icon" class="size-11" />
          </div>
        </div>
        <div class="flex flex-col">
          <div class="flex flex-row gap-2 items-baseline">
            <ULink :to="project.link" class="text-2xl">
              {{ project.title }}
            </ULink>
            <UIcon v-if="external(project.link)" name="mdi:link" class="size-5" />
          </div>
          <div>
            {{ project.description }}
          </div>
        </div>
      </div>
    </div>
  </UPageCard>
</template>

<script setup lang="ts">
// Inject data provided by the page to avoid hydration issues with MDC components
const data = inject('pageData');
const external = (url: string) => url.startsWith('http');
</script>
