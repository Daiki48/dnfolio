+++
title = "WeztermからAlacritty+Zellijへ移行した話"
slug = "migrate-wezterm-to-alacritty-zellij"
description = "長年お世話になったWeztermから、Alacritty + Zellijの組み合わせに移行した。パフォーマンスを求めて旅立った開発者の記録。"
draft = false
[taxonomies]
tags = ["Alacritty", "Zellij", "Wezterm", "Terminal", "Linux Mint"]
languages = ["ja"]
+++

## 環境

本記事の内容は **Linux Mint 22.3 "Zena"** 環境で検証している。

Linux Mint 22.3は2026年1月13日にリリースされたばかりの最新版だ。Ubuntu 24.04.3 LTSをベースに、カーネル6.14を搭載。Cinnamon 6.6デスクトップ環境が採用され、2029年までの長期サポートが約束されている。

新しいOSで新しいターミナル環境。なんとも気持ちが良い。

## はじめに

2025年から2026年にかけて、ターミナル環境での開発が熱い。

Claude CodeやOpenAI Codexといった、ターミナル上で動作するAIコーディングアシスタントの登場により、ターミナルは単なるコマンド実行の場から「開発の中心地」へと進化しつつある。私自身、Claude CodeをNeovim内のターミナルモードで日常的に活用している。

そんな時代だからこそ、ターミナル環境にはこだわりたい。

ある日、私はターミナルを眺めながら思った。

「...重くない？」

Weztermを愛用して数年。タブを開き、ペインを分割し、Neovimでコードを書く日々。何不自由ない生活だった。しかし、開発者という生き物は常に「もっと速く」「もっと軽く」を求めてしまう悲しい性を持っている。

そんなわけで、Alacritty + Zellijという新天地へ**メイン環境を移行**することにした。

## 登場人物紹介

今回の移行劇に登場する素晴らしいツールたちを紹介しよう。どれもRust製という共通点を持つ、いわば「Rustファミリー」である。

### Wezterm - 多機能なオールインワン

