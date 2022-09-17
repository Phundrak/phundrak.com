<template>
  <div v-if="repository">
    <a :href="repository.html_url">
      <div class="github-card">
        <div class="info">
          <div class="name">{{ repository.name }}</div>
          <div class="author">{{ repository.owner.login }}</div>
        </div>
        <p>
          {{ repository.description }}
        </p>
        <div class="stats">
          <span class="lang">
            <Icon icon="code" /> {{ repository.language }}
          </span>
          <span class="stars">
            <Icon icon="star" /> {{ repository.stargazers_count }}
          </span>
          <span class="forks">
            <Icon icon="code-branch" /> {{ repository.forks_count }}
          </span>
        </div>
      </div>
    </a>
  </div>
  <div v-else>Loading repo information</div>
</template>

<script lang="ts" setup>
import { GithubRepo } from "~/composables/github";
const props = defineProps<{
  id?: string;
  repo?: GithubRepo;
}>();

let repository = props.repo;
if (repository === undefined) {
  const fetchUrl = "https://api.github.com/repos/" + props.id;
  const { data } = await useFetch<GithubRepo>(fetchUrl);
  repository = data.value;
}
</script>

<style scoped lang="scss">
@use "sass:color";
@import "~/assets/mixins.scss";

.github-card {
  @include flex-col;
  @include background-theme($nord3, $nord4);
  @include shadow-theme(0.2rem 0.2rem 0.4rem, $nord0, $nord3);
  padding: 1rem;
  width: 25rem;
  border-radius: 0.4rem;

  &:hover {
    @include shadow-theme(0.5rem 0.5rem 1rem, $nord0, $nord3);
  }
}

.stats,
.info {
  @include flex-row;
  flex-wrap: nowrap;
  gap: 1rem;
  justify-content: space-between;
}

.name,
.code {
  flex-grow: 1;
}

.info {
  justify-content: space-between;
}

.name {
  font-size: 2rem;
  @include font-mono;
  color: $nord8;
}

.icon {
  padding: 0.3rem;
}
</style>
