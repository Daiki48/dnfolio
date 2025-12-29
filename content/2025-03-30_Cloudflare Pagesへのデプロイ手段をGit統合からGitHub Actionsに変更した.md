+++
title = "Cloudflare Pagesへのデプロイ手段をGit統合からGitHub Actionsに変更した"
slug = "cloudflare-pages-github-actions"
description = "個人サイトのデプロイ手順を大きく変更した。GitHub ActionsでCloudflare Pagesにデプロイ出来ることを知ったので実装した。"
draft = false
[taxonomies]
tags = ["Cloudflare Pages", "GitHub Actions", "Zola", "cargo-make"]
languages = ["ja"]
+++

## 今までの構成

Zola公式ドキュメントにあるCloudflare Pagesへのデプロイ手順を実行していた。

{{ card(title="Cloudflare Pages | Deployment", url="https://www.getzola.org/documentation/deployment/cloudflare-pages/") }}

GitHubのリポジトリをCloudflare Pagesに統合する方法だ。
ただ、これだと `zola build` しか出来ない。

## Git統合の設定を解除しておく

私の場合は、GitHub側でアクセスするリポジトリから除外した。

{{ card(title="Remove access | GitHub integration", url="https://developers.cloudflare.com/workers/ci-cd/builds/git-integration/github-integration/#remove-access") }}

## GitHub Actionsで `cloudflare/wrangler-action` を使ってデプロイする

`cloudflare/pages-action` が `v1.5.0` と廃止となり、 `cloudflare/wrangler-action` へ移行を推奨していた。

{{ card(title="[DEPRECATED] Cloudflare Pages GitHub Action | GitHub", url="https://github.com/cloudflare/pages-action?tab=readme-ov-file#deprecated-cloudflare-pages-github-action") }}

`cloudflare/wrangler-action` のREADMEに例が書かれていたので参考にした。

{{ card(title="Deploy your Pages site (production & preview) | GitHub", url="https://github.com/cloudflare/wrangler-action?tab=readme-ov-file#deploy-your-pages-site-production--preview") }}

設定した `.github/workflows/deploy.yaml` はこのようになった。

```yaml
on:
  push:
    branches:
      - "main"

jobs:
  deploy:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      deployments: write
    name: Deploy to Cloudflare Pages from GitHub Actions
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Setup Zola
        uses: taiki-e/install-action@v2
        with:
          tool: zola@0.20.0
      - name: Setup cargo-make
        uses: actions-rs/cargo@v1
        with:
          command: install
          args: --debug cargo-make
      - name: Build
        uses: actions-rs/cargo@v1
        with:
          command: make
          args: build_gha
      - name: Deploy
        uses: cloudflare/wrangler-action@v3
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          command: pages deploy dist --project-name=dnfolio
          gitHubToken: ${{ secrets.GITHUB_TOKEN }}
```

個人サイトでは、開発サーバーの起動やコードのフォーマット、ビルドなどを [cargo-make](https://github.com/sagiegurari/cargo-make) で実施している。
この環境でデプロイしたかったのもGit統合から移行した理由の一つ。

## `deploy.yaml` の説明

それぞれのステップについて簡単に書く。

### Setup Zola

Zola公式ドキュメントに書いてあるのをそのままコピペした。

{{ card(title="Via Github Actions | Installation", url="https://www.getzola.org/documentation/getting-started/installation/#via-github-actions") }}

```yaml
- name: Setup Zola
  uses: taiki-e/install-action@v2
  with:
    tool: zola@0.20.0
```

`taiki-e/install-action` はZola以外にも対応してそうだったので気になった。

### Setup cargo-make

`cargo-make` のREADMEを参考にした。

{{ card(title="Github Actions | cargo-make GitHub", url="https://github.com/sagiegurari/cargo-make?tab=readme-ov-file#usage-ci-github-actions") }}

```yaml
- name: Setup cargo-make
  uses: actions-rs/cargo@v1
  with:
    command: install
    args: --debug cargo-make
```

### Build

`cargo-make` の書き方を参考にした。

```yaml
- name: Build
  uses: actions-rs/cargo@v1
  with:
    command: make
    args: build_gha
```

このように書くことで `cargo make build_gha` を実行することになる。ちなみにこの `build_gha` は、プロジェクトルートにある `Makefile.toml` で設定している。

```toml
[tasks.build_gha]
command = "zola"
args = ["build"]
```

今はまだ `zola build` のみだが、今後複数のビルドを実行したいときに拡張しやすい。

### Deploy

`wrangler-action` のREADMEを参考にした。

{{ card(title="Deploy your Pages site (production & preview) | GitHub", url="https://github.com/cloudflare/wrangler-action?tab=readme-ov-file#deploy-your-pages-site-production--preview") }}

```yaml
- name: Deploy
  uses: cloudflare/wrangler-action@v3
  with:
    apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
    accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
    command: pages deploy dist --project-name=dnfolio
    gitHubToken: ${{ secrets.GITHUB_TOKEN }}
```

`dist` は、 `zola build` で出力するディレクトリ名にする。私の場合は、 `config.toml` で `dist` に設定しているが、デフォルトは `public` なので適宜変更する。

```toml
# デフォルト
output_dir = "public"

# 私の設定
output_dir = "dist"
```

{{ card(title="Configuration | Zola", url="https://www.getzola.org/documentation/getting-started/configuration/") }}

`secrets.CLOUDFLARE_ACCOUNT_ID` と `secrets.CLOUDFLARE_API_TOKEN` はGitHub Actionsを構築するリポジトリの設定から事前に準備する必要がある。

1. GitHubのプロジェクトのリポジトリにアクセスする。
2. リポジトリ名の下にある **Settings** を選択する。
3. **Secrets and variables** > **Actions** > **Secrets** タブ内の **New repository secret** を選択する。
4. 名前に `CLOUDFLARE_ACCOUNT_ID` 、値には自分の **CloudflareアカウントID** を指定する。 [Get project account ID](https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/#get-project-account-id) を参考にする。 **Add secret** で作成する。
5. 手順4と同じように、名前に`CLOUDFLARE_API_TOKEN` 、値に **Cloudflare APIトークン** を指定する。 [Generate an API Token](https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/#generate-an-api-token) を参考にする。 **Add secret** で作成する。

詳細はCloudflare Pagesのドキュメントに書かれている。

{{ card(title="Add Cloudflare credentials to GitHub secrets | Cloudflare Docs", url="https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/#add-cloudflare-credentials-to-github-secrets") }}

`GITHUB_TOKEN` はGitHubが自動で取得する。

## おわりに

Git統合によるデプロイはとても簡単だった。
これまでCloudflare PagesでSvelteKitやNext.jsのデプロイを経験したがどれも簡単にデプロイ出来て気に入っていた。
しかし、デプロイ時にビルド以外の処理もしたいと思ったため今回移行することにした。
`wrangler-action` は、今後使っていこうと思っていたCloudflare Workersでも利用出来そう。キャッチアップ出来て良かった。
仕事で利用しているFirebaseやGoogle Cloudのドキュメントに比べると、個人的にCloudflare周辺のドキュメントは分かりやすい。

次はCloudflare Workersを触っていきたい！
