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
};
</script>

<style lang="scss" scoped>
@import "~/assets/mixins.scss";
.content {
  padding: 2rem;
  width: 90%;
  max-width: 800px;
  margin: 0 auto;
}

@include small-screen {
  .content {
    padding: 1rem;
  }
}
</style>

<style lang="scss">
@import "~/assets/mixins.scss";

.content {
  @for $i from 1 through 6 {
    h#{$i} {
      @include font-mono;
      @include title-ruler($right: true, $color: $nord7, $margin: 0 0 0 0);
      gap: 0.5em;

      &::before {
        content: repeat-str("§", $i) + " ";
      }

      a {
        @include plain-link;
      }
    }
  }
}
</style>
