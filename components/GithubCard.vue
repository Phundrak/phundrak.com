<template>
  <Card v-if="repository">
    <template #title>
      <h3>{{ repository.name }}</h3>
      <span class="author">{{ repository.owner.login }}</span>
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
  <ErrorCard v-else-if="error">
    <template #info>
      Cannot retrieve information for repository {{ props.id }}
    </template>
    <template #cause>
      {{ error }}
    </template>
  </ErrorCard>
  <Card v-else-if="pending">
    <template #loading>
      <Loader />
      <p>Loading repository information</p>
    </template>
  </Card>
  <Card v-else>
    <template #error>
      <span class="error-info">Github Card in an unknown state</span>
      <span class="error-cause">
        This card is neither waiting for data, nor does it have any. If you see
        this happen, please report this as a new issue on the website’s Git
        repository (see the link in the footer) or by email. Thank you.
      </span>
    </template>
  </Card>
</template>

<script lang="ts" setup>
import { GithubRepo } from "~/composables/github";
import { useFetch } from "#app";
import axios from "axios";

const props = defineProps<{
  id?: string;
  repo?: GithubRepo;
}>();

let repository = ref(null);
let error = ref(null);
let pending = ref(true);

if (props.repo) {
  repository.value = props.repo;
} else {
  const fetchUrl = "https://api.github.com/repos/" + props.id;
  ({ data: repository, error, pending } = await useFetch(fetchUrl));
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
