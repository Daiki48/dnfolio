import lume from "lume/mod.ts";
import code_highlight from "lume/plugins/code_highlight.ts";
import inline from "lume/plugins/inline.ts";
import nav from "lume/plugins/nav.ts";
import jsx from "lume/plugins/jsx.ts";
import unocss from "lume/plugins/unocss.ts";
import unocssConfig from "./uno.config.ts";

const site = lume({
  src: "./src",
  server: {
    open: true,
  },
});

site.use(
  code_highlight({
    theme: {
      name: "atom-one-dark",
      path: "/styles/atom-one-dark.css",
    },
  }),
);
site.use(inline());
site.use(nav());
site.use(jsx());
site.use(
  unocss({
    options: unocssConfig,
    cssFile: "/styles/uno.css",
  }),
);

site.copy("./_styles/global.css", "/styles/global.css");
site.copy("./_assets", "/assets");

site.ignore("README.md");

export default site;
