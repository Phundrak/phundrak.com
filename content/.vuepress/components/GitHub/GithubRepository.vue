<template>
  <div
    class="githubRepo flex-col flex-space-between gap-1rem rounded-corners card-width"
  >
    <ApiLoader
      :cache-name="repoName()"
      :url="fetchUrl"
      :already-known-data="props.data"
      @loaded="(repo: GithubRepo) => (repository = repo)"
    >
      <h3>{{ repository?.name }}</h3>
      <div>
        <p>
          {{ repository?.description }}
        </p>
      </div>
      <div class="flex-row flex-start gap-1rem stats">
        <div class="stars">
          <Icon name="star" /> {{ repository?.stargazers_count }}
        </div>
        <div class="forks">
          <Icon name="fork" /> {{ repository?.forks_count }}
        </div>
        <div class="link">
          <a :href="repository?.html_url"><i class="icon phunic-link" /></a>
        </div>
      </div>
    </ApiLoader>
  </div>
</template>

<script setup lang="ts">
import ApiLoader from '../ApiLoader.vue';

import { GithubRepo } from '../../types/github';
import { PropType, Ref, ref } from 'vue';

const props = defineProps({
  data: Object as PropType<GithubRepo>,
  repoName: String,
});

const repoName = (): string => {
  return props.data ? props.data.full_name : props.repoName;
};

const fetchUrl = `https://api.github.com/repos/${repoName()}`;
const repository: Ref<GithubRepo | null> = ref(null);
</script>

<style lang="less">
@import 'node_modules/nord/src/lesscss/nord.less';
@import '../../styles/classes.less';

.githubRepo {
  padding: 2rem;
  background-color: @nord4;
  align-self: auto;
  h3,
  h3:first-child {
    margin: 0;
    padding: 0;
  }

  html.dark & {
    background-color: @nord3;
  }

  .info {
    max-width: 30rem;
  }

  .stats {
    width: 4rem;

    div {
      .flex-row();
      gap: 0.3rem;
    }
  }

  .link {
    a {
      display: flex;
      align-items: center;
    }
  }
}
</style>
