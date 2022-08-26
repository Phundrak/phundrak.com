<template>
  <div id="latest-github-repos">
    <div v-if="repos.length">
      <div v-for="repo in repos" :key="repo.id" class="repo">
        <GithubCard :repo="repo" />
      </div>
    </div>
    <div v-else>Loading repos</div>
  </div>
</template>

<script setup lang="ts">
import { GithubRepo } from "~~/composables/github";

const props = defineProps<{
  username: string;
  latestN: number;
}>();

const fetchUrl = `https://api.github.com/users/${props.username}/repos`;
const { data: repos } = await useFetch<GithubRepo[]>(fetchUrl, {
  transform: function (data: GithubRepo[]): GithubRepo[] {
    return data
      .filter((repo) => repo.name != "phundrak")
      .splice(0, props.latestN);
  },
});
</script>
