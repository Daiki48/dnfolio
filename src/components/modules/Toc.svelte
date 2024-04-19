<script lang="ts">
	import { onMount } from 'svelte';
	let isOpen: boolean = false;
	type Headers = {
		text: string;
		id: string;
		key: string;
	};

	let headers: Headers[] = [];

	const toggle = () => {
		isOpen = !isOpen;
	};

	onMount(async () => {
		await new Promise((r) => setTimeout(r, 0));
		headers = Array.from(document.querySelectorAll('h2, h3, h4')).map((header, index) => {
			const htmlElement = header as HTMLElement;
			return {
				text: htmlElement.textContent || '',
				id: htmlElement.id,
				key: `${htmlElement.textContent} - ${index}`
			};
		});
	});
</script>

<button on:click={toggle}>
	{#if isOpen}
		<span class="material-symbols-outlined"> keyboard_double_arrow_right </span>
	{:else}
		<span class="material-symbols-outlined"> keyboard_double_arrow_left </span>
	{/if}
</button>

{#if isOpen}
	<div class="toc open">
		<h5>見出し</h5>
		<ul>
			{#each headers as header (header.key)}
				<li><a href={`#${header.id}`} on:click={() => (isOpen = !isOpen)}>{header.text}</a></li>
			{/each}
		</ul>
	</div>
{:else}
	<div class="toc close"></div>
{/if}

<style>
	button {
		background-color: transparent;
		border: transparent;
		position: fixed;
		right: 6%;
		top: 50%;
		transform: translateY(-50%);
		cursor: pointer;
	}

	.toc {
		background-color: rgba(255, 244, 222, 1);
		border-radius: 6px;
		position: fixed;
		top: 50%;
		transform: translateY(-50%);
		padding: 0 50px;
		margin: 0;
		width: 0;
		overflow: hidden;
		transition: width 0.5s ease-in-out;
	}

	.toc.open {
		right: 10%;
		width: 200px;
	}

	.toc.close {
		right: 100%;
		width: 0;
	}

	a {
		text-decoration: none;
		color: rgb(3, 4, 5);
	}

	a:hover {
		border-bottom: 1px rgb(3, 4, 5) solid;
	}

	li {
		font-size: 0.8rem;
	}

	@media (max-width: 1000px) {
		button {
			right: 2%;
		}

		.toc.open {
			right: 14%;
			width: 200px;
		}
	}
</style>
