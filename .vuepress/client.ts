import { defineClientConfig } from '@vuepress/client';
import PreviewImage from './components/PreviewImage.vue';
import ResponsiveImage from './components/ResponsiveImage.vue';

export default defineClientConfig({
  enhance({ app, router, siteData }) {
    app.component('PreviewImage', PreviewImage);
    app.component('ResponsiveImage', ResponsiveImage);
  },
  setup() {},
  layouts: {},
  rootComponents: [],
});
