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

<section>
	<ul class="posts">
		{#each data.posts as post}
			<a href={`blog/${post.category}/${post.slug}`} class="title">
				<li class="post">
					<h1>{post.title}</h1>
					<div class="middle-container">
						<p class="tags">{post.tags}</p>
						<div class="date-container">
							{#if post.updatedAt}
								<p class="update-date">更新日 : {formatDate(post.updatedAt)}</p>
							{/if}
							<p class="create-date">公開日 : {formatDate(post.createdAt)}</p>
						</div>
					</div>
					<!-- <p class="category">{post.category}</p> -->
					<p class="description">{post.description}</p>
				</li>
			</a>
		{/each}
	</ul>
</section>

<style>
	.posts {
		display: flex;
		flex-direction: column;
		list-style: none;
		padding: 0;
		margin: 0;
	}
	.post {
		/* border: 1px #333 solid; */
		border-radius: 10px;
		box-shadow: 2px 2px 4px gray;
		padding: 10px;
		margin: 10px;
		width: 100%;
	}

	.post:hover {
		background-color: #f7f5f5;
	}

	a {
		text-decoration: none;
		color: #345 !important;
	}

	a:visited {
		color: inherit;
	}

	.middle-container {
		display: flex;
		justify-content: space-between;
	}

	.middle-container .tags {
		justify-content: start;
	}
	.middle-container .date-container {
		justify-content: end;
	}

	.date-container {
		display: flex;
	}

	.date-container .update-date {
		margin-right: 10px;
	}
</style>
