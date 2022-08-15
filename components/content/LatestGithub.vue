<template>
  <div id="latest-github-repos">
    <slot />
    <div v-if="repos.length">
      <div v-for="repo in repos" :key="repo.id" class="repo">
        <GithubCard :repo="repo" />
      </div>
    </div>
    <div v-else>Loading repos</div>
  </div>
</template>

<script lang="ts">
import { GithubRepo } from "~/composables/github";
import { fetchApi } from "~~/composables/api";

export default {
  props: ["username", "latestN"],
  data() {
    return {
      repos: [],
    };
  },
  mounted() {
    const fetchUrl: string = `https://api.github.com/users/${this.username}/repos`;
    fetchApi<GithubRepo[]>(fetchUrl)
      .then((data) => {
        this.repos = data
          .sort((a, b) => {
            return a.pushed_at > b.pushed_at
              ? -1
              : a.pushed_at < b.pushed_at
              ? 1
              : 0;
          })
          .filter((repo) => repo.name != "phundrak")
          .splice(0, this.latestN);
      })
      .catch((error) => console.log("Error: " + error));
  },
};
</script>
