<script lang="ts">
	import Toc from '$components/modules/Toc.svelte';
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<svelte:head>
	<title>{data.title ? `${data.title} | Dnfolio` : 'Dnfolio'}</title>
	<meta
		name="description"
		content={data.description
			? `${data.description}`
			: 'Daikiのブログです。雑多な内容からプログラミングに関する記事まで幅広く書きます'}
	/>
	<meta property="og:title" content={data.title ? `${data.title}` : "Dnfolio's blog"} />
	<meta
		property="og:description"
		content={data.description ? `${data.description}` : 'Daikiの趣味プログラミング'}
	/>
	<meta property="og:site_name" content="Dnfolio" />
	<meta
		property="og:url"
		content={data.category
			? `https://dnfolio.dev/blog/${data.category}/${data.slug}`
			: 'https://dnfolio.dev/blog'}
	/>
	<meta property="og:image" content="https://dnfolio.dev/icon.webp" />
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:site" content="@Daiki48engineer" />
	<meta name="twitter:player" content="@Daiki48engineer" />
</svelte:head>

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
		<p class="description">{data.description}</p>
		<div class="content-toc">
			<div class="content">
				<svelte:component this={data.content} />
			</div>
			<div class="toc">
				<Toc />
			</div>
		</div>
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

	.content-toc {
		display: flex;
		position: relative;
		padding-top: 6rem;
	}

	.content {
		width: 70%;
	}

	.toc {
		position: sticky;
		top: 10px;
		width: 30%;
		height: 80vh;
		overflow: auto;
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

	.description {
		display: flex;
		justify-content: center;
		text-align: center;
	}

	@media (max-width: 800px) {
		section {
			width: 100%;
		}

		.content-toc {
			flex-direction: column-reverse;
		}

		.content {
			width: 100%;
		}

		.toc {
			position: static;
			width: 100%;
		}
	}
</style>
