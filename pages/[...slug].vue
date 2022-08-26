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
@import url('https://fonts.googleapis.com/css2?family=Noto+Sans&family=Noto+Sans+Mono&display=swap');

.content {
  padding: 2rem 4rem;
}
</style>

<style lang="scss">
@import 'node_modules/nord/src/sass/nord.scss';

@function repeat-str($str, $nbr) {
  $outstr: '';
  @for $i from 1 through $nbr {
    $outstr: $outstr + $str;
  }
  @return $outstr;
}

.content {
  @for $i from 1 through 6 {
    h#{$i} {
      font-family: 'Noto Sans Mono', monospace;
      color: $nord7;

      a {
        text-decoration: none;
        box-shadow: none;
      }

      &::before {
        content: repeat-str('§', $i) + ' ';
      }
    }
  }
}
</style>
