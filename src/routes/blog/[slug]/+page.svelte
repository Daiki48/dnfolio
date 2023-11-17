<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<h1>Posts</h1>
<p>{data.props.post.title}</p>
<p>{data.props.post.slug}</p>
<p>{data.props.post.content}</p>
