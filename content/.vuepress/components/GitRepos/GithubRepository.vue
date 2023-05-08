<template>
  <div class="githubRepo flex-col rounded-corners">
    <h4>{{ props.repo.name }}</h4>
    <div class="flex-row space-between">
      <p>{{ props.repo.description }}</p>
      <div class="gap-1rem flex-end">
        <p>Stars: {{ repo.stargazers_count }}</p>
        <p>Forks: {{ repo.forks_count }}</p>
      </div>
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
  max-width: 35rem;
  padding: 3rem;
  background-color: @nord3;
}
</style>
