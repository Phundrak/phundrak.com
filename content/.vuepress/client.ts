import { defineClientConfig } from '@vuepress/client';
import PreviewImage from './components/PreviewImage.vue';
import ResponsiveImage from './components/ResponsiveImage.vue';
import LatestRepositories from './components/LatestRepositories.vue';
import GithubRepository from './components/GithubRepository.vue';

export default defineClientConfig({
  enhance({ app }) {
    app.component('PreviewImage', PreviewImage);
    app.component('ResponsiveImage', ResponsiveImage);
    app.component('LatestRepositories', LatestRepositories);
    app.component('GithubRepository', GithubRepository);
  },
  setup() {},
  layouts: {},
  rootComponents: [],
});
