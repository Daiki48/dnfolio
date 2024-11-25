// https://vitepress.dev/guide/custom-theme
import { h } from "vue";
import type { Theme } from "vitepress";
import DefaultTheme from "vitepress/theme";
import BlogHeader from "./BlogHeader.vue";
import ToggleAppearance from "./ToggleAppearance.vue";
import "./style.css";
import "./custom.css";

const theme: Theme = {
  extends: DefaultTheme,
  Layout: () => {
    return h(DefaultTheme.Layout, null, {
      "doc-before": () => {
        return h(BlogHeader);
      },
      ToggleAppearance,
    });
  },
  enhanceApp({ app, router, siteData }) {
    // ...
  },
} satisfies Theme;

export default theme;
