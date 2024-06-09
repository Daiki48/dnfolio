<script lang="ts">
	import { formatDate } from '$lib/utils';
	import type { PageData } from './$types';
	export let data: PageData;

	let selectedTag: string = '';

	let tagCounts: Record<string, number> = data.posts
		.flatMap((post) => post.tags)
		.reduce((acc: Record<string, number>, tag) => {
			acc[tag] = (acc[tag] || 0) + 1;
			return acc;
		}, {});

	let uniqueTags = Object.keys(tagCounts);
</script>

<svelte:head>
	<title>Blog | Dnfolio</title>
	<meta
		name="description"
		content="Daikiのブログです。雑多な内容からプログラミングに関する記事まで幅広く書きます。"
	/>
	<meta property="og:title" content="Dnfolio's blog" />
	<meta property="og:description" content="Daikiの趣味プログラミング" />
	<meta property="og:site_name" content="Dnfolio" />
	<meta property="og:url" content="https://dnfolio.dev/blog" />
	<meta property="og:image" content="/icon.webp" />
</svelte:head>

<div class="tag-list-container">
	<h1>タグ</h1>
	{#if selectedTag}
		<p class="selected-tag">{selectedTag}</p>
		<button class="tag-clear-button" on:click={() => (selectedTag = '')}>
			<span class="material-symbols-outlined"> close </span>
		</button>
	{/if}
</div>

<section>
	<div class="list-tags">
		{#each uniqueTags as tag}
			<span class="list-tag"
				><button on:click={() => (selectedTag = tag)}
					>{tag}<span class="tag-counts">{tagCounts[tag]}</span></button
				></span
			>
		{/each}
	</div>
</section>

<h1>記事一覧</h1>

<section>
	<div class="posts">
		{#each data.posts as post (post.slug)}
			{#if !selectedTag || post.tags.includes(selectedTag)}
				<span class="post">
					<a href={`blog/${post.category}/${post.slug}`} class="title">
						<h1>{post.title}</h1>
					</a>
					<div class="middle-container">
						<div class="date-container">
							{#if post.updatedAt}
								<p class="update-date">
									{formatDate(post.updatedAt)}
									<span class="material-symbols-outlined"> edit </span>
								</p>
							{:else}
								<p class="create-date">{formatDate(post.createdAt)}</p>
							{/if}
						</div>
					</div>
					<p class="description">{post.description}</p>
					<div class="tag-container">
						{#each post.tags as tag}
							<p class="tags">{tag}</p>
						{/each}
					</div>
				</span>
			{/if}
		{/each}
	</div>
</section>

<style>
	section {
		display: flex;
		padding: 0;
		margin: 0;
	}

	.posts {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(240px, 100%), 1fr));
		gap: 4rem 3rem;
		width: 100%;
		padding: 0;
		margin: 0;
	}

	.post {
		border-radius: 10px;
		box-shadow: 0 0 8px gray;
		padding: 10px;
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.post:hover {
		background-color: #f7f5f5;
	}

	a {
		text-decoration: none;
		color: rgb(3, 4, 5) !important;
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
		display: block;
	}

	.date-container .update-date {
		display: flex;
		align-items: center;
	}

	.update-date span {
		margin-left: 2px;
	}

	.tag-list-container {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
	}

	.tag-list-container h1 {
		padding-right: 40px;
	}

	.selected-tag {
		border-bottom: 2px solid rgba(209, 225, 105, 0.8);
	}

	.tag-container {
		display: flex;
		margin-top: auto;
	}

	.tags {
		padding: 4px;
		margin: 4px;
		border-radius: 4px;
		background-color: #c3c4c0;
	}

	.list-tags {
		display: flex;
		flex-wrap: wrap;
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

	.create-date {
		margin-right: 10px;
	}

	.tag-clear-button {
		background-color: rgba(255, 254, 250, 1);
		border: none;
		cursor: pointer;
	}

	.tag-counts {
		color: green;
		font-weight: bold;
		border-radius: 50px;
		margin: 0.4rem;
	}

	.title h1 {
		font-size: 20px;
	}

	@media(max-width: 1000px) {
		.posts {
			padding: 0;
			margin: 0 1rem;
			max-width: 90%;
		}
	}
</style>
