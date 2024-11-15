export default {
	lang: "en",
	name: "Dnfolio",
	styles: [
		"/styles/uno.css",
		"/styles/atom-one-dark.css",
	],
	icon: "/assets/icon/favicon.ico",
	ogp: {
		name: [
			"og:url",
			"og:title",
			"og:site_name",
			"og:image",
			"og:description",
			"og:type",
		],
		content: [
			"https://dnfolio.dev",
			"Dnfolio",
			"Dnfolio",
			"https://dnfolio.dev/assets/icon/icon.webp",
			"Personal website maintained by Daiki48",
			"website",
		],
	},
} as const
