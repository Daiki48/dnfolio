+++
title = "dnfolioのUIを若干変更した"
description = "個人サイトdnfolioのUIを若干変更した。ちょっと見やすくなった。"
slug = "dnfolioのUIを若干変更した"
draft = false
[taxonomies]
tags = ["Zola", "CSS"]
languages = ["日本語"]
+++

## `body` の幅を変更

スマホで表示時の `body` 要素を変更した。

```css
@media (max-width: 768px) {
  body {
    max-width: 100%;
    width: 90%;
  }
}
```

変更前は `width: 80%;` としていたが、これだと一行あたりの表示文字数が少なくてスマホによっては縦書きぐらいスクロール頻度が多かった。

## 引用 `blockquote` の余白を小さくした

`blockquote` の `padding` と `margin` のどちらも小さくした。
当時は余白多めの方が読みやすかったが、最近自分で見返す際ちょっと読みにくさを感じたのが理由。

```css
.container blockquote {
  margin: 10px 4px;
  padding: 1px 10px;
  border-left: 6px solid vars.$primary-fg-color;
  background-color: vars.$secondary-bg-color;
  color: vars.$secondary-fg-color;
  border-radius: 4px;
}

.container blockquote p {
  line-height: 20px;
}
```

## `blockquote` 内の `code` に背景色を追加した

`blockquote` 内で `code` 要素を利用することが多いのだが、背景色が同じなので強調出来てない。初期実装時に忘れてた...

```css
.container blockquote code {
  background-color: vars.$primary-bg-color;
}
```

## コードブロック内のフォントを変更

これまでは、通常テキストと同様の `BIZ UDPGothic` だった。
記事の抑揚が無くなってしまったので、 `JetBrainsMono Nerd Font Mono` と `monospace` を追加した。
第一優先は `JetBrainsMono Nerd Font Mono` を指定。私が普段利用しているプログラミング用フォント。見やすい！

```css
pre code {
  font-family: JetBrainsMono Nerd Font Mono, monospace, BIZ UDPGothic;
  line-height: 22px;
  background-color: unset;
  padding: unset;
  border-radius: unset;
  word-wrap: unset;
  font-size: 1rem;
}
```

`code` 単体のフォントも同様に変更した。

```css
code {
  font-family: JetBrainsMono Nerd Font Mono, monospace, BIZ UDPGothic;
  background-color: vars.$secondary-bg-color;
  padding: 4px;
  border-radius: 4px;
  word-wrap: break-word;
  font-size: 0.8rem;
}
```

## 記事ページ冒頭に表示するタグをレスポンシブ対応

付与するタグの数が増えた時にUIが壊れていた。要素が壊れないように折り返すよう変更した。

```css
.page-tags {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-wrap: wrap;
  gap: 20px;
  padding-left: 0;
}
```

## おわりに

全体的に見た目を更新した。
コードブロック内のフォントも通常テキストと同じだと文章に抑揚が無くて見ずらかった。修正して個人的に良くなった。
