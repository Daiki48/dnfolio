<script lang="ts">
	import { formatDate } from '$lib/utils';
	import type { PageData } from './$types';
	export let data: PageData;

	let selectedTag: string = '';
</script>

<h1>タグ</h1>

<section>
	<ul class="list-tags">
		<li class="list-tag"><button on:click={() => (selectedTag = '')}>全記事表示</button></li>
		{#each data.posts as post}
			{#each post.tags as tag}
				<li class="list-tag"><button on:click={() => (selectedTag = tag)}>{tag}</button></li>
			{/each}
		{/each}
	</ul>
</section>

<h1>記事一覧</h1>

<section>
	<ul class="posts">
		{#each data.posts as post (post.slug)}
			{#if !selectedTag || post.tags.includes(selectedTag)}
				<li class="post">
					<a href={`blog/${post.category}/${post.slug}`} class="title">
						<h1>{post.title}</h1>
					</a>
					<div class="middle-container">
						<div class="date-container">
							{#if post.updatedAt}
								<p class="update-date">更新日 : {formatDate(post.updatedAt)}</p>
							{/if}
							<p class="create-date">公開日 : {formatDate(post.createdAt)}</p>
						</div>
					</div>
					<p class="description">{post.description}</p>
					<div class="tag-container">
						{#each post.tags as tag}
							<p class="tags">{tag}</p>
						{/each}
					</div>
				</li>
			{/if}
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

	.tag-container {
		display: flex;
	}
	.tags {
		padding: 4px;
		margin: 4px;
		border-radius: 4px;
		background-color: #c3c4c0;
	}

	.list-tags {
		display: flex;
		list-style: none;
	}

	.list-tag button {
		flex-grow: 1;
		flex-shrink: 1;
		margin-right: 20px;
		border-radius: 10px;
		border: none;
		padding: 4px 6px;
		font-size: 12px;
		cursor: pointer;
	}

	.list-tag button:hover {
		background-color: rgb(208, 249, 195);
	}
</style>
