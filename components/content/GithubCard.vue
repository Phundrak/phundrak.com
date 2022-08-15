<template>
  <div v-if="data">
    <a :href="data.html_url">
      <div class="github-card">
        <h3>{{ data.name }}</h3>
        <h4>author: {{ data.owner.login }}</h4>
        <p>
          {{ data.description }}
        </p>
      </div>
    </a>
  </div>
  <div v-else>Loading repo information</div>
</template>

<script lang="ts">
export default {
  props: {
    repo: {
      type: Object as PropType<GithubRepo>,
      required: false,
    },
    url: String,
  },
  data() {
    return {
      data: null,
    };
  },
  mounted() {
    this.data = this.repo;
    if (!this.data) {
      if (!this.url) {
        console.log("Cannot retrieve information for non-existant repo!");
      } else {
        const fetchUrl = "https://api.github.com/repos/" + this.url;
        console.log("Fetch URL: ", fetchUrl);
        fetchApi<GithubRepo>(fetchUrl)
          .then((data) => (this.data = data))
          .catch((error) =>
            console.log(`Error fetching ${this.url}: ${error}`)
          );
      }
    }
  },
};
</script>

<style lang="scss">
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

a {
  text-decoration: none;
  color: inherit;
}
</style>
