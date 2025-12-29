# dnfolio SEO実装ガイド

## 概要

このドキュメントはdnfolioのSEO対策実装に関する技術ガイドです。

**目的**:
- Google Search Consoleでのインデックス登録成功
- 検索エンジンからの適切なクロールとインデックス
- リッチリザルト表示の実現

---

## ドメイン構成

| ドメイン | 用途 | 備考 |
|----------|------|------|
| `dnfolio.me` | メインサイト（ポートフォリオ + ブログ） | 正規ドメイン |
| `www.dnfolio.me` | リダイレクト | 301 → dnfolio.me |
| `dnfaira.dnfolio.me` | アプリ紹介サイト | 別プロジェクト |

---

## 実装済みSEO要素

### 基本メタタグ

```html
<html lang="ja">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{ページタイトル}</title>
  <meta name="description" content="{説明文}">
  <meta name="keywords" content="{キーワード}">
  <meta name="author" content="Daiki Nakashima">
  <meta name="robots" content="index, follow">
  <link rel="canonical" href="{正規URL}">
</head>
```

### OGP（Open Graph Protocol）

```html
<meta property="og:title" content="{タイトル}">
<meta property="og:description" content="{説明文}">
<meta property="og:type" content="website">
<meta property="og:url" content="{ページURL}">
<meta property="og:site_name" content="dnfolio">
<meta property="og:image" content="{OGP画像URL}">
<meta property="og:image:width" content="1200">
<meta property="og:image:height" content="630">
<meta property="og:image:type" content="image/png">
```

### Twitter Card

```html
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="{タイトル}">
<meta name="twitter:description" content="{説明文}">
<meta name="twitter:site" content="@dnfolio_me">
<meta name="twitter:image" content="{OGP画像URL}">
```

### RSSフィード

```html
<link rel="alternate" type="application/rss+xml" title="dnfolio RSS Feed" href="/feed.xml">
```

---

## 構造化データ（JSON-LD）

### WebSite（トップページ）

```json
{
  "@context": "https://schema.org",
  "@type": "WebSite",
  "name": "dnfolio",
  "url": "https://dnfolio.me",
  "description": "Daiki Nakashimaの個人サイト",
  "author": {
    "@type": "Person",
    "name": "Daiki Nakashima",
    "url": "https://dnfolio.me"
  },
  "inLanguage": "ja"
}
```

### BlogPosting（各記事）

```json
{
  "@context": "https://schema.org",
  "@type": "BlogPosting",
  "headline": "{記事タイトル}",
  "description": "{記事説明}",
  "datePublished": "{作成日 ISO8601}",
  "dateModified": "{更新日 ISO8601}",
  "author": {
    "@type": "Person",
    "name": "Daiki Nakashima",
    "url": "https://dnfolio.me"
  },
  "publisher": {
    "@type": "Person",
    "name": "Daiki Nakashima"
  },
  "image": "{OGP画像URL}",
  "url": "{記事URL}",
  "mainEntityOfPage": {
    "@type": "WebPage",
    "@id": "{記事URL}"
  },
  "inLanguage": "ja"
}
```

### BreadcrumbList（パンくずリスト）

```json
{
  "@context": "https://schema.org",
  "@type": "BreadcrumbList",
  "itemListElement": [
    {
      "@type": "ListItem",
      "position": 1,
      "name": "ホーム",
      "item": "https://dnfolio.me"
    },
    {
      "@type": "ListItem",
      "position": 2,
      "name": "{記事タイトル}",
      "item": "{記事URL}"
    }
  ]
}
```

---

## サイトマップ

### 仕様

- **ファイル**: `/sitemap.xml`
- **形式**: XML Sitemap Protocol 0.9
- **更新**: ビルド時に自動生成

### 構造

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://dnfolio.me/</loc>
    <lastmod>{更新日時 ISO8601}</lastmod>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://dnfolio.me/posts/{slug}/</loc>
    <lastmod>{更新日時 ISO8601}</lastmod>
    <priority>0.8</priority>
  </url>
  <!-- ... -->
</urlset>
```

### 注意事項

- URLはすべて英語スラッグを使用
- 日本語URLは使用しない（エンコード問題回避）
- `lastmod`は記事のupdated または created から取得

---

## robots.txt

```
User-agent: *
Allow: /
Sitemap: https://dnfolio.me/sitemap.xml
```

---

## OGP画像生成

### 仕様

- **サイズ**: 1200x630px
- **形式**: PNG
- **フォント**: Noto Sans JP (Regular, Bold)
- **出力先**: `/ogp/{slug}.png`

### 生成フロー

1. 記事タイトルからSVGテンプレートを生成
2. resvgでSVG → PNG変換
3. `/dist/ogp/`に出力

---

## 実装ファイル

| ファイル | 役割 |
|----------|------|
| `src/templates/base.rs` | HTMLテンプレート、メタタグ生成 |
| `src/structured_data.rs` | JSON-LD構造化データ生成 |
| `src/sitemap.rs` | サイトマップ生成 |
| `src/feed.rs` | RSSフィード生成 |
| `src/ogp.rs` | OGP画像生成 |

---

## 検証ツール

### Google Search Console
- サイトマップ送信
- インデックス登録リクエスト
- 検索パフォーマンス確認

### リッチリザルトテスト
- URL: https://search.google.com/test/rich-results
- 構造化データの検証

### OGP確認
- Twitter Card Validator: https://cards-dev.twitter.com/validator
- Facebook Sharing Debugger: https://developers.facebook.com/tools/debug/

### PageSpeed Insights
- URL: https://pagespeed.web.dev/
- Core Web Vitals確認

---

## トラブルシューティング

### インデックスされない場合

1. **サイトマップを確認**
   - Google Search Consoleでサイトマップのステータス確認
   - URLが正しくエンコードされているか確認

2. **robots.txtを確認**
   - `Disallow`でブロックしていないか確認
   - サイトマップのURLが正しいか確認

3. **canonical URLを確認**
   - 重複コンテンツがないか確認
   - 正規URLが正しく設定されているか確認

4. **手動でインデックス登録リクエスト**
   - Google Search Console → URL検査 → インデックス登録をリクエスト

### OGP画像が表示されない場合

1. 画像URLが絶対パスになっているか確認
2. 画像サイズが1200x630px以上か確認
3. 画像形式がPNG/JPEGか確認（SVGは非推奨）
4. キャッシュをクリアして再確認

---

## 参考リンク

- [Google検索セントラル](https://developers.google.com/search)
- [Schema.org](https://schema.org/)
- [Open Graph Protocol](https://ogp.me/)
- [Twitter Cards](https://developer.twitter.com/en/docs/twitter-for-websites/cards/overview/abouts-cards)
