<script>
	import Footer from '$components/layouts/Footer.svelte';
	import Header from '$components/layouts/Header.svelte';
	import Container from '$components/tags/Container.svelte';
	import '../css/global.css';

	let { children } = $props();
</script>

<main>
	<header>
		<Header />
	</header>

	<Container>
		{@render children()}
	</Container>

	<footer>
		<Footer />
	</footer>
</main>

<style>
	main {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	footer {
		margin-top: auto;
	}

	@media (max-width: 800px) {
		main {
			margin: 0;
			padding: 0;
		}
	}
</style>
