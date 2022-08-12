<template>
  <nav>
    <ul class="nav-links nav-local">
      <ContentList :path="uri" v-slot="{ list }">
        <li v-for="page in list" :key="page._path" class="nav-link">
          <NuxtLink :to="page._path">
            <span class="nav-icon">
              <font-awesome-icon :icon="page.faIcon" v-if="page.faIcon" />
            </span>
            <span class="nav-text">{{ page.navTitle || page.title }}</span>
          </NuxtLink>
        </li>
      </ContentList>
    </ul>
    <ul class="nav-links nav-global">
      <ContentNavigation v-slot="{ navigation }">
        <li class="nav-link" v-for="page in navigation" :key="page._path">
          <NuxtLink :to="page._path">
            <span class="nav-icon">
              <font-awesome-icon :icon="page.faIcon" v-if="page.faIcon" />
            </span>
            <span class="nav-text">{{ page.navTitle || page.title }}</span>
          </NuxtLink>
        </li>
      </ContentNavigation>
    </ul>
  </nav>
</template>

<script lang="ts">
export default {
  data() {
    return {
      directory: '',
      uri: '',
    };
  },
  mounted() {
    this.uri = window.location.pathname;
    this.directory = this.uri.replace(/[^/]+$/g, '');
    console.log(`URI: ${this.uri}, DIRECTORY: ${this.directory}`);
  },
  methods: {
    rootPage() {
      return this.directory == '' || this.directory == '/';
    },
  },
};
</script>

<style lang="scss">
@import 'node_modules/nord/src/sass/nord.scss';
</style>
