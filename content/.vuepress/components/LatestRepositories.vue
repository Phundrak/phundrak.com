<template>
  <div v-if="githubRepos && githubRepos.length > 0">
    <div v-for="repo in githubRepos">
      <GithubRepository :repo="repo" v-for="repo in githubRepos" />
    </div>
  </div>
  <p v-else>Error: {{ error }}</p>
</template>

<script setup lang="ts">
import { ref, Ref } from 'vue';
import { readFromCache } from '../composables/cache';
import {
  GithubError,
  GithubRepo,
  getLatestRepositories,
} from '../composables/github';

let githubRepos: Ref<GithubRepo[]> = ref(null);
let error: Ref<GithubError> = ref(null);
const getRepositories = () => {
  return getLatestRepositories('phundrak', 5);
};

// TODO: Cache all repositories and not just these
readFromCache<GithubRepo[]>('latestRepos', getRepositories).subscribe({
  next: (repos: GithubRepo[]) => {
    console.log('Received repos:', repos);
    githubRepos.value = repos;
    error.value = null;
  },
  error: (errorResponse: GithubError) => {
    githubRepos.value = null;
    error.value = errorResponse;
  },
});
</script>

<style lang="less">
@import '../styles/classes.less';
.repositories {
  margin: 2rem auto;
}
</style>
