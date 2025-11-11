import { defineCollection, defineContentConfig } from '@nuxt/content';
import { z } from 'zod';

const commonSchema = z.object({
  title: z.string(),
  description: z.string()
});

export default defineContentConfig({
  collections: {
    content_en: defineCollection({
      type: 'page',
      source: {
        include: 'en/**/*.md',
        prefix: '',
      },
      schema: commonSchema,
    }),
    content_fr: defineCollection({
      type: 'page',
      source: {
        include: 'fr/**/*.md',
        prefix: '',
      },
      schema: commonSchema,
    }),
    content_data_en: defineCollection({
      type: 'data',
      source: {
        include: 'en/**/*.json',
        prefix: ''
      },
    }),
    content_data_fr: defineCollection({
      type: 'data',
      source: {
        include: 'fr/**/*.json',
        prefix: ''
      },
    }),
  },
});
