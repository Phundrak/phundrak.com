<template>
  <nav @mouseleave="closeMenu">
    <button id="open-menu" @click="toggleMenu">
      <font-awesome-icon icon="fa-solid fa-bars" size="lg" fixed-width />
    </button>
    <div id="nav-global" @click="closeMenu">
      <h3>Navigation</h3>
      <ul class="nav-links">
        <ContentNavigation v-slot="{ navigation }">
          <li class="nav-link" v-for="page in navigation" :key="page._path">
            <IconLink :icon="page.faIcon" :path="page._path">
              {{ page.navTitle || page.title }}
            </IconLink>
          </li>
        </ContentNavigation>
      </ul>
    </div>
    <div id="nav-local" @click="closeMenu"></div>
    <div id="theme" @click="closeMenu">
      <button id="theme-switcher">
        <span class="nav-icon">
          <font-awesome-icon :icon="themeIcon" size="lg" fixed-width />
        </span>
        <span class="nav-text">Change Theme</span>
      </button>
    </div>
  </nav>
</template>

<style lang="scss">
@import "node_modules/nord/src/sass/nord.scss";

nav {
  position: fixed;
  top: 0;
  width: 2.5rem;
  background-color: $nord1;
  height: 100%;
  z-index: 2;
  padding: 1rem 1rem;
  white-space: nowrap;
  overflow-y: auto;

  transition: width 0.3s ease;

  h3,
  .nav-text {
    /* display: none; */
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
#theme-switcher {
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

#theme-switcher {
  bottom: 0;
}
</style>

<script lang="ts">
export default {
  data() {
    return {
      uri: "",
      themeIcon: "",
    };
  },
  created() {
    this.themeIcon = "fa-solid fa-" + this.isDarkTheme() ? "sun" : "moon";
  },
  methods: {
    isDarkTheme() {
      return true;
    },
    toggleMenu() {
      var menu = document.getElementsByTagName("nav")[0];
      const navClass = menu.className;
      menu.className = navClass == "" ? "menu-opened" : "";
    },
    closeMenu(_: Event) {
      document.getElementsByTagName("nav")[0].className = "";
    },
  },
};
</script>
