<div class="container">
	<slot />
</div>

<style>
	.container {
		margin: 2em 4em;
		padding: 1em;
	}
	@media (max-width: 800px) {
		.container {
			margin: 0 0.8em;
			padding: 10px;
		}
	}
</style>
