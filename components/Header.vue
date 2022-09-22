<template>
  <nav @mouseleave="closeMenu">
    <ul class="nav-links">
      <li class="nav-link">
        <IconButton icon="bars" class="menu-button" @click="toggleMenu">
          Menu
        </IconButton>
      </li>
    </ul>
    <div id="nav-global" @click="closeMenu">
      <ul class="nav-links">
        <li v-for="link of pages" :key="link._page" class="nav-link">
          <IconLink :icon="link.icon" :to="localePath(link._path)">
            {{ link.navTitlo || link.title }}
          </IconLink>
        </li>
      </ul>
    </div>
    <ul id="bottom-items" class="nav-links">
      <li v-for="locale in availableLocales" :key="locale.code">
        <IconLink :to="switchLocalePath(locale.code)" icon="language">
          {{ locale.name }}
        </IconLink>
      </li>
      <li id="theme" class="nav-link" @click="closeMenu">
        <IconButton icon="theme" @click="toggleDark()">Theme</IconButton>
      </li>
    </ul>
  </nav>
</template>

<style lang="scss">
@import "~/assets/mixins.scss";

@mixin opacity-transform($opacity) {
  filter: opacity($opacity);
}

nav {
  box-sizing: content-box;

  .nav-text,
  h3 {
    @include opacity-transform(0%);
  }

  &.open {
    .nav-text,
    h3 {
      @include opacity-transform(100%);
    }
  }
}

@include large-screen {
  nav {
    .nav-text,
    h3 {
      @include opacity-transform(100%);
    }
  }
}
</style>

<style lang="scss" scoped>
@use "sass:color";
@import "~/assets/mixins.scss";

$width-menu-open: 18rem;
$width-menu-collapsed: 5rem;

h3 {
  text-align: center;
}

.nav-links {
  @include flex-col();
  gap: 0.5rem;

  list-style-type: none;
  margin: 0;
  padding: 1rem;
  white-space: nowrap;

  li {
    height: 3rem;
    border-radius: 0.2rem;

    button {
      background-color: inherit;
      @include font-sans;
      font-size: inherit;
      color: inherit;
      cursor: pointer;
      width: 100%;
    }

    &:hover,
    button:hover {
      @include background-theme($nord3, $nord5);
    }

    a,
    button {
      @include flex-row();
      box-shadow: none;
      padding: 1rem;
      justify-content: flex-start;
      gap: 1rem;
    }
  }
}

.nav-text {
  overflow-x: hidden;
}

#nav-global {
  @include overflow-gradient($nord1, $nord0);
  flex-grow: 1;
  overflow-y: auto;
  overflow-x: hidden;
  [color-scheme="light"] & {
    @include overflow-gradient($nord4, $nord7);
  }
}

nav {
  height: 100%;
  width: $width-menu-collapsed;
  overflow: hidden;
  @include transition(width);
  @include background-theme($nord1, $nord4);
  @include flex-col();

  &.open {
    width: $width-menu-open;
    @include transition(width);
  }
}

@include small-screen {
  nav {
    height: 5rem;
    width: 100vw;
    @include transition(height, $speed: $medium-transition);
    &.open {
      position: absolute;
      width: 100vw;
      height: 100vh;
      z-index: 100;
      @include transition(height, $speed: $medium-transition);
    }
  }

  .menu-button > .nav-text {
    filter: opacity(100%) !important;
  }
}

@include large-screen {
  nav {
    width: $width-menu-open;
  }
}
</style>

<script setup lang="ts">
import { useDark, useToggle } from "@vueuse/core";
import { ParsedContent } from "@nuxt/content/dist/runtime/types";
import IconButton from "./IconButton.vue";

const route = useRoute();

async function updatePages(route: string): Promise<ParsedContent[]> {
  const english = route.startsWith("/en");
  const query = english ? queryContent("/en") : queryContent();
  const pages = await query.find().then((pages) =>
    pages
      .filter((page) => {
        return english
          ? page._path.startsWith("/en")
          : !page._path.startsWith("/en");
      })
      .filter((page) => page.toc != false)
  );
  console.log(pages);
  return pages;
}

let pages = ref(await updatePages(route.fullPath));

const isDark = useDark({
  selector: "body",
  attribute: "color-scheme",
  valueDark: "",
  valueLight: "light",
});

const toggleDark = useToggle(isDark);

watch(
  () => route.fullPath,
  async (route) => {
    pages.value = await updatePages(route);
  }
);
</script>

<script lang="ts">
import { LocaleInfo } from "node_modules/@nuxtjs/i18n/dist/module";

export default {
  methods: {
    toggleMenu() {
      var menu = document.getElementsByTagName("nav")[0];
      const navClass = menu.className;
      menu.className = navClass == "" ? "open" : "";
    },
    closeMenu(_: Event) {
      document.getElementsByTagName("nav")[0].className = "";
    },
  },
  computed: {
    availableLocales() {
      return this.$i18n.locales.filter(
        (lang: LocaleInfo) => lang.code !== this.$i18n.locale
      );
    },
  },
};
</script>
