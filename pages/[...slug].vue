<template>
  <main>
    <ContentDoc>
      <template #not-found>
        <h1>I don’t know what you mean</h1>
        <p>Looks like the path {{ $route.path }} does not exist here.</p>
        <button class="link-back" @click="back">Take me back!</button>
      </template>
    </ContentDoc>
  </main>
</template>

<script lang="ts">
export default {
  methods: {
    back() {
      this.$router.go(-1);
    },
  },
};
</script>
