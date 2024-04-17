<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<article>
	<section>
		<div class="header">
			<h1>{data.title}</h1>
			<div class="time-container">
				<span class="material-symbols-outlined"> calendar_month </span>
				<time>
					{data.createdAt}
				</time>
				{#if data.updatedAt}
					<span class="material-symbols-outlined"> update </span>
					<time>
						{data.updatedAt}
					</time>
				{/if}
			</div>
		</div>
		<svelte:component this={data.content} />
		<div class="blog-top">
			<a href="/blog">記事一覧へ</a>
		</div>
	</section>
</article>

<style>
	article {
		display: flex;
		justify-content: center;
	}

	section {
		width: 80%;
	}

	h1 {
		font-size: 40px;
	}

	.blog-top {
		display: flex;
		justify-content: center;
		margin: 100px;
	}

	.header {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.time-container {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.time-container time {
		margin: 4px;
	}

	@media (max-width: 800px) {
		section {
			width: 100%;
		}
	}
</style>
