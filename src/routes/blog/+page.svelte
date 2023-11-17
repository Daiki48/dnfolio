<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<h1>記事一覧</h1>

{#each data.posts as post (post.slug)}
	<a href={`/blog/${post.slug}`}>
		<h2 class="post">
			{post.title}
			<p class="description">{post.content}</p>
		</h2>
	</a>
{/each}

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
	}

	a:visited {
		color: inherit;
	}

	.description {
		font-size: 2px;
	}
</style>
