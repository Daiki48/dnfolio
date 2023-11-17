<script>
	import Footer from '$components/layouts/Footer.svelte';
	import Header from '$components/layouts/Header.svelte';
	import Container from '$components/tags/Container.svelte';
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
		background-color: #fffefa;
		color: #345;
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
</style>
