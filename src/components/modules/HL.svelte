<!-- このタグを利用して書いた見出しはidを付与する。 -->
<script lang="ts">
	export let text: string = '';
	export let el: string = '';
</script>

{#if el === 'h2'}
	<h2 id={text}>{text}</h2>
{:else if el === 'h3'}
	<h3 id={text}>{text}</h3>
{:else if el === 'h4'}
	<h4 id={text}>{text}</h4>
{/if}
