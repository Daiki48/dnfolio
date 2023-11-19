<script lang="ts">
	import { formatDate } from '$lib/utils';
	import * as config from '$lib/config';
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<svelte:head>
	<title>{config.title}</title>
</svelte:head>

<h1>記事一覧</h1>

<!-- <a href="blog/posts">カテゴリから選択</a> -->

<section>
	<ul class="posts">
		{#each data.posts as post}
			<a href={`blog/${post.category}/${post.slug}`} class="title">
				<!-- <a href={`blog/${post.slug}`} class="title"> -->
				<li class="post">
					{post.title}
					<ul>
						<li>{post.tags}</li>
					</ul>
					<p class="date">公開日 : {formatDate(post.createdAt)}</p>
					{#if post.updatedAt}
						<p class="date">更新日 : {formatDate(post.updatedAt)}</p>
					{/if}
					<p>{post.category}</p>
					<p>{post.slug}</p>
					<p class="description">{post.description}</p>
				</li>
			</a>
		{/each}
	</ul>
</section>

<style>
	.posts {
		display: flex;
		list-style: none;
	}
	.post {
		border: 1px #333 solid;
		border-radius: 10px;
		padding: 10px;
	}

	.post:hover {
		background-color: #f2f0f0;
	}

	a {
		text-decoration: none;
		color: #345 !important;
	}

	a:visited {
		color: inherit;
	}
</style>
