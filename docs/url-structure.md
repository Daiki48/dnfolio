# dnfolio URL構造仕様

## 概要

このドキュメントはdnfolioのURL設計に関する仕様です。

**設計方針**:
- SEOフレンドリーなURL構造
- 英語スラッグの使用（日本語URLを避ける）
- ハイフン区切り（アンダースコアは使用しない）
- クリーンURL（末尾スラッシュ、.html拡張子なし）

---

## URL構造

### 一覧

| ページ種別 | URL形式 | 例 |
|-----------|---------|-----|
| トップページ | `/` | `https://dnfolio.me/` |
| 記事ページ | `/posts/{slug}/` | `https://dnfolio.me/posts/git-command-memo/` |
| タグページ | `/tags/{tag-slug}/` | `https://dnfolio.me/posts/rust/` |
| プライバシーポリシー | `/privacy/` | `https://dnfolio.me/privacy/` |
| サイトマップ | `/sitemap.xml` | `https://dnfolio.me/sitemap.xml` |
| RSSフィード | `/feed.xml` | `https://dnfolio.me/feed.xml` |
| OGP画像 | `/ogp/{slug}.png` | `https://dnfolio.me/ogp/git-command-memo.png` |

---

## スラッグ命名規則

### 基本ルール

1. **英語のみ使用**（日本語は禁止）
2. **小文字のみ**
3. **ハイフン区切り**（スペース、アンダースコアは使用しない）
4. **簡潔で意味のある名前**

### 良い例

| 記事タイトル | スラッグ |
|-------------|---------|
| Gitコマンド備忘録 | `git-command-memo` |
| OGP画像を表示出来るようになった | `ogp-image-display` |
| WSL2環境構築(Zsh, Neovim) | `wsl2-setup-zsh-neovim` |
| Neovim v0.11がリリースしたのでLSP周辺の設定を見直した | `neovim-v0-11-lsp-config` |

### 悪い例（避けるべき）

| 避けるべきスラッグ | 理由 |
|-------------------|------|
| `Gitコマンド備忘録` | 日本語を含む |
| `git_command_memo` | アンダースコア使用 |
| `GIT-COMMAND-MEMO` | 大文字使用 |
| `git-command-memo.html` | 拡張子を含む |
| `2025-09-29_git-command` | アンダースコア使用 |

---

## 記事メタデータでのslug指定

### フロントマター形式

```toml
+++
title = "Gitコマンド備忘録"
slug = "git-command-memo"
description = "よく使うGitコマンドのメモ"
created = "2025-09-29"
updated = "2025-09-29"
[taxonomies]
tags = ["Git"]
languages = ["ja"]
+++
```

### slugフィールドの優先度

1. `slug`フィールドが指定されている場合 → その値を使用
2. `slug`フィールドがない場合 → ファイル名から自動生成（日付部分を除去してslugify）

---

## 出力ディレクトリ構造

### ビルド後の構造

```
dist/
├── index.html                      # トップページ
├── privacy/
│   └── index.html                  # プライバシーポリシー
├── posts/
│   ├── git-command-memo/
│   │   └── index.html              # 記事ページ
│   ├── ogp-image-display/
│   │   └── index.html
│   └── ...
├── tags/
│   ├── rust/
│   │   └── index.html              # タグページ
│   ├── neovim/
│   │   └── index.html
│   └── ...
├── ogp/
│   ├── git-command-memo.png        # OGP画像
│   ├── ogp-image-display.png
│   └── ...
├── sitemap.xml
├── feed.xml
├── robots.txt
└── ...
```

---

## 旧URLからのリダイレクト

### リダイレクトマッピング

旧URL構造から新URL構造への301リダイレクトを設定する必要があります。

```
# 旧URL → 新URL
/content/2025-09-29_Gitコマンド備忘録.html → /posts/git-command-memo/
/content/2025-10-03_OGP画像を表示出来るようになった.html → /posts/ogp-image-display/
```

### Cloudflare Pages _redirectsファイル

```
# _redirects
/content/2025-09-29_Git%E3%82%B3%E3%83%9E%E3%83%B3%E3%83%89%E5%82%99%E5%BF%98%E9%8C%B2.html /posts/git-command-memo/ 301
/content/2025-10-03_OGP%E7%94%BB%E5%83%8F%E3%82%92%E8%A1%A8%E7%A4%BA%E5%87%BA%E6%9D%A5%E3%82%8B%E3%82%88%E3%81%86%E3%81%AB%E3%81%AA%E3%81%A3%E3%81%9F.html /posts/ogp-image-display/ 301
# ... 他の記事も同様
```

---

## タグスラッグ

### 変換ルール

| タグ名 | スラッグ |
|--------|---------|
| Rust | `rust` |
| Neovim | `neovim` |
| GitHub Actions | `github-actions` |
| WSL2 | `wsl2` |
| coc.nvim | `coc-nvim` |
| Node.js | `node-js` |

### 実装

`slug`クレートの`slugify`関数を使用：

```rust
use slug::slugify;

let tag = "GitHub Actions";
let slug = slugify(tag); // "github-actions"
```

---

## URL生成ロジック（Rust実装）

### 記事URL生成

```rust
fn generate_article_url(metadata: &MetaData, file_stem: &str) -> String {
    let slug = metadata
        .slug
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            // ファイル名から日付部分を除去してslugify
            let name = file_stem
                .split('_')
                .skip(1)
                .collect::<Vec<_>>()
                .join("-");
            slugify(&name)
        });

    format!("/posts/{}/", slug)
}
```

