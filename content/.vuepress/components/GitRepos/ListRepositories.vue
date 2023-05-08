<template>
  <ApiLoader :url="fetchUrl" @dataLoaded="filterRepos">
    <div class="flex-col gap-1rem list-repos">
      <GithubRepository
        :repo="repo"
        type="repositories"
        v-for="repo in repos"
        class="center"
      />
    </div>
  </ApiLoader>
</template>

<script setup lang="ts">
import { PropType, Ref, ref } from 'vue';
import { GithubRepo } from '../../composables/github';
import GithubRepository from './GithubRepository.vue';
const props = defineProps({
  sortBy: {
    default: 'pushed_at',
    required: false,
    type: String as PropType<'stars' | 'forks' | 'pushed_at'>,
  },
  user: {
    default: '',
    required: true,
    type: String,
  },
  limit: {
    default: 5,
    required: false,
    type: Number,
  },
});

let repos: Ref<GithubRepo[]> = ref(null);

const fetchUrl = `https://api.github.com/users/${props.user}/repos?per_page=100`;

const filterRepos = (response: GithubRepo[]) => {
  repos.value = response
    .sort((a, b) => {
      if (props.sortBy === 'stars') {
        return b.stargazers_count - a.stargazers_count;
      }
      if (props.sortBy === 'pushed_at') {
        const dateA = new Date(a.pushed_at);
        const dateB = new Date(b.pushed_at);
        return dateB.getTime() - dateA.getTime();
      }
      return b.forks_count - a.forks_count;
    })
    .slice(0, +props.limit);
};
</script>

<style lang="less">
@import '../../styles/classes.less';

.list-repos {
  margin: 2rem auto;
}
</style>
