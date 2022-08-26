<template>
  <nav @mouseleave="closeMenu">
    <button id="open-menu" @click="toggleMenu">
      <Icon icon="bars" />
    </button>
    <div id="nav-global" @click="closeMenu">
      <h3>Navigation</h3>
      <ul class="nav-links">
        <div v-for="link of pages" :key="link._page">
          <IconLink :icon="link.icon" :to="localePath(link._path)">
            {{ link.navTitlo || link.title }}
          </IconLink>
        </div>
      </ul>
    </div>
    <div id="nav-local" @click="closeMenu"></div>
    <ul id="bottom-items" class="nav-links">
      <li v-for="locale in availableLocales" :key="locale.code">
        <IconLink :to="switchLocalePath(locale.code)">
          {{ locale.name }}
        </IconLink>
      </li>
      <li id="theme" class="nav-link" @click="closeMenu">
        <button id="theme-switcher">
          <span class="nav-icon">
            <Icon icon="theme" />
          </span>
          <span class="nav-text">Change Theme</span>
        </button>
      </li>
    </ul>
  </nav>
</template>

<script setup lang="ts">
const lang = useCookie('lang');
const homeQuery =
  lang.value === 'fr' ? queryContent() : queryContent('/' + lang.value);
const pages = await homeQuery.find().then((pages) =>
  pages.filter((page) => {
    if (lang.value === 'fr') {
      return !page._path.startsWith('/en');
    }
    return page._path.startsWith('/' + lang.value);
  })
);
</script>

<script lang="ts">
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
      menu.className = navClass == '' ? 'menu-opened' : '';
    },
    closeMenu(_: Event) {
      document.getElementsByTagName('nav')[0].className = '';
    },
  },
  computed: {
    availableLocales() {
      return this.$i18n.locales.filter(
        (lang) => lang.code !== this.$i18n.locale
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

<style lang="scss" scoped>
@import 'node_modules/nord/src/sass/nord.scss';

nav {
  width: 5rem;
  background-color: $nord1;
}
</style>
