# Scruff Tali showcase (GitHub Pages)

An [Astro](https://astro.build/) and MDX site documenting Tali mode formatting
rules with before/after showcases and amalgamated examples. The documentation
content lives in `src/pages/index.mdx`; reusable presentation lives in
`src/components/`.

`.github/workflows/github-pages.yml` builds the site and publishes `dist/` to
the `gh-pages` branch:

- pushes to `master` publish to the Pages root
- pull requests touching `website/**` publish a preview at
    `/pr-preview/pr-<number>/` and get a comment linking to it

Pages must be configured as **Deploy from a branch** using `gh-pages` (root).

Install dependencies and start the development server:

```sh
npm ci
npm run dev
```

Run a production build with:

```sh
npm run build
```
