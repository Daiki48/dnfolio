<script lang="ts">
	export let src = '';
	export let alt = '';
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
