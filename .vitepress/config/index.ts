import { defineConfig } from "vitepress";

import { shared } from "./shared";
import { en } from "./en";
import { ja } from "./ja";

export default defineConfig({
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
      ["meta", { name: "twitter:title", content: pageData.title }],
      ["meta", { name: "twitter:description", content: pageData.description }],
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
	...shared,
	locales: {
		root: { label: "English", ...en },
		ja: { label: "日本語", ...ja },
	}
});
