+++
title = "htmldjangoファイルのフォーマットに成功した"
description = "dnfolioでは、TeraというHTMLテンプレートファイルを使用している。このTeraの文法で書くとファイルタイプがhtmldjangoとなりHTMLのフォーマットが変になる。その問題を解決した。"
slug = "htmldjangoファイルのフォーマットに成功した"
draft = false
[taxonomies]
tags = ["htmldjango", "Tera", "Python", "coc-htmldjango", "djLint"]
languages = ["日本語"]
+++

## 困っていること

dnfolioでは、[Tera](https://keats.github.io/tera/) というHTMLテンプレートエンジンを利用している。
例えば、`about` ページ用の `templates/about.html` はこう書いている。

```html
{% extends "base.html" %} {% block head %}
<meta name="description" content="{{page.description}}">
<meta property="og:type" content="website">
<meta property="og:image" content="https://dnfolio.dev/icons/icon.png">
<meta property="og:title" content="{{page.title}}">
<meta property="og:description" content="{{page.description}}">
<meta property="og:site_name" content="dnfolio" />
<meta property="og:email" content="daiki48.engineer@gmail.com">
<meta property="og:url" content="https://dnfolio.dev/about">

<meta name="twitter:card" content="summary">
<meta name="twitter:site" content="@Daiki48engineer">
<meta name="twitter:title" content="{{page.title}}">
<meta name="twitter:description" content="{{page.description}}">
<meta name="twitter:image" content="https://dnfolio.dev/icons/icon.png">

{% endblock head %} {% block title %} {{ page.title }} {% endblock title %} {%
block content %}
<h1 class="title">{{ page.title }}</h1>
{{ page.content | safe }} {% endblock content %}
```

これでもまぁ読めることは読めるが、この部分が個人的に気になる。

```html
{% endblock head %} {% block title %} {{ page.title }} {% endblock title %} {%
block content %}
<h1 class="title">{{ page.title }}</h1>
{{ page.content | safe }} {% endblock content %}
```

`{%` で折り返しているのは、 `dprint` の幅を `80` に設定しているため。
`Prettier` でもこの幅で利用しているので問題無いが、変な部分で折り返しているなぁと思う。
あと、そもそも `{% endblock head %}` の部分で改行して次の行は `{% block title %}` からスタートしてほしい。
このフォーマットは `dprint` の [markup_fmt](https://dprint.dev/plugins/markup_fmt/) による結果。

こちらが現在の `dprint.json` 設定。

```json
{
  "json": {
  },
  "markdown": {
  },
  "toml": {
  },
  "malva": {
  },
  "markup": {
    "printWidth": 80,
    "formatComments": true,
    "scriptIndent": true,
    "styleIndent": true
  },
  "excludes": [
    "**/*-lock.json",
    "**/node_modules",
    "**/templates"
  ],
  "plugins": [
    "https://plugins.dprint.dev/json-0.19.4.wasm",
    "https://plugins.dprint.dev/markdown-0.17.8.wasm",
    "https://plugins.dprint.dev/toml-0.6.4.wasm",
    "https://plugins.dprint.dev/g-plane/malva-v0.11.1.wasm",
    "https://plugins.dprint.dev/g-plane/markup_fmt-v0.18.0.wasm"
  ]
}
```

## `coc-htmldjango` を発見した

私は `Neovim` の [coc.nvim](https://github.com/neoclide/coc.nvim) を利用している。この `extensions` として公開されていた。

{{ card(title="coc-htmldjango | yaegassy GitHub", url="https://github.com/yaegassy/coc-htmldjango") }}

これを導入すればフォーマット出来るのでは？と思い早速導入してみた。

## Pythonインストール

私は、`Python` を使ったことが無かったのでインストールからしていく。

{{ card(title="Python Releases for Windows | python", url="https://www.python.org/downloads/windows/") }}

環境によってインストールするファイルは異なるが、私の場合は `Download Windows installer (64-bit)` をインストールした。
インストーラーに従って進める。

インストーラーの画面でPathを通すチェックボックスもあるのでチェックを忘れずに。
チェックしなかった場合はインストール後に手動で環境変数にPathを通す必要がある。

インストール出来たか確認する。

```sh
python --version

Python 3.13.1
```

私の場合は、`pip` も一緒にインストールされていた。

```sh
pip --version

pip 24.3.1 from D:\python\Lib\site-packages\pip (python 3.13)
```

## `coc-htmldjango` をインストール

`README` の `Install` 項目を確認する。

{{ card(title="Install | coc-htmldjango", url="https://github.com/yaegassy/coc-htmldjango?tab=readme-ov-file#install") }}

私は、すでに `coc.nvim` ユーザーなので `CocInstall` コマンドでインストールした。

```sh
:CocInstall coc-htmldjango
```

[coc-html](https://github.com/neoclide/coc-html) も一緒にインストールする必要があるようだ。

> Recommended coc-extensions to install together\
> [coc-html](https://github.com/neoclide/coc-html)

```sh
:CocInstall coc-html
```

## 対象のファイルを開いてフォーマットを実行する

先ほどの `templates/about.html` をフォーマットしてみる。

フォーマット前がこちら。

```
{% extends "base.html" %} {% block head %}
<meta name="description" content="{{page.description}}">
<meta property="og:type" content="website">
<meta property="og:image" content="https://dnfolio.dev/icons/icon.png">
<meta property="og:title" content="{{page.title}}">
<meta property="og:description" content="{{page.description}}">
<meta property="og:site_name" content="dnfolio" />
<meta property="og:email" content="daiki48.engineer@gmail.com">
<meta property="og:url" content="https://dnfolio.dev/about">

<meta name="twitter:card" content="summary">
<meta name="twitter:site" content="@Daiki48engineer">
<meta name="twitter:title" content="{{page.title}}">
<meta name="twitter:description" content="{{page.description}}">
<meta name="twitter:image" content="https://dnfolio.dev/icons/icon.png">

{% endblock head %} {% block title %} {{ page.title }} {% endblock title %} {%
block content %}
<h1 class="title">{{ page.title }}</h1>
{{ page.content | safe }} {% endblock content %}
```

フォーマットしてみる。

{{ card(title="Commands | coc-htmldjango", url="https://github.com/yaegassy/coc-htmldjango?tab=readme-ov-file#commands") }}

```sh
:CocCommand htmldjango.djlint.format
```

すると

```html
{% extends "base.html" %}
{% block head %}
    <meta name="description" content="{{ page.description }}">
    <meta property="og:type" content="website">
    <meta property="og:image" content="https://dnfolio.dev/icons/icon.png">
    <meta property="og:title" content="{{ page.title }}">
    <meta property="og:description" content="{{ page.description }}">
    <meta property="og:site_name" content="dnfolio" />
    <meta property="og:email" content="daiki48.engineer@gmail.com">
    <meta property="og:url" content="https://dnfolio.dev/about">
    <meta name="twitter:card" content="summary">
    <meta name="twitter:site" content="@Daiki48engineer">
    <meta name="twitter:title" content="{{ page.title }}">
    <meta name="twitter:description" content="{{ page.description }}">
    <meta name="twitter:image" content="https://dnfolio.dev/icons/icon.png">
{% endblock head %}
{% block title %}
    {{ page.title }}
{% endblock title %}
{%
block content %}
<h1 class="title">{{ page.title }}</h1>
{{ page.content | safe }}
{% endblock content %}
```

フォーマット出来た！\
めちゃくちゃ見やすい。

## `dotfiles` を編集

毎回このフォーマットコマンドを打つのはめんどくさい...

```sh
:CocCommand htmldjango.djlint.format
```

よって `coc.nvim` の設定を編集する。

```lua
vim.api.nvim_create_user_command("Djlint", function()
  vim.fn.CocAction("runCommand", "htmldjango.djlint.format")
end, {})
```

詳しい設定内容は `dotfiles` を確認してほしい。

{{ card(title="dotfiles | Daiki48 GitHub", url="https://github.com/Daiki48/dotfiles/commit/aefa9135db177098dc5fc62be38dff27da197608#diff-0537d20ea404c397a15fe03c267943e61d452bdbebc680082ffbc7fce1642f38R208") }}

今回の `htmldjango` フォーマットには [djLint](https://www.djlint.com/) を利用している。

テンプレートエンジンは `Tera` しか利用したことが無いが、今後別のテンプレートエンジンでフォーマット出来ない場合にこの記事を参考にしようと思う。
