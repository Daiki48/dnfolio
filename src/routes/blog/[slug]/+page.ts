import { error } from '@sveltejs/kit';
import { posts } from '$lib/postsList';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	const post = posts.find((post) => post.slug === params.slug);

	console.log('post : ', post);
	console.log('params : ', params);
	if (!post) throw error(404);
	return { props: { post } };
};
