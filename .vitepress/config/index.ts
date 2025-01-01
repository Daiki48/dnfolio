import { defineConfig } from "vitepress";

import { shared } from "./shared";
import { en } from "./en";
import { ja } from "./ja";

export default defineConfig({
	...shared,
	locales: {
		root: { label: "English", ...en },
		ja: { label: "日本語", ...ja },
	}
});
