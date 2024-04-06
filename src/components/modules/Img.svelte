<script lang="ts">
	export let src: string = '';
	export let alt: string = '';
</script>

<div class="container">
	<img {src} {alt} />
</div>

<style>
	img {
		height: 50%;
		max-width: 90%;
	}

	.container {
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
