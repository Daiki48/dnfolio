<script lang="ts">
	import { onMount } from 'svelte';
	type Headers = {
		text: string;
		id: string;
		tag: string;
		key: string;
	};

	let headers: Headers[] = [];

	onMount(async () => {
		await new Promise((r) => setTimeout(r, 0));
		headers = Array.from(document.querySelectorAll('h2, h3, h4')).map((header, index) => {
			const htmlElement = header as HTMLElement;
			return {
				text: htmlElement.textContent || '',
				id: htmlElement.id,
				tag: htmlElement.tagName,
				key: `${htmlElement.textContent} - ${index}`
			};
		});
	});
</script>

<div class="toc open">
	<h5>見出し</h5>
	<ul>
		{#each headers as header (header.key)}
			{#if header.tag === 'H3'}
				<li class="indent-h3">
					<a href={`#${header.id}`}>{header.text}</a>
				</li>
			{:else if header.tag === 'H4'}
				<li class="indent-h4">
					<a href={`#${header.id}`}>{header.text}</a>
				</li>
			{:else}
				<li>
					<a href={`#${header.id}`}>{header.text}</a>
				</li>
			{/if}
		{/each}
	</ul>
</div>

<style>
	.toc {
		background-color: rgba(255, 244, 222, 0.6);
		border-radius: 6px;
		position: sticky;
		top: 6rem;
		padding: 0 50px;
		margin: 1rem;
		width: 0;
		overflow: hidden;
		transition: width 0.5s ease-in-out;
	}

	.toc.open {
		right: 8%;
		width: auto;
	}

	.indent-h3 {
		padding-left: 0.8rem;
	}

	.indent-h4 {
		padding-left: 1.2rem;
	}

	a {
		text-decoration: none;
		color: rgb(3, 4, 5);
	}

	a:hover {
		border-bottom: 1px rgb(3, 4, 5) solid;
	}

	ul {
		padding-left: 0;
	}

	li {
		font-size: 0.8rem;
		list-style: none;
	}

	@media (max-width: 1000px) {
		.toc {
			right: 0;
			width: 100%;
		}
	}
</style>
