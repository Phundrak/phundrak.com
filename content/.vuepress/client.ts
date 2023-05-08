import { defineClientConfig } from '@vuepress/client';
import ResponsiveImage from './components/ResponsiveImage.vue';
import ListRepositories from './components/GitHub/ListRepositories.vue';
import FetchRepositories from './components/GitHub/FetchRepositories.vue';
import GithubRepository from './components/GitHub/GithubRepository.vue';
import ApiLoader from './components/ApiLoader.vue';
import Loader from './components/Loader.vue';
import Cache from './components/Cache.vue';

export default defineClientConfig({
  enhance({ app }) {
    app.component('ResponsiveImage', ResponsiveImage);
    app.component('ListRepositories', ListRepositories);
    app.component('FetchRepositories', FetchRepositories);
    app.component('GithubRepository', GithubRepository);
    app.component('ApiLoader', ApiLoader);
    app.component('Loader', Loader);
    app.component('Cache', Cache);
  },
  setup() {},
  layouts: {},
  rootComponents: [],
});
