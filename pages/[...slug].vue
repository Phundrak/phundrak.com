<template>
  <Transition>
    <div>
      <pre>{{ $route.path }}, {{ localePath($route.path) }}</pre>
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

<style lang="scss">
@import "node_modules/nord/src/sass/nord.scss";

@import url("https://fonts.googleapis.com/css2?family=Noto+Sans&family=Noto+Sans+Mono&display=swap");

main {
  margin: 0;
  padding: 0 6rem;
  transition: padding-left 0.3s;
  height: 100vh;

  div {
    padding-bottom: 2.5rem;
  }
}

h1,
h2,
h3,
h4,
h5 {
  font-family: "Noto Sans Mono", monospace;

  a {
    box-shadow: none;

    &:after {
      content: " §";
    }

    &:hover {
      box-shadow: none;
    }
  }
}

h1,
h5 {
  color: $nord10;
}

h2 {
  color: $nord8;
}

h3 {
  color: $nord7;
}

h4 {
  color: $nord9;
}

@media screen and (min-width: 1300px) {
  main {
    padding-left: 18rem;
  }
}
</style>
