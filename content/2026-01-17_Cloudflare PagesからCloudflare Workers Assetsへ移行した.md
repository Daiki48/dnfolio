+++
title = "Cloudflare PagesからCloudflare Workers Assetsへ移行した"
slug = "migrate-cloudflare-pages-to-workers-assets"
description = "個人サイトをCloudflare Pagesで公開していたが、Workers Assetsへ移行した。"
created = "2026-01-17"
draft = false
[taxonomies]
tags = ["Cloudflare"]
languages = ["ja"]
+++

## はじめに

この個人サイトは、これまでCloudflare Pagesでホスティングしていた。GitHub Actionsでビルドして、`wrangler pages deploy`でデプロイする構成だ。

特に不満はなかった。ビルドは速いし、デプロイも安定している。無料だし。

でも、Cloudflareの公式ブログを読んでいたら、気になる発表を見つけてしまった。

## Cloudflare公式の発表

2025年4月、Cloudflareは「Your frontend, backend, and database — now in one Cloudflare Worker」というブログ記事を公開した。

[Your frontend, backend, and database — now in one Cloudflare Worker | Cloudflare Blog](https://blog.cloudflare.com/full-stack-development-on-cloudflare-workers/)

日本語に翻訳された記事はこれ。

[フロントエンド、バックエンド、データベースのすべてが1つのCloudflare Workerで実現 | Cloudflare Blog](https://blog.cloudflare.com/ja-jp/full-stack-development-on-cloudflare-workers/)

この記事で、Workers Assetsが正式リリース（GA）となり、静的アセットのホスティングがWorkersで可能になったことが発表された。

そして、こんな一文があった。

> Now that Workers supports both serving static assets and server-side rendering, you should start with Workers. Cloudflare Pages will continue to be supported, but, going forward, all of our investment, optimizations, and feature work will be dedicated to improving Workers.

「今後はWorkersから始めるべき。Pagesは引き続きサポートするが、**今後の投資・最適化・機能開発はすべてWorkersに集中する**」と。

つまり、Pagesは廃止されないが、積極的な機能追加や改善はWorkersに集中するということだ。

## 移行を決めた理由

正直、Pagesのままでも当面は問題ない。Cloudflareは「Pagesは引き続きサポートする」と明言している。

でも、今後の機能開発がWorkersに集中するなら、早めに移行しておいた方が良いだろう。新しい機能や最適化の恩恵を受けられるし、何より新しいものを試したいという好奇心もある。エンジニアの性だ。

## 移行手順

### 1. wrangler.tomlの作成

Pagesでは設定ファイルなしでCLIオプションだけでデプロイできた。Workers Assetsでは`wrangler.toml`が必要になる。

プロジェクトルートに以下の内容で作成した。

```toml
name = "your-project-name"
compatibility_date = "2026-01-15"

[assets]
directory = "./dist"
html_handling = "auto-trailing-slash"
not_found_handling = "404-page"
```

設定項目の説明：

- `name`: Cloudflareダッシュボードに表示されるWorker名
- `compatibility_date`: Workers APIの互換性日付
- `directory`: 静的ファイルの出力ディレクトリ
- `html_handling`: `/posts/foo/`へのアクセスで`index.html`を自動配信
- `not_found_handling`: 存在しないパスで`404.html`を返す

### 2. GitHub Actionsの修正

デプロイコマンドを変更する。

**変更前**:
```yaml
- name: Deploy
  uses: cloudflare/wrangler-action@v3
  with:
    apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
    accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
    command: pages deploy dist --project-name=your-project --commit-dirty=true --branch=main
    gitHubToken: ${{ secrets.GITHUB_TOKEN }}
```

**変更後**:
```yaml
- name: Deploy
  uses: cloudflare/wrangler-action@v3
  with:
    apiToken: ${{ secrets.YOUR_WORKERS_API_TOKEN }}
    accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
    command: deploy
```

シンプルになった。`wrangler.toml`があるので、オプションは不要だ。

`gitHubToken`も削除した。これはPagesのGitHub Deployments連携用で、Workers Assetsでは使わない。

### 3. 404ページの追加

`not_found_handling = "404-page"`を設定したので、`dist/404.html`が必要になる。

私の場合はRust製SSGなので、ビルドスクリプトに404ページ生成を追加した。

```rust
// 404ページを生成
let not_found_html = base::layout(
    PageConfig {
        page_title: "ページが見つかりません - yoursite",
        canonical_url: "https://example.com/404",
        metadata: None,
        ogp_image_path: Some("/ogp/default.png"),
        structured_data_html: None,
    },
    sidebar_markup,
    not_found_content,
    sidebar_right_markup,
)
.into_string();

fs::write(dist_dir.join("404.html"), not_found_html)?;
```

一般的なSSGを使っている場合は、`dist/404.html`を手動で作成するか、SSGの機能を使って生成すればいい。

### 4. APIトークンの権限問題

ここでハマった。

最初のデプロイで、こんなエラーが出た。

```
✘ [ERROR] A request to the Cloudflare API (/accounts/***/workers/services/your-project) failed.
  Authentication error [code: 10000]
```

原因は、**Pages用に作成したAPIトークンにはWorkers権限がなかった**ことだ。

Cloudflare DashboardでAPIトークンを確認すると、「Cloudflare Pages: Edit」しか権限がなかった。Workers Assetsにデプロイするには、追加の権限が必要だ。

#### 解決策: Workers用の新しいAPIトークンを作成

Cloudflare Dashboard → Profile → API Tokens → Create Token → **Create Custom Token**

| カテゴリ | リソース | 権限 |
|---------|---------|------|
| Account | Workers Scripts | Edit |
| Account | Account Settings | Read |
| Zone | Zone | Read |
| Zone | Workers Routes | Edit |

さらに、Account ResourcesとZone Resourcesで対象を限定する。

- Account: 特定のアカウントのみ
- Zone: 特定のゾーン（ドメイン）のみ

これで最小権限のトークンが作れる。公開リポジトリなので、万が一漏洩しても被害を最小限に抑えたい。

作成したトークンをGitHub Secretsに登録して、`deploy.yaml`で参照するように変更した。

### 5. ローカルでの動作確認

デプロイ前にローカルで確認する。

```bash
# ビルド
cargo run -- build  # または npm run build など

# Workers Assetsでローカル配信
npx wrangler dev
```

`http://localhost:8787`でサイトが表示されればOK。存在しないURLにアクセスして404ページが表示されることも確認する。

### 6. デプロイとドメイン移行

#### workers.devでの確認

まず、カスタムドメインなしでデプロイする。

```bash
git add .
git commit -m "Migrate from Cloudflare Pages to Workers Assets"
git tag v1.0.0
git push origin main
git push origin v1.0.0
```

GitHub Actionsが実行され、`your-project.<account>.workers.dev`にデプロイされる。このURLで動作確認。

#### カスタムドメインの移行

ここが少しドキドキするポイントだ。

Cloudflareでは、**同じドメインをPagesとWorkers両方に設定することはできない**。設定しようとすると「domain already in use」エラーが出る。

つまり、ドメイン移行は以下の順序で行う必要がある。

1. Pagesプロジェクトからカスタムドメインを**削除**
2. **即座に**Workersにカスタムドメインを**追加**

この間がダウンタイムになる。私の場合は数秒で完了したが、手動操作なので多少のラグは覚悟しよう。

**Cloudflare Dashboard**での操作：

1. Workers & Pages → Pages → プロジェクト → Custom domains → ドメインを削除
2. Workers & Pages → Workers → プロジェクト → Settings → Domains & Routes → Add → Custom Domain

### 7. 旧Pagesプロジェクトの扱い

ドメイン移行後、旧Pagesプロジェクトはどうなるか。

- `*.pages.dev`ドメインからは引き続きアクセス可能
- 本番ドメインからは配信されない
- 新しい記事をデプロイしてもPagesは更新されない（古いまま）

放置しても実害はないが、古いコンテンツが公開され続けるのは気持ち悪い。私はPagesプロジェクトを削除した。

削除手順：Workers & Pages → Pages → プロジェクト → Settings → General → Delete project

## まとめ

Cloudflare PagesからWorkers Assetsへの移行は、思ったより簡単だった。

1. `wrangler.toml`を作成
2. `deploy.yaml`のコマンドを`pages deploy`から`deploy`に変更
3. 404ページを追加
4. Workers用のAPIトークンを作成
5. ドメインを移行

ハマりポイントはAPIトークンの権限くらい。これも一度設定すれば終わりだ。

Cloudflareが「今後はWorkersに集中する」と言っている以上、早めに移行しておいて損はない。

それに、新しいプラットフォームを試すのは楽しい。エンジニアはこうでなくちゃ。

## 参考リンク

- [Your frontend, backend, and database — now in one Cloudflare Worker | Cloudflare Blog](https://blog.cloudflare.com/full-stack-development-on-cloudflare-workers/)
- [フロントエンド、バックエンド、データベースのすべてが1つのCloudflare Workerで実現 | Cloudflare Blog](https://blog.cloudflare.com/ja-jp/full-stack-development-on-cloudflare-workers/)
- [Migrate from Pages to Workers | Cloudflare Docs](https://developers.cloudflare.com/workers/static-assets/migration-guides/migrate-from-pages/)
- [Static Assets | Cloudflare Workers Docs](https://developers.cloudflare.com/workers/static-assets/)
