<template>
  <div v-if="githubRepos && githubRepos.length > 0">
    <div v-for="repo in githubRepos">
      <p>{{ repo.name }} updated at {{ repo.updated_at }}</p>
    </div>
  </div>
  <p v-else>Erreur: {{ error }}</p>
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

<style lang="less"></style>
