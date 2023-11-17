import adapter from "@sveltejs/adapter-cloudflare";
import { vitePreprocess } from "@sveltejs/kit/vite";
import { mdsvex } from "mdsvex";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [
    vitePreprocess(),
    mdsvex({
      extensions: [".md"],
    }),
  ],

  kit: {
    adapter: adapter({
      // pages: "build",
      // assets: "build",
      // fallback: undefined,
      // precompress: false,
      // strict: true,
      routes: {
        include: ["/*"],
        exclude: ["<all>"],
      },
    }),
    alias: { "$components": "./src/components" },
  },
  extensions: [".svelte", ".md"],
};

export default config;
