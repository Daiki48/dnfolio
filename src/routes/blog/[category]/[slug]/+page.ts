import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	try {
		const post = await import(`../../../../posts/${params.category}/${params.slug}/+page.md`);
		// console.log('post : ', post);
		return {
			...post.metadata,
			content: post.default
		};
	} catch (e) {
		throw error(404, `Not found : ${params.slug}`);
	}
};
