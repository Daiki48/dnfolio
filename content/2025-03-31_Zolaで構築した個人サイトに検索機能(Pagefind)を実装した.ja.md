+++
title = "Zolaで構築した個人サイトに検索機能(Pagefind)を実装した"
description = "このサイトはZolaというRust製SSGで構築している。公式ドキュメントでは書かれていないPagefindによる検索機能の実装が出来た。"
slug = "Zolaで構築した個人サイトに検索機能(Pagefind)を実装した"
draft = false
[taxonomies]
tags = ["Zola", "Pagefind"]
languages = ["日本語"]
+++

## 検索機能を実装したい！

Zolaの公式ドキュメントによると、検索機能に関する記載がある。

{{ card(title="Search | Zola", url="https://www.getzola.org/documentation/content/search/") }}

このサイトでは、日本語と英語で記事を書くことが多い。現状、どっちかというと日本語の方が多い。

Zolaは、ビルド時に日本語用辞書も作ることが出来るみたい。
ただし、日本語辞書は大きいみたい。

> Note: By default, Chinese and Japanese search indexing is not included. You can include the support by building zola using cargo build --features indexing-ja --features indexing-zh. Please also note that, enabling Chinese indexing will increase the binary size by approximately 5 MB while enabling Japanese indexing will increase the binary size by approximately 70 MB due to the incredibly large dictionaries.

{{ card(title="Configuration | Multilingual sites", url="https://www.getzola.org/documentation/content/multilingual/#configuration") }}

