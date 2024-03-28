export const ssr = true;
export const prerender = true;

import type { Post } from '$lib/types';
import type { PageLoad } from '../$types';

export const load: PageLoad = async ({ fetch }) => {
	console.log('Start response');
	const response = await fetch('api/posts');
	console.log('End response');
	const posts: Post[] = await response.json();
	console.log('After response posts is ', posts);
	return { posts };
};
