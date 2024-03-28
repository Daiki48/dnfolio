import adapter from "@sveltejs/adapter-cloudflare";
import preprocess from "svelte-preprocess";
import { mdsvex } from "mdsvex";
import mdsvexConfig from "./mdsvex.config.js";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [
    preprocess(),
    mdsvex(mdsvexConfig),
  ],

  kit: {
    adapter: adapter({
      routes: {
        include: ["/*"],
        exclude: ["<all>"],
      },
    }),
    alias: { "$components": "./src/components" },
  },
  extensions: [".svelte", ...mdsvexConfig.extensions],
};

export default config;
