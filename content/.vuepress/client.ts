import { defineClientConfig } from '@vuepress/client';
import PreviewImage from './components/PreviewImage.vue';
import ResponsiveImage from './components/ResponsiveImage.vue';
import LatestRepositories from './components/LatestRepositories.vue';

export default defineClientConfig({
  enhance({ app, router, siteData }) {
    app.component('PreviewImage', PreviewImage);
    app.component('ResponsiveImage', ResponsiveImage);
    app.component('LatestRepositories', LatestRepositories);
  },
  setup() {},
  layouts: {},
  rootComponents: [],
});
