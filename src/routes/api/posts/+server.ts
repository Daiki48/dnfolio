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
		console.log('parts is ', parts);

		if (file && typeof file === 'object' && 'metadata' in file && slug && category) {
			const metadata = file.metadata as Omit<Post, 'slug'>;
			const post = { ...metadata, slug, category } satisfies Post;
			console.log("Publish status is ", post.published);
			post.published && posts.push(post);
			console.log("Pushed post is ", post);
		}
	}

	posts.sort((first, second) => {
		const dateDiff = new Date(second.createdAt).getTime() - new Date(first.createdAt).getTime();
		if (dateDiff !== 0) {
			return dateDiff;
		} else {
			return parseInt(second.slug.slice(0, 3)) - parseInt(first.slug.slice(0, 3));
		}
	});

	console.log("Return before ", posts);

	return posts;
}

export async function GET() {
	const posts = await getPosts();
  console.log("posts is ", posts);
	return json(posts);
}
