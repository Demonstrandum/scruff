import mdx from "@astrojs/mdx";
import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://demonstrandum.github.io",
  base: process.env.SITE_BASE ?? "/scruff/",
  integrations: [mdx()],
});
