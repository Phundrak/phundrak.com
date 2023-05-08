<template>
  <div class="githubRepo flex-row flex-space-between gap-1rem rounded-corners">
    <div class="flex-col info">
      <h3>{{ props.repo.name }}</h3>
      <div>
        <p>
          {{ props.repo.description }}
        </p>
      </div>
    </div>
    <div class="flex-col flex-start gap-1rem stats">
      <p>Stars: {{ repo.stargazers_count }}</p>
      <p>Forks: {{ repo.forks_count }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { GithubRepo } from '../../composables/github';
import { PropType } from 'vue';
const props = defineProps({
  repo: Object as PropType<GithubRepo>,
});
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