### タグURL生成

```rust
fn generate_tag_url(tag_name: &str) -> String {
    format!("/tags/{}/", slugify(tag_name))
}
```

---

## SEO上の重要ポイント

### なぜ英語スラッグなのか

1. **URLエンコード問題の回避**
   - 日本語URLはブラウザで見た目は良いが、実際には長いエンコード文字列になる
   - サイトマップやリンク共有時に問題が発生しやすい

2. **Googleの推奨**
   - Googleは読みやすい、意味のあるURLを推奨
   - ハイフン区切りが単語として認識される

3. **リンク共有のしやすさ**
   - SNSやメールでURLを共有する際に文字化けしない

### なぜ末尾スラッシュなのか

1. **ディレクトリとして扱われる**
   - `/posts/slug/` は `/posts/slug/index.html` として解決される
   - Cloudflare Pagesでの静的ファイル配信に適している

2. **一貫性**
   - すべてのページURLが同じパターンになる
   - canonical URLの正規化が容易

---

## 既存記事のslugマッピング一覧

以下は既存記事に対するslug割り当ての例です：

| ファイル名 | 推奨slug |
|-----------|----------|
| `2025-09-29_Gitコマンド備忘録.md` | `git-command-memo` |
| `2025-10-03_OGP画像を表示出来るようになった.md` | `ogp-image-display` |
| `2025-09-25_Ubuntuリポジトリのミラーサーバーを変更した.md` | `ubuntu-mirror-server-change` |
| `2025-08-24_GitHub OrganizationでPush出来なかったため、GitHub CLIを導入した.md` | `github-org-push-github-cli` |
| `2025-05-18_WSL2をアップデート.md` | `wsl2-update` |
| `2025-05-08_WSL2でNode.jsプロジェクトをビルド時にJavaScript heap out of memoryが発生した場合.md` | `wsl2-nodejs-heap-out-of-memory` |
| `2025-05-06_WSL2のUbuntuにZigをインストールする.md` | `wsl2-ubuntu-zig-install` |
| `2025-03-31_Zolaで構築した個人サイトに検索機能(Pagefind)を実装した.md` | `zola-pagefind-search` |
| `2025-03-30_Cloudflare Pagesへのデプロイ手段をGit統合からGitHub Actionsに変更した.md` | `cloudflare-pages-github-actions` |
| `2025-03-28_Neovim v0.11がリリースしたのでLSP周辺の設定を見直した.md` | `neovim-v0-11-lsp-config` |
| `2025-03-26_Created sample project powerd by Hono + Cloudflare Workers + Bun.md` | `hono-cloudflare-workers-bun` |
| `2025-03-20_dnfolioのUIを若干変更した.md` | `dnfolio-ui-update` |
| `2025-02-24_WSL2環境構築(Zsh, Neovim).md` | `wsl2-setup-zsh-neovim` |
| `2025-02-19_coc.nvimからnvim-lspconfigへの移行.md` | `coc-nvim-to-nvim-lspconfig` |
| `2025-02-07_developing a CLI tool called thumbgen.md` | `thumbgen-cli-development` |
| `2025-01-31_NeovimでGoの環境構築.md` | `neovim-go-setup` |
| `2025-01-21_個人的ノート管理2025年1月.md` | `personal-note-management-2025-01` |
| `2025-01-20_試験的にリーディングタイムを表示してみた.md` | `reading-time-display` |
| `2025-01-18_LAPRASに登録してみた.md` | `lapras-registration` |
| `2025-01-17_workers-rsを試してみた.md` | `workers-rs-trial` |
| `2025-01-15_changed-tld.md` | `changed-tld` |
| `2025-01-14_GitHubのREADMEをシンプルにした.md` | `github-readme-simplify` |
| `2025-01-13_個人サイトの改修記録その2.md` | `dnfolio-update-2` |
| `2025-01-12_個人サイト開発にcargo-makeを導入した.md` | `cargo-make-introduction` |
| `2025-01-10_htmldjangoファイルのフォーマットに成功した.md` | `htmldjango-format-success` |
| `2025-01-09_Xへの共有ボタンを記事に追加した.md` | `x-share-button` |
| `2025-01-09_created-share-button-for-bluesky.md` | `bluesky-share-button` |
| `2025-01-09_個人サイトのdnfolioを大きく改修した.md` | `dnfolio-major-update` |
| `2025-01-09_changed-bluesky-handle-to-my-domain.md` | `bluesky-handle-custom-domain` |
| `2025-01-09_bun upgradeに失敗した.md` | `bun-upgrade-failed` |
| `2025-01-08_Zolaのshortcodesを使ってみた.md` | `zola-shortcodes` |
| `2025-01-08_コードフォーマッターにdprintを使ってみた.md` | `dprint-formatter` |
| `2025-01-08_updated-templates-for-axum-and-tera.md` | `axum-tera-templates-update` |
| `2025-01-07_n回目の個人サイト引越しを検討中.md` | `site-migration-consideration` |
| `2025-01-07_dnfolioをVitePressからZolaへ移行中.md` | `vitepress-to-zola-migration` |

---

## 参考

- [Google URL構造ガイドライン](https://developers.google.com/search/docs/crawling-indexing/url-structure)
- [Cloudflare Pages リダイレクト](https://developers.cloudflare.com/pages/platform/redirects/)
