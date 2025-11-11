<template>
  <NuxtLayout name="default">
    <h1 class="text-4xl text-highlighted font-bold mb-8">
      {{ $t('pages.resume.name') }}
    </h1>
    <UPageCard class="bg-background-100 my-10">
      <p>
        {{ $t('pages.resume.experience') }}
      </p>
      <UTimeline v-model="valueExp" reverse :items="resumeContent?.experience" class="w-full">
        <template #description="{ item }">
          <div class="flex flex-col gap-2">
            <p>
              {{ item.description }}
            </p>
            <UiBadgeList :tools="item.tools" />
          </div>
        </template>
      </UTimeline>
    </UPageCard>
    <UPageCard class="bg-background-100 my-10">
      <p>
        {{ $t('pages.resume.education') }}
      </p>
      <UTimeline v-model="valueEd" reverse :items="resumeContent?.education" class="w-full" />
    </UPageCard>
    <UiBadgeListCard :tools="resumeContent?.otherTools">{{ $t('pages.resume.tools') }}</UiBadgeListCard>
    <UiBadgeListCard :tools="resumeContent?.devops">{{ $t('pages.resume.devops') }}</UiBadgeListCard>
    <UiBadgeListCard :tools="resumeContent?.os">{{ $t('pages.resume.os') }}</UiBadgeListCard>
    <UiBadgeListCard :tools="resumeContent?.programmingLanguages">{{
      $t('pages.resume.programmingLanguages')
    }}</UiBadgeListCard>
    <UiBadgeListCard :tools="resumeContent?.frameworks">{{ $t('pages.resume.frameworks') }}</UiBadgeListCard>
  </NuxtLayout>
</template>

<script setup lang="ts">
useMeta({
  title: $t('pages.resume.name'),
  description: $t('pages.resume.description'),
});
const { getJsonData } = useDataJson('resume');
const resumeContent = await getJsonData();
const arrLength = (array?: T[]) => (array ? array.length - 1 : 0);
const valueExp = computed(() => arrLength(resumeContent.value?.experience));
const valueEd = computed(() => arrLength(resumeContent.value?.education));
</script>
