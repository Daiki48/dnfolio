import { posts } from '$lib/postsList';
import type { PageLoad } from '../$types';

export const load: PageLoad = () => {
	return { posts };
};