[Wezterm](https://wezterm.org/) は [@wez](https://github.com/wez) 氏によって開発された、Rust製のクロスプラットフォームターミナルエミュレータ兼マルチプレクサだ。

**特徴:**
- Linux、macOS、Windows、FreeBSD、NetBSDに対応
- ターミナルペイン、タブ、ウィンドウのマルチプレクサ機能を内蔵
- リガチャ、カラー絵文字、フォントフォールバック対応
- Luaによる柔軟な設定

一言で表すなら「全部入り弁当」。これ一つで何でもできる。私も長らくお世話になった。ありがとう、Wezterm。君のことは忘れない。

### Alacritty - 速さこそ正義

[Alacritty](https://alacritty.org/) は「sensible defaults」を掲げる、OpenGLベースのモダンターミナルエミュレータだ。

**特徴:**
- BSD、Linux、macOS、Windowsに対応
- GPUアクセラレーションによる高速レンダリング
- Vi Mode（viバインディングでの移動・選択）
- スクロールバック内テキスト検索
- Multi-Window（単一プロセスでリソース効率化）

一言で表すなら「シンプル・イズ・ベスト」。余計な機能は持たない。ターミナルエミュレータとしての仕事を、ただ高速にこなす。その潔さが美しい。

### Zellij - ワークスペースの魔術師

[Zellij](https://zellij.dev/) は「A terminal workspace with batteries included」を掲げる、Rust製のターミナルマルチプレクサだ。

**特徴:**
- Linux、macOSに対応
- セッション永続化（PCを再起動しても復帰可能）
- タブとペインによる柔軟なレイアウト
- プラグインシステム
- 直感的なUI（初心者にも優しい）

一言で表すなら「tmuxの正統進化系」。tmuxを使ったことがある人なら、Zellijの快適さに感動するだろう。使ったことがない人でも、Zellijから始めれば幸せになれる。

### Ghosttyという選択肢

実は、もう一つ気になっていたターミナルエミュレータがある。[Ghostty](https://ghostty.org/)だ。

HashiCorpの共同創業者であるMitchell Hashimoto氏が開発した、GPUアクセラレーション対応のモダンなターミナルエミュレータ。Zigで書かれており、macOSではMetal、LinuxではOpenGLを使用した高速レンダリングが特徴だ。Kittyグラフィックスプロトコルや、ネイティブUIコンポーネントの採用など、技術的にも非常に興味深い。

正直、かなり迷った。

しかし、今回はAlacrittyを選んだ。理由は**Rust製であること**。私の開発環境はRustで統一する方向で進めており、Neovimの設定もRust（nvim-oxi）で書いている。ターミナルエミュレータもRust製で揃えたかった。完全に趣味の問題だ。

Ghosttyは気が向いたら触ってみたい。きっと素晴らしい体験が待っているだろう。

## なぜ移行したのか

理由はシンプル。**パフォーマンス**だ。

Weztermは素晴らしいツールだが、オールインワンであるがゆえにリソース消費がやや多い。日常的な開発では気にならないが、重いビルドを走らせながらNeovimで編集し、Dockerコンテナを複数立ち上げていると...ふと思うのだ。

「ターミナルエミュレータって、文字を表示するだけでいいのでは？」

そう、Alacrittyはまさにその哲学を体現している。文字を表示する。それだけ。マルチプレクサ機能？それはZellijに任せればいい。[Unix哲学](https://en.wikipedia.org/wiki/Unix_philosophy)の「一つのことをうまくやる（Do One Thing and Do It Well）」を地で行く構成だ。この原則は1978年にDoug McIlroy氏によって提唱された。

## 新環境の構成

最終的に落ち着いた構成はこちら。

```
Alacritty（ターミナルエミュレータ）
    └── Zellij（マルチプレクサ）
          ├── Tab 1: Neovim
          └── Tab 2: ペイン分割
                ├── Docker開発環境
                └── cargo clippy / git
```

Alacrittyを起動すると自動的にZellijが立ち上がり、セッションに接続する。シャットダウンしてもセッションは保持され、翌日また同じ状態から作業を再開できる。

## Zellijのセッション管理が素晴らしい

Zellijの最大の魅力は**セッション管理**だ。

```
┌─────────────────────────────────────────────────┐
│ Zellijセッション一覧                             │
├─────────────────────────────────────────────────┤
│  web-app     [2 tabs] ── Webアプリ開発          │
│  desktop-app [2 tabs] ── デスクトップアプリ開発  │
│  dotfiles    [1 tab]  ── 設定ファイル管理       │
└─────────────────────────────────────────────────┘
```

プロジェクトごとにセッションを作成しておけば、`Ctrl+g → o → w` でセッションマネージャーを開いて瞬時に切り替えられる。

「今日はWebアプリの開発をしよう」→ web-appセッションを選択
「あ、dotfilesの設定も直さなきゃ」→ dotfilesセッションに切り替え

この体験は、Weztermでは得られなかったものだ。

## キーバインド設定

Weztermで慣れ親しんだキーバインドをZellijでも再現した。

| 操作 | キー |
|------|------|
| タブ切り替え | `Shift+Tab` |
| 新規タブ | `Ctrl+Shift+T` |
| Zellijモード | `Ctrl+g` |
| セッションマネージャー | `Ctrl+g → o → w` |

HHKBユーザーとして、Altキーを多用するバインドは避けた。ホームポジションから遠いのだ。

## パフォーマンスの違い

体感での比較になるが、明らかに軽くなった。

- **起動速度**: Alacrittyは一瞬で起動する
- **スクロール**: 大量のログ出力でもヌルヌル
- **メモリ使用量**: Zellijのセッションを複数立ち上げても安定

特に `cargo build` の出力が大量に流れる場面で違いを感じる。Alacrittyはひたすら文字を描画し続け、一切もたつかない。GPUアクセラレーションの恩恵だろう。

## 移行時のハマりポイント

### 1. Zellijの設定ファイル構文

Zellijの設定は `.kdl` 形式で書く。最初、`Escape` キーを `"Escape"` と書いたらエラーになった。正解は `"Esc"` だ。

```kdl
// NG
bind "Escape" { SwitchToMode "locked"; }

// OK
bind "Esc" { SwitchToMode "locked"; }
```

### 2. IMEのカーソル追従

Fcitx5（SKK）を使っている環境で、Neovim内ではIMEの候補ウィンドウがカーソルに追従しない問題がある。これはAlacritty固有ではなく、TUIアプリケーション全般の制限だ。ターミナルのプロンプトでは正常に動作するので、Neovim特有の問題と言える。どうしても気になる場合は、skkeleton等のNeovimプラグインを検討するのも手だ。

### 3. Alacrittyの透過設定

Weztermでは `window_background_opacity = 0.9` だったが、Alacrittyでは `[window]` セクションに `opacity = 0.85` と書く。TOMLの構造が若干異なる。

## おわりに

WeztermからAlacritty + Zellijへの移行は、予想以上にスムーズだった。

とはいえ、これはWeztermとの永遠の別れではない。Weztermは今後も普通に使うと思う。ちょっとした作業や、別のマシンでの開発では引き続き活躍してもらうつもりだ。今回はあくまで「メイン環境」を移行したという話。

Weztermは素晴らしいツールだ。これ一つで完結する手軽さは、特に初心者や設定をシンプルに保ちたい人には最適だろう。

一方で、パフォーマンスを追求し、Unix哲学的な「分離された責務」を好む人には、Alacritty + Zellijの組み合わせをおすすめする。

どちらも正解だ。大事なのは、自分の開発スタイルに合ったツールを選ぶこと。

---

開発環境の移行は、正直なところ面倒だ。設定ファイルを書き、動作確認をし、細かい調整を繰り返す。何度「前の環境に戻そうかな...」と思ったことか。

しかし、移行が完了した後の達成感は格別だ。新しい環境でターミナルを開くと、なんだか新鮮な気持ちになる。同じコードを書いているはずなのに、ちょっとだけワクワクする。

この気持ちを味わうために、私たちは設定をいじり続けるのかもしれない。

...さて、次は何を移行しようか。

## 参考リンク

- [Linux Mint 22.3 Release Notes](https://linuxmint.com/rel_zena.php)
- [Wezterm公式](https://wezterm.org/)
- [Alacritty公式](https://alacritty.org/)
- [Zellij公式](https://zellij.dev/)
- [Zellij Documentation](https://zellij.dev/documentation/)
- [Ghostty公式](https://ghostty.org/)
