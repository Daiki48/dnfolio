<svelte:head>
	<title>HOME | Dnfolio</title>
	<meta name="description" content="Daikiの個人サイトです。" />
	<meta property="og:title" content="Dnfolio" />
	<meta property="og:description" content="Daikiの趣味プログラミング" />
	<meta property="og:site_name" content="Dnfolio" />
	<meta property="og:url" content="https://dnfolio.dev" />
	<meta property="og:image" content="/icon.webp" />
</svelte:head>

<div class="main">
	<img src="/icon.webp" alt="icon" class="icon-img" height="88" width="116" />
</div>
<h1>Hello✋</h1>

<style>
	h1 {
		display: flex;
		justify-content: center;
		font-size: 3em;
	}

	.main {
		display: flex;
		justify-content: center;
	}

	.icon-img {
		border-radius: 50%;
		width: 40%;
		height: auto;
	}

	@media (width: 900px) {
		.icon-img {
			border-radius: 50%;
			width: 30%;
			height: auto;
		}
	}
</style>
