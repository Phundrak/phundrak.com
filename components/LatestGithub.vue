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

function api<T>(url: string): Promise<T> {
  return fetch(url).then((response) => {
    if (!response.ok) {
      throw new Error(response.statusText);
    }
    return response.json();
  });
}

export default {
  props: ["username", "latestN"],
  data() {
    return {
      repos: [],
    };
  },
  mounted() {
    console.log("Loading repos from Github");
    console.log("Username: " + this.username);
    console.log("Number of repos: " + this.latestN);

    const fetchUrl: string = `https://api.github.com/users/${this.username}/repos`;

    console.log("Fetching repos");
    api<GithubRepo[]>(fetchUrl)
      .then((data) => {
        console.log(data);
        data = data
          .sort((a, b) => {
            return a.pushed_at > b.pushed_at
              ? -1
              : a.pushed_at < b.pushed_at
              ? 1
              : 0;
          })
          .filter((repo) => repo.name != "phundrak");
        data.forEach((repo) => {
          console.log(repo.name);
        });
        this.repos = data.splice(0, this.latestN);
      })
      .catch((error) => console.log("Error: " + error));
  },
};
</script>
