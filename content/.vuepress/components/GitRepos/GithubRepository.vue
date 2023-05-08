<template>
  <div class="githubRepo flex-row flex-space-between gap-1rem rounded-corners">
    <ApiLoader
      :cache-name="repoName()"
      :url="fetchUrl"
      :already-known-data="props.data"
      @data-loaded="(repo: GithubRepo) => (repository = repo)"
    >
      <div class="flex-col info">
        <h3>{{ props.data.name }}</h3>
        <div>
          <p>
            {{ props.data.description }}
          </p>
        </div>
      </div>
      <div class="flex-col flex-start gap-1rem stats">
        <p>Stars: {{ data.stargazers_count }}</p>
        <p>Forks: {{ data.forks_count }}</p>
      </div>
    </ApiLoader>
  </div>
</template>

<script setup lang="ts">
import ApiLoader from '../ApiLoader.vue';

import { GithubRepo } from '../../composables/github';
import { PropType, Ref, ref } from 'vue';

const props = defineProps({
  data: Object as PropType<GithubRepo>,
  repoName: String,
  fetcher: String as PropType<'gitea' | 'github'>,
});

const repoName = (): string => {
  return props.data ? props.data.full_name : props.repoName;
};

const fetchUrl = `https://api.github.com/repos/${repoName()}`;
const repository: Ref<GithubRepo> = ref(null);
</script>

<style lang="less">
@import 'node_modules/nord/src/lesscss/nord.less';
@import '../../styles/classes.less';

.githubRepo {
  width: 35rem;
  padding: 3rem;
  background-color: @nord4;

  html.dark & {
    background-color: @nord3;
  }

  .info {
    max-width: 30rem;
  }

  .stats {
    width: 4rem;
  }
}
</style>
