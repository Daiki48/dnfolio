export type Tags = 'sveltekit' | 'svelte';

export type Post = {
	title: string;
	slug: string;
	description: string;
	createdAt: string;
	updatedAt: string | undefined;
	category: string;
	tags: Tags[];
	published: boolean;
};
