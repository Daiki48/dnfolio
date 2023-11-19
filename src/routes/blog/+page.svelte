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

<a href="blog/posts">カテゴリから選択</a>

<section>
	<ul class="posts">
		{#each data.posts as post}
			<li class="post">
				<a href={`blog/posts/${post.category}/${post.slug}`} class="title">{post.title}</a>
				<p>Slug : {post.slug}</p>
				<p>Category : {post.category}</p>
				<p class="date">公開日 : {formatDate(post.createdAt)}</p>
				{#if post.updatedAt}
					<p class="date">更新日 : {formatDate(post.updatedAt)}</p>
				{/if}
				<p class="description">{post.description}</p>
			</li>
		{/each}
	</ul>
</section>

<style>
	.post {
		border: 1px #333 solid;
		border-radius: 20px;
		padding: 10px;
	}

	.post:hover {
		background-color: #ededed;
	}

	a {
		text-decoration: none;
		color: #345 !important;
	}

	a:visited {
		color: inherit;
	}
</style>
