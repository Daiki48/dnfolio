import { defineConfig } from "vitepress";

// import { search as jaSearch } from "./ja";

export const shared = defineConfig({
  base: "/",
  title: "dnfolio",
  srcExclude: ["**/README.md"],
  ignoreDeadLinks: true,
  themeConfig: {
    search: {
      provider: "local",
      options: {
        detailedView: true,
        locales: {
          // ...jaSearch,
        },
      },
    },
  },
});
