<template>
  <Card v-if="repository">
    <template #title>
      <h3>{{ repository.name }}</h3>
    </template>
    <template #content>
      <p>{{ repository.description }}</p>
      <div class="stats">
        <span class="lang">
          <Icon icon="code" />
          {{ repository.language }}
        </span>
        <span class="stars">
          <Icon icon="star" />
          {{ repository.stargazers_count }}
        </span>
        <span class="forks">
          <Icon icon="code-branch" />
          {{ repository.forks_count }}
        </span>
      </div>
    </template>
  </Card>
  <Card v-else>
    <template #loading>
      <Loader />
      <p>Loading repository information</p>
    </template>
  </Card>
</template>

<script lang="ts" setup>
import { GithubRepo } from "~/composables/github";
import axios from "axios";

const props = defineProps<{
  id?: string;
  repo?: GithubRepo;
}>();

let repository = ref(null);

if (props.repo) {
  repository.value = props.repo;
} else {
  const fetchUrl = "https://api.github.com/repos/" + props.id;
  axios.get(fetchUrl).then((data) => (repository.value = data.data));
}
</script>

<style scoped lang="scss">
@import "~/assets/mixins.scss";

.github-card {
  @include flex-col;
  padding: 1rem;
  width: 25rem;
  border-radius: 0.4rem;
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
