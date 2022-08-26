<template>
  <Transition>
    <div class="content">
      <ContentDoc :path="localePath($route.path)">
        <template #not-found>
          <h1>I don’t know what you mean</h1>
          <p>
            Looks like the path
            <code>{{ path }}</code>
            (internal
            <code>{{ $route.path }}</code>
            ) does not exist here.
          </p>
          <button class="link-back" @click="back">Take me back!</button>
        </template>
      </ContentDoc>
    </div>
  </Transition>
</template>

<script lang="ts">
export default {
  data() {
    return {
      path: null,
    };
  },
  methods: {
    back() {
      this.$router.go(-1);
    },
  },
  // computed: {
  //   // currentLocale() {
  // //   return this.$i18n.locale;
  // // },
  //   // path() {
  // //   const localePath = localePath(this.$route.path);
  // //   console.log("LOCALE PATH:", localePath);
  // //   return localePath;
  // // },
  // },
};
</script>
