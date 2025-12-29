+++
title = "Zolaのshortcodesを使ってみた"
slug = "zola-shortcodes"
description = "Zolaにshortcodesという機能があるらしい。使ってみよう。"
[taxonomies]
tags = ["Zola"]
languages = ["ja"]
+++

## Youtube動画の埋め込み

これは、Zolaの公式でサンプルコードがあるので使ってみます。

`templates/shortcodes/youtube.html` を追加します。

```html
<div {% if class %}class="{{class}}" {% endif %}>
  <iframe
    src="https://www.youtube.com/embed/{{id}}{% if autoplay %}?autoplay=1{% endif %}"
    webkitallowfullscreen
    mozallowfullscreen
    allowfullscreen
  >
  </iframe>
</div>
```

そして、Markdownではこのように書いてみます。

```md
{{ youtube(class="yt", id="gdIuTqrFPx4") }}
```

すると...

<iframe width="560" height="315" src="https://www.youtube.com/embed/gdIuTqrFPx4" frameborder="0" allowfullscreen></iframe>

表示出来ました！

## リンクカードの埋め込み

Youtubeのコードを参考に、`iframe` タグで実装を試みました。

`templates/shortcodes/card.html` を追加します。

```html
<div {% if class %}class="{{class}}" {% endif %}>
  <iframe
    style="width: 600px; max-width: 100%; display: block; box-shadow: 0 0 10px rgba(0, 0, 0, 0.5)"
    title="{{title}}"
    src="{{url}}"
    frameborder="0"
    scrolling="no"
  >
  </iframe>
</div>
```

そして、Markdownも書いてみます。

```md
[Deno公式サイト](https://deno.com)
```

コンソールでエラーが発生していた。

```sh
Refused to display 'https://deno.com/' in a frame because it set 'X-Frame-Options' to 'deny'.
```

セキュリティ上、デフォルトで `deny` が設定されているようだ。

[object から iframe まで — 一般的な埋め込み技術 | MDN](https://developer.mozilla.org/ja/docs/Learn_web_development/Core/Structuring_content/General_embedding_technologies)

セキュリティ知識無しでiframeを使うのなんか怖くなってきたので別の方法を探してみよう。一旦リンクカードは保留で。

[Deno公式サイト](https://deno.com)
