[![Build Status](https://drone.phundrak.com/api/badges/phundrak/phundrak.com/status.svg)](https://drone.phundrak.com/phundrak/phundrak.com)

# My Personal Website

This personal website is written with Nuxt based on its module
[Content](https://content-v2.nuxtjs.org/).

I plan making it available on [phundrak.com](https://phundrak.com). It
is for now available at
[beta.phundrak.com](https://beta.phundrak.com), though it doesn’t look
great yet.

## Setup

Make sure to install the dependencies:

```bash
# yarn
yarn install

# npm
npm install

# pnpm
pnpm install --shamefully-hoist
```

## Development Server

Start the development server on http://localhost:3000

```bash
# yarn
yarn dev

# npm
npm run dev

# pnpm
pnpm run dev
```

## Production

Build the application for production:

```bash
# yarn
yarn generate

# npm
npm run generate

# pnpm
pnpm run generate
```

As it is a static website, there is no need to run `yarn build` or
equivalent.
