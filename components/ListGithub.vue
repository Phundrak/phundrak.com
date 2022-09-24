<template>
  <section class="repos">
    <div v-if="repos && repos.length">
      <div v-for="repo in repos" :key="repo.id" class="repo">
        <GithubCard :repo="repo" class="card" />
      </div>
    </div>
    <div v-else>Loading repos</div>
  </section>
</template>

<script setup lang="ts">
import { GithubRepo } from "~~/composables/github";
import { useFetch } from "#app";

const props = defineProps<{
  username: string;
  keep: number;
  type: string;
}>();

const latestRepo = (a: GithubRepo, b: GithubRepo) => {
  return a.updated_at < b.updated_at ? 1 : a.updated_at > b.updated_at ? -1 : 0;
};

const mostStars = (a: GithubRepo, b: GithubRepo) => {
  return a.stargazers_count < b.stargazers_count
    ? 1
    : a.stargazers_count > b.stargazers_count
    ? -1
    : 0;
};

const mostForks = (a: GithubRepo, b: GithubRepo) => {
  return a.stargazers_count < b.stargazers_count
    ? 1
    : a.stargazers_count > b.stargazers_count
    ? -1
    : 0;
};

const fetchUrl = `https://api.github.com/users/${props.username}/repos`;
const { data: repos } = await useFetch<GithubRepo[]>(fetchUrl, {
  transform: function (data: GithubRepo[]): GithubRepo[] {
    return data
      .filter((repo) => repo.name != "phundrak")
      .sort(function (a: GithubRepo, b: GithubRepo) {
        if (props.type === "latest") {
          return latestRepo(a, b);
        }
        if (props.type === "most-stars") {
          return mostStars(a, b);
        }
        return mostForks(a, b);
      })
      .splice(0, props.keep);
  },
});
</script>

<style lang="scss">
@import "~/assets/mixins.scss";

.repos > div {
  @include flex-row(2rem);
  flex-wrap: wrap;
  justify-content: space-around;
  align-items: stretch;
}

.repo {
  flex-grow: 1;
  height: 100%;
}

.card {
  height: 100%;
}
</style>
