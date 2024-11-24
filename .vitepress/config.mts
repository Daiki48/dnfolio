import { defineConfig } from "vitepress";

export default defineConfig({
  async transformHead(context) {
		const isBlog = context.pageData.filePath.startsWith("blog/");
    return [
      ["meta", { property: "og:title", content: context.pageData.title }],
      [
        "meta",
        {
          property: "og:description",
          content: context.pageData.description,
        },
      ],
      [
        "meta",
        {
          property: "og:url",
          content: `https://dnfolio.dev/${context.pageData.filePath
            .replace(/^\//, "")
            .replace(/\.md$/, ".html")}`,
        },
      ],
			[
				"meta",
				{
					property: "og:type",
					content: isBlog ? "blog" : "wetsite",
				},
			],
    ];
  },
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
  base: "/",
  lang: "en-US",
  title: "dnfolio",
  description: "Personal website maintained by Daiki48",
  srcExclude: ["**/README.md"],
  themeConfig: {
    outline: [2, 3],
    lastUpdated: {
      text: "Updated at",
      formatOptions: {
        dateStyle: "full",
        timeStyle: "medium",
      },
    },
    externalLinkIcon: true,
    search: {
      provider: "local",
      options: {
        detailedView: true,
      },
    },
    footer: {
      message: "Personal website for Daiki48",
      copyright: "Copyright © 2024 Daiki Nakashima",
    },
    logo: "/icon/icon.svg",
    nav: [{ text: "Blog", link: "/blog" }],

    sidebar: {
      "/blog/": [
        {
          text: "Blog",
          items: [
            { text: "Build with Lume", link: "/blog/build-with-lume/" },
            {
              text: "Add blockquote style",
              link: "/blog/add-blockquote-style/",
            },
            {
              text: "Add mdit plugin alert",
              link: "/blog/add-mdit-plugin-alert/",
            },
            { text: "Adjusted img", link: "/blog/adjusted-img/" },
            {
              text: "Changed Bluesky handle to my domain",
              link: "/blog/changed-bluesky-handle-to-my-domain/",
            },
            {
              text: "Updated ogp setting in post page",
              link: "/blog/updated-ogp-setting-in-post-page/",
            },
            {
              text: "Customised VitePress search",
              link: "/blog/customised-vitepress-search/",
            },
            {
              text: "Migrated VitePress from Lume",
              link: "/blog/migrated-vitepress-from-lume/",
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
