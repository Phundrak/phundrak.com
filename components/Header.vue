<template>
  <nav @mouseleave="closeMenu">
    <ul class="nav-links">
      <li class="nav-link">
        <IconLink to="#" icon="bars" @click="toggleMenu">Menu</IconLink>
      </li>
    </ul>
    <div id="nav-global" @click="closeMenu">
      <h3>Navigation</h3>
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
        <IconLink to="#" icon="theme" @click="toggleDark()"> Theme </IconLink>
      </li>
    </ul>
  </nav>
</template>

<style lang="scss">
$large-screen: 1200px;

@mixin transition($opacity) {
  filter: opacity($opacity);
  transition: filter 0.3s ease-in-out;
}

nav {
  .nav-text,
  h3 {
    @include transition(0%);
  }
  &.open {
    .nav-text,
    h3 {
      @include transition(100%);
    }
  }
}

@media only screen and (min-width: $large-screen) {
  nav {
    width: 16rem;
    .nav-text,
    h3 {
      @include transition(100%);
    }
  }
}
</style>

<style lang="scss" scoped>
@use 'sass:color';
@import 'node_modules/nord/src/sass/nord.scss';

$small-screen: 600px;
$large-screen: 1200px;

@mixin transition($property) {
  transition: #{$property} 0.3s ease-in-out;
}

h3 {
  text-align: center;
}

.nav-links {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  list-style-type: none;
  margin: 0;
  padding: 1rem;

  li {
    height: 3rem;
    border-radius: 0.2rem;
    @include transition(background-color);

    &:hover {
      background-color: $nord3;
      @include transition(background-color);

      [color-scheme='light'] & {
        background-color: $nord5;
        transition: background-color 0.3s ease-in-out;
      }
    }

    a {
      box-shadow: none;
      padding: 1rem;
      display: flex;
      flex-direction: row;
      justify-content: flex-start;
      gap: 1rem;
    }
  }
}

.nav-text {
  overflow-x: hidden;
}

#nav-global {
  flex-grow: 4;
}

#container {
  grid-template-columns: 5rem auto;
}

nav {
  width: 5rem;
  height: 100%;
  background-color: $nord1;
  @include transition(width);
  @include transition(background-color);

  display: flex;
  flex-direction: column;

  body[color-scheme='light'] & {
    background-color: $nord4;
    @include transition(background-color);
  }

  &.open {
    width: 16rem;
    @include transition(width);
  }
}

@media only screen and (max-width: $small-screen) {
  nav.open {
    width: 100vw;
  }
}

@media only screen and (min-width: $large-screen) {
  nav {
    width: 16rem;
  }
}
</style>

<script setup lang="ts">
const lang = useCookie('lang');
const homeQuery =
  lang.value === 'fr' ? queryContent() : queryContent('/' + lang.value);
const pages = await homeQuery.find().then((pages) =>
  pages.filter((page) => {
    return lang.value === 'fr'
      ? !page._path.startsWith('/en')
      : page._path.startsWith('/' + lang.value);
  })
);
const isDark = useDark({
  selector: 'body',
  attribute: 'color-scheme',
  valueDark: '',
  valueLight: 'light',
});
const toggleDark = useToggle(isDark);
</script>

<script lang="ts">
import { LocaleInfo } from 'node_modules/@nuxtjs/i18n/dist/module';

export default {
  data() {
    return {
      query: null,
    };
  },
  methods: {
    toggleMenu() {
      var menu = document.getElementsByTagName('nav')[0];
      const navClass = menu.className;
      menu.className = navClass == '' ? 'open' : '';
    },
    closeMenu(_: Event) {
      document.getElementsByTagName('nav')[0].className = '';
    },
  },
  computed: {
    availableLocales() {
      return this.$i18n.locales.filter(
        (lang: LocaleInfo) => lang.code !== this.$i18n.locale
      );
    },
  },
  created() {
    const fallbackLocale = this.$i18n.fallbackLocale;
    const locale = this.$i18n.locale;
    if (fallbackLocale != locale) {
      this.query = queryContent('en');
    }
  },
};
</script>
