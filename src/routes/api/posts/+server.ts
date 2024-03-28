import { json } from '@sveltejs/kit';
import type { Post } from '$lib/types';

async function getPosts() {
	let posts: Post[] = [];

	const paths = import.meta.glob('/src/posts/**/**/*.svx', { eager: true });

	for (const path in paths) {
		const file = paths[path];
		const parts = path.split('/');
		const slug = parts.at(-2);
		const category = parts.at(-3);
		console.log('slug in server : ', slug);

		if (file && typeof file === 'object' && 'metadata' in file && slug && category) {
			const metadata = file.metadata as Omit<Post, 'slug'>;
			const post = { ...metadata, slug, category } satisfies Post;
			post.published && posts.push(post);
		}
	}

	posts = posts.sort(
		(first, second) => new Date(second.createdAt).getTime() - new Date(first.createdAt).getTime()
	);

	return posts;
}

export async function GET() {
	const posts = await getPosts();
  console.log("posts is ", posts);
	return json(posts);
}
