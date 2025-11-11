export interface MetaImageOptions {
  url: string;
  alt: string;
}

export interface MetaOptions {
  title: string;
  description: string;
  image?: MetaImageOptions;
}

export const useMeta = (options: MetaOptions) => {
  const titleSuffix = ' – Lucien Cartier-Tilet';
  useSeoMeta({
    title: () => options.title + titleSuffix,
    ogTitle: () => options.title + titleSuffix,
    twitterTitle: () => options.title + titleSuffix,
    description: () => options.description,
    ogDescription: () => options.description,
    twitterDescription: () => options.description,
    twitterCard: options.image ? 'summary_large_image' : 'summary',
    ogImage: () => options.image?.url,
    ogImageAlt: () => options.image?.alt,
    twitterImage: () => options.image?.url,
    twitterImageAlt: () => options.image?.alt,
  });
};
