---
title: タグ一覧をレスポンシブに対応
description: タグ一覧がモバイルサイズで見ると折り返し表示に未対応だったため、レスポンシブに折り返すようにしました。
createdAt: '2024-4-13'
tags:
  - アナウンス
published: true
---

<script>
  import Img from '$components/modules/Img.svelte';
  import HL from '$components/modules/HL.svelte';
</script>

<HL el="h2" text="以前まで" />

タグが折り返さないため、他の要素が小さく表示されていた。  
縦長に表示されたり、見た目が壊れていた。

<Img src="/images/infomation/002-support-tags-wrap/dnfolio-iphone-screen-before.png" alt="before" />

<HL el="h2" text="修正後" />

画面サイズに合わせて折り返して表示出来るようになった。

<Img src="/images/infomation/002-support-tags-wrap/dnfolio-iphone-screen-after.png" alt="after" />

下記の設定を追加して対応した。

```css
flex-wrap: wrap;
```
