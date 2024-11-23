import { defineConfig } from "vitepress";

export default defineConfig({
	head: [
		['link', { rel: 'icon', href: '/icon/favicon.ico' }],
	],
  lang: "en-US",
  title: "dnfolio",
  description: "Personal website maintained by Daiki48",
	srcExclude: ["/README.md"],
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
			provider: 'local',
			options: {
				detailedView: true,
			},
		},
		footer: {
      message: 'Personal website for Daiki48',
      copyright: 'Copyright © 2024 Daiki Nakashima'
    },
    logo: "/icon/icon.svg",
    nav: [
      { text: "Blog", link: "/blog" },
    ],

    sidebar: {
      "/blog/": [
        {
          text: "Blog",
          items: [
            { text: "build-with-lume", link: "/blog/build-with-lume/" },
            {
              text: "add-blockquote-style",
              link: "/blog/add-blockquote-style/",
            },
            {
              text: "add-mdit-plugin-alert",
              link: "/blog/add-mdit-plugin-alert/",
            },
            { text: "adjusted-img", link: "/blog/adjusted-img/" },
            {
              text: "changed-bluesky-handle-to-my-domain",
              link: "/blog/changed-bluesky-handle-to-my-domain/",
            },
            {
              text: "updated-ogp-setting-in-post-page",
              link: "/blog/updated-ogp-setting-in-post-page/",
            },
            {
              text: "customised-vitepress-search",
              link: "/blog/customised-vitepress-search/",
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
