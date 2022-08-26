<template>
  <div v-if="repository">
    <a :href="repository.html_url">
      <div class="github-card">
        <h3>{{ repository.name }}</h3>
        <h4>author: {{ repository.owner.login }}</h4>
        <p>
          {{ repository.description }}
        </p>
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
