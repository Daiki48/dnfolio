<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<article>
	<section>
		<h1>{data.title}</h1>
		<time>{data.createdAt}</time>
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
		font-size: 20px;
	}

	.blog-top {
		display: flex;
		justify-content: center;
		margin: 100px;
	}
</style>
