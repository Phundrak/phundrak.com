import { defineClientConfig } from '@vuepress/client';
import ResponsiveImage from './components/ResponsiveImage.vue';
import ListRepositories from './components/GitRepos/ListRepositories.vue';
import GithubRepository from './components/GitRepos/GithubRepository.vue';
import ApiLoader from './components/ApiLoader.vue';
import Loader from './components/Loader.vue';
import Cache from './components/Cache.vue';

export default defineClientConfig({
  enhance({ app }) {
    app.component('ResponsiveImage', ResponsiveImage);
    app.component('ListRepositories', ListRepositories);
    app.component('GithubRepository', GithubRepository);
    app.component('ApiLoader', ApiLoader);
    app.component('Loader', Loader);
    app.component('Cache', Cache);
  },
  setup() {},
  layouts: {},
  rootComponents: [],
});
