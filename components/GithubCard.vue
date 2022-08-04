<template>
  <NuxtLink :to="repo.html_url">
    <div class="github-card">
      <h3>{{ repo.name }}</h3>
      <p>
        {{ repo.description }}
      </p>
    </div>
  </NuxtLink>
</template>

<script lang="ts">
import { GithubRepo } from "~~/composables/github";
import { PropType } from "vue";
export default {
  props: {
    repo: {
      type: Object as PropType<GithubRepo>,
      required: true,
    },
  },
};
</script>
