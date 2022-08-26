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
      var menu = document.getElementsByTagName("nav")[0];
      const navClass = menu.className;
      menu.className = navClass == "" ? "menu-opened" : "";
    },
    closeMenu(_: Event) {
      document.getElementsByTagName("nav")[0].className = "";
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

<style lang="scss">
@import "node_modules/nord/src/sass/nord.scss";

nav {
  position: fixed;
  top: 0;
  width: 2.5rem;
  background-color: $nord1;
  height: 100%;
  z-index: 1;
  padding: 1rem 1rem;
  white-space: nowrap;
  overflow-y: auto;

  transition: width 0.3s ease;

  h3,
  .nav-text {
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  &.menu-opened {
    width: 100%;

    h3,
    .nav-text {
      opacity: 1;
    }
  }
}

/* Medium screens */
@media screen and (min-width: 600px) {
  nav.menu-opened {
    width: 14rem;
  }
}

@media screen and (min-width: 1300px) {
  nav {
    width: 14rem;

    h3,
    .nav-text {
      opacity: 1;
    }

    #open-menu {
      display: none;
      transition: display 0.6s ease;
    }
  }
}

.nav-links {
  display: flex;
  flex-direction: column;
  align-content: center;
  list-style: none;
  padding: 0;
  text-decoration: none;

  .nav-link {
    &:hover {
      background-color: $nord2;
    }
  }

  a {
    display: flex;
    flex-direction: row;
    box-shadow: none;
    padding: 0.5rem;

    &:hover {
      background: $nord2;
      box-shadow: none;
    }
  }
}

.nav-icon {
  display: block;
  height: 2rem;
  padding-right: 1.5rem;
}

#open-menu,
#theme-switcher,
#lang-switcher {
  position: relative;
  background: inherit;
  color: inherit;
  border: none;
  font-size: inherit;
  display: flex;
  flex-direction: row;
}

#open-menu {
  padding-left: 0.5rem;
}

#bottom-items {
  position: fixed;
  bottom: 2.5rem;
}
</style>
