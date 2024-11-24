<script setup lang="ts">
	import { useData } from 'vitepress';

	const { frontmatter } = useData();
	// const tags: string[] = frontmatter.tags;
</script>

<template>
	<div class="vp-doc">
		<h1>{{frontmatter.title}}</h1>
		<p>{{frontmatter.description}}</p>
		<ul>
			<li v-for="tag in frontmatter.tags" class="tag">
				#{{ tag }}
			</li>
		</ul>
	</div>
</template>

<style>
	ul {
		padding-bottom: 2rem;
	}

	.tag {
		display: flex;
		justify-content: start;
		align-items: center;
		list-style: none;
	}
</style>