他に全文検索エンジンで良いのが無いか探してみたら [Pagefind](https://pagefind.app/) を見つけた。

## Pagefindについて

詳しくは公式ドキュメントを確認。

{{ card(title="Pagefind", url="https://pagefind.app") }}

この部分を見てから、Zolaでも使えるんじゃないかと思った。

> Pagefind runs after Hugo, Eleventy, Jekyll, Next, Astro, SvelteKit, or any other website framework. The installation process is always the same: Pagefind only requires a folder containing the built static files of your website, so in most cases no configuration is needed to get started.

> Pagefindは、Hugo、Eleventy、Jekyll、Next、Astro、SvelteKit、またはその他のWebサイトフレームワークの後に実行されます。Pagefindは、あなたのウェブサイトのビルドされた静的ファイルを含むフォルダを必要とするだけなので、ほとんどの場合、開始するための設定は必要ありません。

つまり、 `zola build` で出力された後に処理すれば良さそう。

## 実装内容

- どのページでも検索出来るようにする
- モーダルで表示する

## searchコンポーネントを作成

`templates/components/search.html` に作った。

```html
<button class="search-button">
  <a href="#modal">Search</a>
</button>
<div id="modal" class="modal">
  <div class="modal-content">
    <a href="#" class="modal-close">&times;</a>
    <link href="/pagefind/pagefind-ui.css" rel="stylesheet" />
    <script src="/pagefind/pagefind-ui.js"></script>
    <div id="search"></div>
    <script>
      window.addEventListener("DOMContentLoaded", (event) => {
        new PagefindUI({
          element: "#search",
          showSubResults: true,
          showImages: false,
          excerptLength: 15,
          translations: {
            placeholder: "Search in dnfolio",
            zero_results: "Couldn't find [SEARCH_TERM]",
          },
          highlightParam: "highlight",
        });
      });
    </script>
    <script>
      document.addEventListener('DOMContentLoaded', function() {
        const modal = document.getElementById('modal');

        modal.addEventListener('click', function(event) {
          if (event.target === modal) {
            window.location.hash = '';
          }
        });
      });
    </script>
  </div>
</div>
```

`PagefindUI` 内の以下の設定は公式ドキュメントで確認しながら設定した。

```html
<script>
  window.addEventListener("DOMContentLoaded", (event) => {
    new PagefindUI({
      element: "#search",
      showSubResults: true,
      showImages: false,
      excerptLength: 15,
      translations: {
        placeholder: "Search in dnfolio",
        zero_results: "Couldn't find [SEARCH_TERM]",
      },
      highlightParam: "highlight",
    });
  });
</script>
```
| 設定値 | 内容 |
| :---- | :---- |
| [element](https://pagefind.app/docs/ui/#element) | `id` 名になる。CSSでカスタマイズしようとした時に必要になる。 |
| [showSubResults](https://pagefind.app/docs/ui/#show-sub-results) | 見出しごとに結果を出力するかどうか。 `true` にしているので見出しごとに出力するようにしている。 |
| [showImages](https://pagefind.app/docs/ui/#show-images) | ページごとに画像を設定していないので `false` に設定している。 |
| [excerptLength](https://pagefind.app/docs/ui/#excerpt-length) | 抜粋される内容の長さ。あまり長くなくて良いので `15` にした。 |
| [translations](https://pagefind.app/docs/ui/#translations) | 検索欄未入力状態の時に表示する文字列が `placeholder` 。結果が見つからなかった時に表示する文字列が `zero_results` 。 |
| [highlightParam](https://pagefind.app/docs/search-config/#highlight-query-parameter) | デフォルトと同じだけど明記してる。 |

モーダルの外をクリックしたらモーダルを閉じる処理も追加した。

```html
    <script>
      document.addEventListener('DOMContentLoaded', function() {
        const modal = document.getElementById('modal');

        modal.addEventListener('click', function(event) {
          if (event.target === modal) {
            window.location.hash = '';
          }
        });
      });
    </script>
```

疑似クラスでなんちゃって動作してるので、JavaScriptで書き直すことも検討してる。

この `templates/components/search.html` を `templates/base.html` に追加する。

```xhtml
<!DOCTYPE html>
<html lang="en">
  <head prefix="og: https://ogp.me/ns#">
  <!-- ヘッダー -->
  </head>
  <body>
    {% include "components/header.html" %}
    <div class="search-pagefind">{% include "components/search.html" %}</div>
    <section class="section" data-pagefind-body>
      <div class="container">
        {% block content %}{% endblock %}
      </div>
    </section>
    {% include "components/footer.html" %}
  </body>
</html>
```

`data-pagefind-body` を使って `section` タグ以下のみインデックスするようにした。

{{ card(title="Limiting what sections of a page are indexed | Pagefind", url="https://pagefind.app/docs/indexing/#limiting-what-sections-of-a-page-are-indexed") }}

## モーダルとして表示するUI

`sass/_search.scss` にUIを書いていく。

```css
@use "vars";

$modal-bg-color: rgba(0, 0, 0, 0.5);
$modal-content-bg-color: rgba(192, 198, 207, 1);
$modal-width: 80%;
$modal-max-width: 600px;
$modal-padding: 30px;

.search-button {
  padding: 10px 20px;
  background-color: vars.$secondary-bg-color;
  color: vars.$primary-fg-color;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  font-size: 16px;
  text-decoration: none;

  a {
    color: white;
    text-decoration: none;
  }

  &:hover {
    color: vars.$secondary-fg-color;
  }
}

.modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: $modal-bg-color;
  opacity: 0;
  visibility: hidden;
  transition:
    opacity 0.3s ease-in-out,
    visibility 0s linear 0.3s;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: auto;

  &:target {
    opacity: 1;
    visibility: visible;
    transition:
      opacity 0.3s ease-in-out,
      visibility 0s linear 0s;
  }

  .modal-content {
    background-color: $modal-content-bg-color;
    width: $modal-width;
    max-width: $modal-max-width;
    max-height: 80vh;
    padding: $modal-padding;
    border-radius: 5px;
    position: relative;
    overflow-y: auto;
  }

  .modal-close {
    position: absolute;
    top: 10px;
    right: 10px;
    font-size: 24px;
    font-weight: bold;
    color: #333;
    text-decoration: none;

    &:hover {
      color: #000;
    }
  }
}

.search-pagefind {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

@media (max-width: 768px) {
  .search-pagefind {
    display: none;
  }
}
```

Zolaで構築した時にCSS拡張言語のSASSを有効にしていた。

そして `sass/style.css` に追加する。

```css
@use "vars";
@use "tags";
@use "classes";
@use "card";
@use "codeblocks";
@use "search";
```

## 完成した様子

動画を撮ってみた。

<video src="/content/Zolaで構築した個人サイトに検索機能(Pagefind)を実装した/dnfolio-search-modal-ui.mp4" controls="true" width="100%"></video>

もっとすごい検索モーダル作りたかったけど、今の自分にはこれで精一杯だった...

CSSの知識を付けるとWebサイトが華やかになるだろうからちゃんと時間取って学びたいなぁ。
