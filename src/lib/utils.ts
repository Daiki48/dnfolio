import fs from 'fs';
import path from 'path';

export function getPostSlugs() {
	const postsDirectory = path.join(process.cwd(), 'src/posts');
	return fs.readdirSync(postsDirectory);
}
