<template>
  <Teleport to="#nav-local">
    <h3>{{ title }}</h3>
    <ul class="nav-links">
      <ContentList :path="path" v-slot="{ list }">
        <li v-for="page in list" :key="page._path" class="nav-link">
          <IconLink :icon="page.faIcon" :to="page._path">
            {{ page.navTitle || page.title }}
          </IconLink>
        </li>
      </ContentList>
    </ul>
  </Teleport>
</template>

<script lang="ts">
export default {
  props: {
    path: String,
  },
  data() {
    return {
      title: '',
    };
  },
  mounted() {
    this.title = this.path.charAt(0).toUpperCase() + this.path.slice(1);
    console.log('PAAATH: ' + this.path);
    console.log('TITLE: ' + this.title);
  },
};
</script>
