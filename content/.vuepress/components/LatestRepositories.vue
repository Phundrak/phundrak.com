<template>
  <div v-if="error">
    {{ error }}
  </div>
  <div v-else v-for="repo in githubRepos">
    <p>{{ repo.name }} updated at {{ repo.updated_at }}</p>
  </div>
</template>

<script setup lang="ts">
import { readFromCache } from '../composables/cache';
import {
  GithubError,
  GithubRepo,
  getLatestRepositories,
} from '../composables/github';

let githubRepos: GithubRepo[] | null = null;
let error: GithubError | null;
const getRepositories = () => {
  return getLatestRepositories('phundrak', 5);
};

readFromCache<GithubRepo[]>('latestRepos', getRepositories).subscribe({
  next: (repos: GithubRepo[]) => {
    githubRepos = repos;
    error = null;
  },
  error: (errorResponse: GithubError) => {
    githubRepos = null;
    error = errorResponse;
  },
});
</script>

<style lang="less"></style>
