import mdx from "@astrojs/mdx";
import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://demonstrandum.github.io",
  base: process.env.SITE_BASE ?? "/scruff/",
  build: {
    // Branch-based GitHub Pages runs Jekyll, which drops underscore-prefixed paths.
    assets: "astro",
  },
  integrations: [mdx()],
});
