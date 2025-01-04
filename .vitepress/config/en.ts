import { defineConfig } from "vitepress";

export const en = defineConfig({
  head: [
    [
      "meta",
      { property: "og:image", content: "https://dnfolio.dev/icon/icon.webp" },
    ],
    ["meta", { property: "og:site_name", content: "dnfolio" }],
    ["meta", { property: "og:locale", content: "en-US" }],
    ["meta", { property: "twitter:card", content: "summary" }],
    ["meta", { property: "twitter:site", content: "@Daiki48engineer" }],
    ["link", { rel: "icon", href: "/icon/favicon.ico" }],
  ],
  lang: "en-US",
  description: "Personal website maintained by Daiki48",
  themeConfig: {
    outline: [2, 3],
    outlineTitle: "Table of Contents",
    lastUpdated: {
      text: "Updated at",
      formatOptions: {
        dateStyle: "full",
        timeStyle: "medium",
      },
    },
    externalLinkIcon: true,
    footer: {
      message: "Personal website for Daiki48",
      copyright: "Copyright © 2024 - 2025 Daiki Nakashima",
    },
    logo: "/icon/icon.svg",
    nav: [{ text: "Blog", link: "/blog" }],
    sidebar: {
      "/blog/": [
        {
          text: "Articles",
          collapsed: true,
          items: [
            {
              text: "Fix oil.nvim in fork",
              link: "/blog/articles/fix-oil-nvim-in-fork/",
            },
            {
              text: "Updated templates for Axum and Tera",
              link: "/blog/articles/updated-templates-for-axum-and-tera/",
            },
            {
              text: "Setting up monorepo development in Rust",
              link: "/blog/articles/setting-up-monorepo-development-in-rust/",
            },
            {
              text: "Build with Lume",
              link: "/blog/articles/build-with-lume/",
            },
            {
              text: "Add blockquote style",
              link: "/blog/articles/add-blockquote-style/",
            },
            {
              text: "Add mdit plugin alert",
              link: "/blog/articles/add-mdit-plugin-alert/",
            },
            { text: "Adjusted img", link: "/blog/articles/adjusted-img/" },
            {
              text: "Changed Bluesky handle to my domain",
              link: "/blog/articles/changed-bluesky-handle-to-my-domain/",
            },
            {
              text: "Updated ogp setting in post page",
              link: "/blog/articles/updated-ogp-setting-in-post-page/",
            },
            {
              text: "Customised VitePress search",
              link: "/blog/articles/customised-vitepress-search/",
            },
            {
              text: "Migrated VitePress from Lume",
              link: "/blog/articles/migrated-vitepress-from-lume/",
            },
          ],
        },
        {
          text: "Daily Report",
          items: [
            {
              text: "2025",
              collapsed: true,
              items: [
								{ text: "1/4", link: "/blog/daily-report/2025/1/4/" },
								{ text: "1/3", link: "/blog/daily-report/2025/1/3/" },
								{ text: "1/2", link: "/blog/daily-report/2025/1/2/" }
							],
            },
            {
              text: "2024",
              collapsed: true,
              items: [
                { text: "12/31", link: "/blog/daily-report/2024/12/31/" },
                { text: "12/30", link: "/blog/daily-report/2024/12/30/" },
                { text: "12/29", link: "/blog/daily-report/2024/12/29/" },
                { text: "12/28", link: "/blog/daily-report/2024/12/28/" },
                { text: "12/27", link: "/blog/daily-report/2024/12/27/" },
                { text: "12/26", link: "/blog/daily-report/2024/12/26/" },
                { text: "12/25", link: "/blog/daily-report/2024/12/25/" },
                { text: "12/24", link: "/blog/daily-report/2024/12/24/" },
                { text: "12/23", link: "/blog/daily-report/2024/12/23/" },
                { text: "12/22", link: "/blog/daily-report/2024/12/22/" },
                { text: "12/21", link: "/blog/daily-report/2024/12/21/" },
                { text: "12/20", link: "/blog/daily-report/2024/12/20/" },
                { text: "12/19", link: "/blog/daily-report/2024/12/19/" },
                { text: "12/18", link: "/blog/daily-report/2024/12/18/" },
                { text: "12/17", link: "/blog/daily-report/2024/12/17/" },
                { text: "12/16", link: "/blog/daily-report/2024/12/16/" },
                { text: "12/15", link: "/blog/daily-report/2024/12/15/" },
                { text: "12/14", link: "/blog/daily-report/2024/12/14/" },
                { text: "12/13", link: "/blog/daily-report/2024/12/13/" },
                { text: "12/12", link: "/blog/daily-report/2024/12/12/" },
                { text: "12/11", link: "/blog/daily-report/2024/12/11/" },
                { text: "12/10", link: "/blog/daily-report/2024/12/10/" },
                { text: "12/9", link: "/blog/daily-report/2024/12/09/" },
                { text: "12/8", link: "/blog/daily-report/2024/12/08/" },
                { text: "12/7", link: "/blog/daily-report/2024/12/07/" },
                { text: "12/6", link: "/blog/daily-report/2024/12/06/" },
                { text: "12/5", link: "/blog/daily-report/2024/12/05/" },
                { text: "12/4", link: "/blog/daily-report/2024/12/04/" },
                { text: "12/3", link: "/blog/daily-report/2024/12/03/" },
                { text: "12/2", link: "/blog/daily-report/2024/12/02/" },
                { text: "12/1", link: "/blog/daily-report/2024/12/01/" },
                { text: "11/30", link: "/blog/daily-report/2024/11/30/" },
                { text: "11/29", link: "/blog/daily-report/2024/11/29/" },
                { text: "11/28", link: "/blog/daily-report/2024/11/28/" },
                { text: "11/27", link: "/blog/daily-report/2024/11/27/" },
                { text: "11/26", link: "/blog/daily-report/2024/11/26/" },
                { text: "11/25", link: "/blog/daily-report/2024/11/25/" },
              ],
            },
          ],
        },
      ],
    },
    socialLinks: [
      { icon: "github", link: "https://github.com/Daiki48/dnfolio" },
      { icon: "bluesky", link: "https://bsky.app/profile/dnfolio.dev" },
    ],
  },
});
