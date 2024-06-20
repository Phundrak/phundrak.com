<template>
  <div class="list-repos flex-col gap-1rem">
    <FetchRepositories
      v-if="props.user !== ''"
      :sort-by="props.sortBy"
      :user="props.user"
      :limit="props.limit"
      @loaded="(response: GithubRepo[]) => (repos = response)"
    >
      <GithubRepository
        :data="repo"
        type="repositories"
        v-for="repo in repos"
      />
    </FetchRepositories>
    <slot v-else />
  </div>
</template>

<script setup lang="ts">
import FetchRepositories from './FetchRepositories.vue';
import GithubRepository from './GithubRepository.vue';

import { PropType, Ref, ref } from 'vue';
import { GithubRepo } from '../../types/github';

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
