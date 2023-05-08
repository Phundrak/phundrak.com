<template>
  <div v-if="props.user !== ''" class="list-repos flex-col gap-1rem">
    <FetchRepositories
      :sort-by="props.sortBy"
      :user="props.user"
      :limit="props.limit"
      @data-loaded="(response: GithubRepo[]) => (repos = response)"
    />
    <GithubRepository :data="repo" type="repositories" v-for="repo in repos" />
  </div>
  <div v-else class="list-repos flex-col gap-1rem">
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import FetchRepositories from './FetchRepositories.vue';
import GithubRepository from './GithubRepository.vue';

import { PropType, Ref, ref } from 'vue';
import { GithubRepo } from '../../composables/github';

const props = defineProps({
  sortBy: {
    default: 'none',
    required: false,
    type: String as PropType<'stars' | 'forks' | 'pushed_at'>,
  },
  user: {
    default: '',
    required: false,
    type: String,
  },
  limit: {
    default: 5,
    required: false,
    type: Number,
  },
});

const repos: Ref<GithubRepo[]> = ref(null);
</script>

<style lang="less">
@import '../../styles/classes.less';

.list-repos {
  margin: 2rem auto;
}
</style>
