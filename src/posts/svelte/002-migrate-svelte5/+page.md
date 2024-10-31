---
title: dnfolioのSvelteをv5へ
description: dnfolioでは、プロジェクト作成時からSvelte v4を使用していました。しかし、先日v5の安定板がリリースされたため、今回はv4からv5への移行した手順を記事にします。
createdAt: '2024-11-1'
tags:
  - Svelte
published: true
---

<script>
  import HL from '$components/modules/HL.svelte';
  import Img from '$components/modules/Img.svelte';
</script>

<HL el="h2" text="Svelteのv5がリリース" />

先日、Svelteがv5をリリースしました。おめでとう〜

https://svelte.dev/blog/svelte-5-is-alive

本サイトも、Svelteで作成しているため、v4からv5に移行してみようと思います。  
といっても、上記ブログの冒頭で書いてある通り、v5は下位互換性があるためv4ユーザーは安心して移行出来ると思います。

<HL el="h2" text="package.json編集" />

`package.json` を編集しました。  
あまり更新する必要が無さそうです。

<Img src="/images/svelte/002-migrate-svelte5/update-svelte5.webp" alt="migrate svelte v5" />

<HL el="h2" text="開発サーバー起動" />

Runesを試す前に、一旦動くか開発サーバーを起動して確認します。

`node_modules` 、 `.svelte-kit` 、 `bun.lockb` を削除して依存関係をインストールします。

```shell
bun install
```

そして開発サーバー起動します。

```shell
bun run dev
```

動作しました。  
この時点で一安心。次にv5新機能のRuneを少し試してみます。

<HL el="h2" text="Runeお試し" />

下記は、`export let` を使用して `data` を取得しています。

```svelte
<script lang="ts">
	import { formatDate } from '$lib/utils';
	import type { PageData } from './$types';
	export let data: PageData;

	let selectedTag: string = '';

	let tagCounts: Record<string, number> = data.posts
		.flatMap((post) => post.tags)
		.reduce((acc: Record<string, number>, tag) => {
			acc[tag] = (acc[tag] || 0) + 1;
			return acc;
		}, {});

	let uniqueTags = Object.keys(tagCounts);
</script>
```

これを、[マイグレーションガイド](https://svelte.dev/docs/svelte/v5-migration-guide#Reactivity-syntax-changes-export-let-$props) に従って移行してみます。

`export let` は `$props()` に、`let` で定義していた可変な変数は `$state()` で書き直しました。これがRuneです。

```svelte
<script lang="ts">
	import { formatDate } from '$lib/utils';
	// import type { PageData } from './$types';
	// export let data: PageData;
	let { data } = $props();

	// let selectedTag: string = '';
	let selectedTag: string = $state('');

	let tagCounts: Record<string, number> = data.posts
		.flatMap((post) => post.tags)
		.reduce((acc: Record<string, number>, tag) => {
			acc[tag] = (acc[tag] || 0) + 1;
			return acc;
		}, {});

	let uniqueTags = Object.keys(tagCounts);
</script>
```

下記の `on:click` は、 `onclick` に変更です。

```html
<button class="tag-clear-button" on:click="{()" ="">(selectedTag = '')}></button>
```

`on:click` が見慣れているためか、今はまだ違和感があります。

```html
<button class="tag-clear-button" onclick="{()" ="">(selectedTag = '')}></button>
```

<HL el="h2" text="v5使いやすそう" />

[Svelteの公式サイト](https://svelte.dev) が見やすくなりました。(個人的に)  
どのライブラリ、フレームワークを利用するにせよ公式ドキュメントが見やすいだけで幸福度が上がります。  
[SvelteKitのドキュメント](https://svelte.dev/docs/kit/introduction)も含まれるようになったっぽいですね。

仕事でもプライベートでも利用しているフレームワークなのでRuneを使い込んで慣れたいと思います。
