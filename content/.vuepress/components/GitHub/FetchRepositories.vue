<template>
  <ApiLoader :url="fetchUrl" @loaded="filterRepos" cache-name="repos" />
  <slot />
</template>

<script setup lang="ts">
import { PropType, ref } from 'vue';
import { GithubRepo } from '../../types/github';
const props = defineProps({
  sortBy: {
    default: 'none',
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
const emits = defineEmits(['loaded']);
const fetchUrl = `https://api.github.com/users/${props.user}/repos?per_page=100`;
const repos = ref<GithubRepo[]>([]);

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
  emits('loaded', repos.value);
};
</script>
