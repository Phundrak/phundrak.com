import { HeadAttrsConfig } from 'vuepress';

interface SimplifiedHeader {
  tag: string;
  content: HeadAttrsConfig[];
}

const simplifiedHead: SimplifiedHeader[] = [
  {
    tag: 'meta',
    content: [
      {
        name: 'author',
        content: 'Lucien Cartier-Tilet',
      },
      {
        name: 'fediverse:creator',
        content: '@phundrak@mastodon.phundrak.com',
      },
      {
        property: 'og:image',
        content: 'https://cdn.phundrak.com/img/rich_preview.png',
      },
      {
        property: 'org:title',
        content: 'Lucien Cartier-Tilet',
      },
      {
        property: 'og:description',
        content: 'Site web personnel de Lucien Cartier-Tilet',
      },
      {
        name: 'twitter:card',
        content: 'summary',
      },
      {
        name: 'twitter:site',
        content: '@phundrak',
      },
      {
        name: 'twitter:creator',
        content: '@phundrak',
      },
      {
        name: 'build-status',
        content: `value: ${process.env.NODE_ENV}`,
      },
      { name: 'msapplication-TileColor', content: '#3b4252' },
      { name: 'msapplication-TileImage', content: '/ms-icon-144x144.png' },
      { name: 'theme-color', content: '#3b4252' },
    ],
  },
  {
    tag: 'link',
    content: [
      {
        rel: 'apple-touch-icon',
        sizes: '57x57',
        href: '/apple-icon-57x57.png',
      },
      {
        rel: 'apple-touch-icon',
        sizes: '60x60',
        href: '/apple-icon-60x60.png',
      },
      {
        rel: 'apple-touch-icon',
        sizes: '72x72',
        href: '/apple-icon-72x72.png',
      },
      {
        rel: 'apple-touch-icon',
        sizes: '76x76',
        href: '/apple-icon-76x76.png',
      },
      {
        rel: 'apple-touch-icon',
        sizes: '114x114',
        href: '/apple-icon-114x114.png',
      },
      {
        rel: 'apple-touch-icon',
        sizes: '120x120',
        href: '/apple-icon-120x120.png',
      },
      {
        rel: 'apple-touch-icon',
        sizes: '144x144',
        href: '/apple-icon-144x144.png',
      },
      {
        rel: 'apple-touch-icon',
        sizes: '152x152',
        href: '/apple-icon-152x152.png',
      },
      {
        rel: 'apple-touch-icon',
        sizes: '180x180',
        href: '/apple-icon-180x180.png',
      },
      {
        rel: 'icon',
        type: 'image/png',
        sizes: '192x192',
        href: '/android-icon-192x192.png',
      },
      {
        rel: 'icon',
        type: 'image/png',
        sizes: '32x32',
        href: '/favicon-32x32.png',
      },
      {
        rel: 'icon',
        type: 'image/png',
        sizes: '96x96',
        href: '/favicon-96x96.png',
      },
      {
        rel: 'icon',
        type: 'image/png',
        sizes: '16x16',
        href: '/favicon-16x16.png',
      },
      { rel: 'manifest', href: '/manifest.json' },
    ],
  },
];

const headBuilder = [];
simplifiedHead.forEach((tag) => {
  tag.content.forEach((element) => {
    headBuilder.push([tag.tag, element]);
  });
});
headBuilder.push([
  'a',
  { rel: 'me', href: 'https://mastodon.phundrak.com/@phundrak' },
  'Mastodon',
]);

export const head = headBuilder;
