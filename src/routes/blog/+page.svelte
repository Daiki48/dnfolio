<script lang="ts">
	import { formatDate } from '$lib/utils';
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<h1>記事一覧</h1>

<section>
	<ul class="posts">
		{#each data.posts as post}
			<a href={`blog/${post.category}/${post.slug}`} class="title">
				<li class="post">
					<h1>{post.title}</h1>
					<div class="middle-container">
						<div class="date-container">
							{#if post.updatedAt}
								<p class="update-date">更新日 : {formatDate(post.updatedAt)}</p>
							{/if}
							<p class="create-date">公開日 : {formatDate(post.createdAt)}</p>
						</div>
					</div>
					<p class="description">{post.description}</p>
					<p class="tags">{post.tags}</p>
				</li>
			</a>
		{/each}
	</ul>
</section>

<style>
	ul {
		margin: 0;
		padding: 0;
	}
	.posts {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(240px, 100%), 1fr));
		gap: 4rem 3rem;
		list-style: none;
		width: 100%;
		padding: 0;
		margin: 0;
	}
	.post {
		border-radius: 10px;
		box-shadow: 2px 2px 4px gray;
		padding: 10px;
		width: 100%;
		height: 100%;
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
