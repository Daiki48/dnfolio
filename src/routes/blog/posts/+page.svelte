<script lang="ts">
	import { formatDate } from '$lib/utils';
	import * as config from '$lib/config';
	import type { PageData } from '../$types';
	export let data: PageData;
</script>

<h1>カテゴリ</h1>

<ul>
	{#each data.posts as post}
		<li>
			{post.category}
			<ul>
				<li><a href={`/blog/posts/${post.category}/${post.slug}`}>{post.title}</a></li>
			</ul>
		</li>
	{/each}
</ul>

<style>
	ul li a:visited {
		color: inherit;
	}

	a {
		text-decoration: none;
	}
</style>
