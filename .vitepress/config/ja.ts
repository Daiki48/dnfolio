import { defineConfig } from "vitepress";
import type { DefaultTheme } from "vitepress";

export const ja = defineConfig({
  transformPageData(pageData) {
    const isArticle = pageData.filePath.startsWith("ja/blog/articles/");
    const isDailyReport = pageData.filePath.startsWith("ja/blog/daily-report/");
    const canonicalUrl = `https://dnfolio.dev/${pageData.relativePath}`
      .replace(/index\.md$/, "")
      .replace(/\.md$/, ".html");

    pageData.frontmatter.head ??= [];
    pageData.frontmatter.head.push(
      ["link", { rel: "canonical", href: canonicalUrl }],
      ["meta", { property: "og:title", content: pageData.title }],
      [
        "meta",
        {
          property: "og:url",
          content: `https://dnfolio.dev/${pageData.filePath
            .replace(/^\//, "")
            .replace(/\.md$/, ".html")}`,
        },
      ],
      [
        "meta",
        {
          property: "og:type",
          content: isArticle ? "article" : isDailyReport ? "blog" : "wetsite",
        },
      ]
    );
  },
  head: [
    [
      "meta",
      { property: "og:image", content: "https://dnfolio.dev/icon/icon.webp" },
    ],
    ["meta", { property: "og:site_name", content: "dnfolio" }],
    ["meta", { property: "og:locale", content: "ja-JP" }],
    ["meta", { property: "twitter:card", content: "summary" }],
    ["meta", { property: "twitter:site", content: "@Daiki48engineer" }],
    ["link", { rel: "icon", href: "/icon/favicon.ico" }],
  ],
  lang: "ja-JP",
  description: "Daiki48の個人サイトです。",
  themeConfig: {
    outline: [2, 3],
    outlineTitle: "目次",
    lastUpdated: {
      text: "更新日時",
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
      message: "Daiki48の個人サイト",
      copyright: "Copyright © 2024 - 2025 Daiki Nakashima",
    },
    logo: "/icon/icon.svg",
    nav: [{ text: "ブログ", link: "/ja/blog" }],
    sidebar: {
      "/ja/blog/": [
        {
          text: "記事",
          collapsed: true,
          items: [
            {
              text: "AxumとTeraのお試しリポジトリを更新した",
              link: "/ja/blog/articles/updated-templates-for-axum-and-tera/",
            },
          ],
        },
        {
          text: "日報",
          items: [
            {
              text: "2025",
              collapsed: true,
              items: [{ text: "1/1", link: "/ja/blog/daily-report/2025/1/1/" }],
            },
          ],
        },
      ],
    },
    socialLinks: [
      { icon: "github", link: "https://github.com/Daiki48/dnfolio" },
      { icon: "x", link: "https://x.com/Daiki48engineer" },
    ],
  },
});

export const search: DefaultTheme.LocalSearchOptions['locales'] = {
	ja: {
		translations: {
			button: {
				buttonText: "検索"
			}
		}
	}
}
