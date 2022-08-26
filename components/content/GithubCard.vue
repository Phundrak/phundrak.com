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

<style lang="scss" scoped>
@import "node_modules/nord/src/sass/nord.scss";

.github-card {
  width: min(400px, 90%);
  border-radius: 12px;
  padding: 1em;
  background: $nord3;
  margin: 20px auto;
  box-shadow: 5px 5px 5px 3px $nord0;

  h3 {
    font-size: 1.5em;
  }

  h4 {
    font-weight: normal;
    font-size: 1.2em;
  }
}
</style>
