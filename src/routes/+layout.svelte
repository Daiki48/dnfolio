<script>
	import Footer from '$components/layouts/Footer.svelte';
	import Header from '$components/layouts/Header.svelte';
	import Container from '$components/tags/Container.svelte';
	import '../css/global.css';
</script>

<body>
	<header>
		<Header />
	</header>

	<Container>
		<main>
			<slot />
		</main>
	</Container>

	<footer>
		<Footer />
	</footer>
</body>

<style>
	body {
		font-family: 'BIZ UDPGothic', 'ヒラギノ角ゴ Pro W3', 'Hiragino Kaku Gothic Pro', 'メイリオ',
			Meiryo, Osaka, 'MS PGothic', sans-serif;
		background-color: rgba(255, 254, 250, 1);
		color: rgba(3, 4, 5, 0.8);
		line-height: 2;
	}
	main {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}
	footer {
		margin-top: auto;
	}

	@media (max-width: 800px) {
		body {
			margin: 0;
			padding: 0;
		}
	}
</style>
