+++
title = "workers-rsを試してみた"
slug = "workers-rs-trial"
description = "Cloudflare workersにRustアプリをデプロイしてみたいのでworkers-rsクレートを試してみる。"
draft = false
[taxonomies]
tags = ["Cloudflare", "workers-rs"]
languages = ["ja"]
+++

## `workers-rs` とは

`WebAssembly` 経由で `Cloudflare` を動かす `Rust` のクレートっぽい。

> Work-in-progress ergonomic Rust bindings to Cloudflare Workers environment. Write your entire worker in Rust!

{{ card(title="workers-rs | GitHub", url="https://github.com/cloudflare/workers-rs") }}

## プロジェクトを作成

`README` を流し見して早速触ってみる。まずはプロジェクトを作成する。

{{ card(title="Getting started | workers-rs GitHub", url="https://github.com/cloudflare/workers-rs?tab=readme-ov-file#getting-started") }}

`Axum` のテンプレートがあったので選択した。

```sh
cargo generate cloudflare/workers-rs
⚠️   Favorite `cloudflare/workers-rs` not found in config, using it as a git repository: https://github.com/cloudflare/workers-rs.git
✔ 🤷   Which template should be expanded? · templates\axum
🤷   Project Name: workers-rs-sample
🔧   Destination: D:\dev\sample\workers-rs-sample ...
🔧   project-name: workers-rs-sample ...
🔧   Generating template ...
🔧   Moving generated files into: `D:\dev\sample\workers-rs-sample`...
🔧   Initializing a fresh Git repository
✨   Done! New project created D:\dev\sample\workers-rs-sample
```

`cd workers-rs-sample` として作成したプロジェクトを確認していく。

## プロジェクトの構成

シンプルだった。

```
.git/
src/
.gitignore
Cargo.toml
wrangler.toml
```

### `Cargo.toml`

`authors` 情報がすでに入ってた。

```toml
[package]
name = "workers-rs-sample"
version = "0.1.0"
edition = "2021"
authors = ["Daiki48 <daiki48.engineer@gmail.com>"]

[package.metadata.release]
release = false

# https://github.com/rustwasm/wasm-pack/issues/1247
[package.metadata.wasm-pack.profile.release]
wasm-opt = false

[lib]
crate-type = ["cdylib"]

[dependencies]
worker = { version = "0.5.0", features = ['http', 'axum'] }
worker-macros = { version = "0.5.0", features = ['http'] }
axum = { version = "0.7", default-features = false }
tower-service = "0.3.2"
console_error_panic_hook = { version = "0.1.1" }
```

### `wrangler.toml`

これが `Cloudflare` 関連の `CLI` である `wrangler` 用の設定。

```toml
name = "workers-rs-sample"
main = "build/worker/shim.mjs"
compatibility_date = "2025-01-13"

[build]
command = "cargo install -q worker-build && worker-build --release"
```

### `src/lib.rs`

最初に `Axum` テンプレートを選択したので簡単なコードがすでに書かれている。

```rust
use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new().route("/", get(root))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}
```

## 動作確認

`wrangler` をインストールしていない場合は公式の手順でインストールしておく。

{{ card(title="Install/Update Wrangler | Cloudflare Docs", url="https://developers.cloudflare.com/workers/wrangler/install-and-update/") }}

ローカルサーバーの起動。

```sh
npx wrangler dev
```

するとビルド処理が進行する。

```sh
npx wrangler dev
Need to install the following packages:
wrangler@3.101.0
Ok to proceed? (y)

npm warn deprecated rollup-plugin-inject@3.0.2: This package has been deprecated and is no longer maintained. Please use @rollup/plugin-inject.
npm warn deprecated sourcemap-codec@1.4.8: Please use @jridgewell/sourcemap-codec instead

 ⛅️ wrangler 3.101.0
--------------------

Running custom build: cargo install -q worker-build && worker-build --release
╭──────────────────────────────────────────────────────────────────────────────────────────────────╮
│  [b] open a browser, [d] open devtools, [l] turn off local mode, [c] clear console, [x] to exit  │
╰──────────────────────────────────────────────────────────────────────────────────────────────────╯
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
   Compiling unicode-ident v1.0.14

...

    Finished `release` profile [optimized] target(s) in 20.49s
[INFO]: ⬇️  Installing wasm-bindgen...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: ✨   Done in 23.13s
[INFO]: 📦   Your wasm pkg is ready to publish at D:\dev\sample\workers-rs-sample\build.
Installing esbuild...

The file src changed, restarting build...
Running custom build: cargo install -q worker-build && worker-build --release
[wrangler:inf] Ready on http://127.0.0.1:8787
╭──────────────────────────────────────────────────────────────────────────────────────────────────╮
│  [b] open a browser, [d] open devtools, [l] turn off local mode, [c] clear console, [x] to exit  │
╰──────────────────────────────────────────────────────────────────────────────────────────────────╯
```

`[b] open a browser` を選択するとブラウザで `http://127.0.0.1:8787/` を開き(もしくは勝手に開かれている) `Hello Axum!` が表示されている。

開発環境での動作確認は大丈夫そう。

あとはゾーンとかを設定して `npx wrangler deploy` すれば良さそう。この処理はまだ試していない。

`Axum` とか利用してアプリっぽいのも作ってみたいなぁ。
