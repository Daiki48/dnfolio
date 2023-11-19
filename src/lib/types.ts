import type { Tags } from './taglist';

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
