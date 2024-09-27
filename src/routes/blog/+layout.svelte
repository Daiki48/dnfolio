<script lang="ts">
	import * as config from '$lib/config';

	let { children } = $props();
</script>

<svelte:head>
	<title>{config.title}</title>
</svelte:head>

{@render children()}
