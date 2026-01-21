+++
title = "Ghosttyを試してみた結果、Alacrittyに落ち着いた話"
slug = "tried-ghostty-but-settled-on-alacritty"
description = "話題のGhosttyを実際に試してみた。リガチャ対応やテーマ内蔵など魅力的な機能がある一方、最終的にはAlacritty + tmuxの組み合わせに落ち着いた。3つのターミナルエミュレータと2つのマルチプレクサを比較した体験記。"
draft = false
[taxonomies]
tags = ["Ghostty", "Alacritty", "Wezterm", "tmux", "Zellij", "Terminal", "Linux Mint"]
languages = ["ja"]
+++

## 結論

先に結論を書いておく。

**Alacritty + tmux** が私の好みだった。

Ghosttyは素晴らしいターミナルエミュレータだ。リガチャ対応、Tokyo Nightテーマ内蔵、高速なGPUレンダリング。どれも魅力的な機能である。

しかし、実際に使ってみた結果、Alacrittyの軽快な動作とtmuxとの相性の良さに軍配が上がった。

これは優劣の話ではない。好みの話だ。

本記事では、Wezterm、Alacritty、Ghosttyという3つのターミナルエミュレータと、Zellij、tmuxという2つのマルチプレクサを実際に使い比べた体験を記録する。どのツールも素晴らしい。あとは好みで選べばいい。

## 環境

本記事の内容は以下の環境で検証している。

| 項目 | 値 |
|------|-----|
| OS | Linux Mint 22.3 "Zena" |
| ベース | Ubuntu 24.04 LTS |
| カーネル | 6.14 (HWE) |
| GPU | NVIDIA (プロプライエタリドライバ) |
| ディスプレイサーバー | X11 |
| IME | Fcitx5 + SKK |

Linux Mint 22.3は2029年4月までの長期サポートが約束されている。安心して開発環境を構築できる。

## シリーズの経緯

実は、この記事には前日譚がある。

1. [WeztermからAlacritty+Zellijへ移行した話](/posts/migrate-wezterm-to-alacritty-zellij/) — 長年愛用したWeztermからAlacritty + Zellijへ移行
2. [Alacritty+Zellijの描画問題でtmuxへ移行した話](/posts/migrate-alacritty-zellij-to-tmux/) — Zellijの描画問題に直面し、tmuxへ移行

そう、私は2日間で3回も環境を変えている。

前回の記事で「Ghosttyも試したい」と書いた。そして今日、実際に試してみた。これが3部作の完結編...になるかどうかは分からない。また気が変わるかもしれない。ターミナル沼には底がないのだ。

## 登場人物紹介

今回の比較に登場するツールたちを紹介しよう。

### ターミナルエミュレータ

#### Wezterm - 多機能なオールインワン

[Wezterm](https://wezterm.org/) は [@wez](https://github.com/wez) 氏によって開発された、Rust製のターミナルエミュレータだ。

**特徴:**
- Linux、macOS、Windows、FreeBSD、NetBSDに対応
- ターミナルペイン、タブ、ウィンドウのマルチプレクサ機能を内蔵
- リガチャ、カラー絵文字、フォントフォールバック対応
- Luaによる柔軟な設定
- ハイパーリンク対応

一言で表すなら「スイスアーミーナイフ」。ターミナルに求める機能が全て揃っている。私も約3年間、メイン環境として愛用してきた。

#### Alacritty - 速さこそ正義

[Alacritty](https://alacritty.org/) は「sensible defaults」を掲げる、OpenGLベースのモダンターミナルエミュレータだ。

**特徴:**
- BSD、Linux、macOS、Windowsに対応
- GPUアクセラレーションによる高速レンダリング
- Vi Mode（viバインディングでの移動・選択）
- スクロールバック内テキスト検索
- Multi-Window（単一プロセスでリソース効率化）
- TOML形式のシンプルな設定

一言で表すなら「引き算の美学」。必要最低限の機能だけを、最高の品質で提供する。余計なものは何もない。

#### Ghostty - 新世代の刺客

[Ghostty](https://ghostty.org/) はMitchell Hashimoto氏（HashiCorp共同創業者）が開発した、GPUアクセラレーション対応のモダンターミナルエミュレータだ。2024年12月にオープンソース化された。

**特徴:**
- macOS、Linuxに対応（Windowsは開発中）
- Zig製で、macOSはMetal、LinuxはOpenGL/Vulkanを使用
- ネイティブUI（macOSはCocoa、LinuxはGTK4）
- リガチャ対応
- 豊富な組み込みテーマ
- Kittyグラフィックスプロトコル対応

一言で表すなら「モダンの申し子」。2026年現在、周囲のエンジニアがこぞって移行している話題のターミナルだ。

### マルチプレクサ

#### tmux - 枯れた技術の信頼性

[tmux](https://github.com/tmux/tmux/wiki) は2007年から開発が続く、ターミナルマルチプレクサの定番だ。

**特徴:**
- セッション管理とデタッチ/アタッチ
- 高度なカスタマイズ性
- 豊富なプラグインエコシステム（TPM）
- 長年の運用実績に裏打ちされた安定性

#### Zellij - 新世代のマルチプレクサ

[Zellij](https://zellij.dev/) はRust製のターミナルマルチプレクサ。公式が掲げる「A terminal workspace with batteries included」の通り、箱から出してすぐ使える設計だ。

**特徴:**
- Linux、macOSに対応
- セッション永続化
- 画面下部にキーヒントが表示される親切設計
- プラグインシステム
- 浮動ペイン機能

## 3つのターミナルエミュレータ比較

実際に使ってみた上での比較表を作成した。

| 項目 | Alacritty | Wezterm | Ghostty |
|------|-----------|---------|---------|
| **言語** | Rust | Rust | Zig |
| **GPUレンダラー** | OpenGL | OpenGL/WebGPU | OpenGL/Metal/Vulkan |
| **設定形式** | TOML | Lua | 独自（key=value） |
| **マルチプレクサ** | なし | 内蔵 | なし |
| **タブ/ペイン** | なし | あり | ネイティブタブ |
| **リガチャ** | ❌ | ✅ | ✅ |
| **IME対応** | △（一部制限あり） | ✅ | ✅ |
| **テーマ** | カスタム定義 | カスタム定義 | 組み込み多数 |
| **起動速度** | 最速クラス | やや重い | 最速クラス |
| **メモリ使用量** | 少ない | 多め | 少ない |

### パフォーマンスについて

Ghosttyの開発者Mitchell Hashimoto氏によると、GhosttyとAlacrittyの速度差は10%未満（多くの場合5%未満）とのこと。両者とも「最速クラス」のグループに属する。

体感では、どちらも十分に高速だ。大量のログ出力でもヌルヌル動く。

### リガチャについて

リガチャとは、複数の文字を結合して1つのグリフとして表示する機能だ。

```
リガチャなし    リガチャあり
-----------    -----------
  ->             →
  =>             ⇒
  !=             ≠
  <=             ≤
  >=             ≥
```

Alacrittyは開発方針としてリガチャに対応していない。Weztermと Ghosttyは対応している。

正直に言うと、私はリガチャにそこまで魅力を感じなかった。`->` が `→` になっても、`->` のままでも、コードを書く上で大きな差はない。これは完全に好みの問題だ。リガチャが欲しい人にとっては、AlacrittyよりGhosttyやWeztermの方が良い選択肢だろう。

## Zellij vs tmux比較

マルチプレクサの比較も記録しておく。

| 項目 | tmux | Zellij |
|------|------|--------|
| **開発開始** | 2007年〜 | 2021年〜 |
| **言語** | C | Rust |
| **TERM処理** | 独自の`tmux-256color` | 親ターミナルのTERMを継承 |
| **Neovimとの互換性** | 広くテストされている | まだエッジケースあり |
| **CSI 2026対応** | 安定 | 実装に課題あり |
| **学習コスト** | やや高い | 低い（直感的UI） |
| **設定形式** | 独自構文 | KDL |
| **プラグイン** | TPM（豊富） | あり（発展途上） |

### 私がtmuxを選んだ理由

Zellijは画面下部にキーヒントが表示され、初見でも迷わず操作できる。マルチプレクサ入門にはZellijの方がおすすめだ。

しかし、前回の記事で書いた通り、ZellijにはCSI 2026（synchronized renders）の実装に課題があり、Neovim使用時に描画が抜ける問題が発生した。この問題はAlacritty単体では発生しなかったため、Zellijが原因と判断した。

tmuxは枯れた技術だ。枯れているからこそ、エッジケースが潰されている。Neovimとの組み合わせでも、世界中の開発者によって検証済みだ。安定性を求めて、tmuxを選択した。

### 用語の違い

ZellijとtmuxではUIコンポーネントの呼び方が異なる。

| Zellij | tmux | 説明 |
|--------|------|------|
| Session | Session | 複数のタブをまとめる単位 |
| Tab | Window | セッション内の画面切り替え単位 |
| Pane | Pane | 画面を分割した領域 |

## Ghosttyを試す

さて、本題に入ろう。

### インストール（Linux Mint 22.3）

Linux Mint 22.3はUbuntu 24.04ベースのため、公式パッケージは提供されていない。コミュニティが提供する[ghostty-ubuntu](https://github.com/mkasberg/ghostty-ubuntu)を利用する。

**ワンライナーでインストール:**

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/mkasberg/ghostty-ubuntu/HEAD/install.sh)"
```

このスクリプトはLinux MintをUbuntu 24.04（Noble）として認識し、適切な.debパッケージをダウンロード・インストールしてくれる。

実行するとsudoのパスワードを求められる。入力すれば、あっという間にインストール完了だ。

```bash
# バージョン確認
ghostty --version
# ghostty 1.2.3
```

2026年1月時点での最新版は1.2.3だ。

### 設定ファイルの作成

Ghosttyの設定ファイルは `~/.config/ghostty/config` に配置する。

Alacrittyと同等の使い心地にするため、以下の設定を作成した。

```
# Ghostty設定
# Alacritty + tmux構成と同等の使い心地を目指す

# シェル設定 - 起動時にtmuxを自動起動
command = tmux new-session -A -s main

# フォント設定
font-family = JetBrainsMono Nerd Font
font-size = 12

# リガチャ有効
font-feature = calt
font-feature = liga

# ウィンドウ設定
window-width = 140
window-height = 35
window-decoration = server
window-padding-x = 4
window-padding-y = 4

# カラースキーム
theme = TokyoNight

# その他
mouse-hide-while-typing = true
cursor-style = block
cursor-style-blink = false
clipboard-read = allow
clipboard-write = allow
confirm-close-surface = false
gtk-single-instance = true
```

Ghosttyの設定形式は `key = value` というシンプルな構文だ。TOMLでもLuaでもない独自形式だが、直感的で分かりやすい。

### 設定のポイント

**1. tmux自動起動**

```
command = tmux new-session -A -s main
```

Alacrittyでは `[terminal.shell]` セクションで設定していたが、Ghosttyでは `command` で指定する。

**2. テーマ名の注意**

```
theme = TokyoNight
```

最初 `theme = tokyonight` と書いたらエラーになった。正しくは `TokyoNight`（キャメルケース）だ。

利用可能なテーマは以下のコマンドで確認できる。

```bash
ghostty +list-themes | grep -i tokyo
# TokyoNight (resources)
# TokyoNight Day (resources)
# TokyoNight Moon (resources)
# TokyoNight Night (resources)
# TokyoNight Storm (resources)
```

Tokyo Nightだけで5種類もある。至れり尽くせりだ。

**3. フォントサイズの違い**

Alacrittyでは `size = 10.0` だったが、Ghosttyでは同じサイズでも小さく見えた。GhosttyはGTK4ベースでフォントレンダリングが異なるためだろう。

最終的に `font-size = 12` で落ち着いた。

## Ghosttyの印象

実際に使ってみて感じたことを正直に書く。

### 良かった点

**1. Tokyo Nightテーマ内蔵**

設定ファイルに `theme = TokyoNight` と1行書くだけで、綺麗なTokyo Nightカラースキームが適用される。Alacrittyでは30行近いカラー定義を書く必要があった。これは楽だ。

**2. リガチャ対応**

`->` が矢印になる。`=>` も矢印になる。見た目は確かに綺麗だ。

ただ、前述の通り、私にはあまり刺さらなかった。リガチャしてもしなくても、コードの可読性に大きな差は感じない。これは個人差があるだろう。

**3. 設定がシンプル**

`key = value` 形式で、TOMLよりもシンプル。設定項目も少なめで、迷うことが少ない。

### 気になった点

**1. IMEの二重表示**

Fcitx5（SKK）を使っていると、IMEの候補ウィンドウが2つ表示される。システム側のウィンドウと、Ghostty内のインライン表示だ。

これはGhosttyがGTK4のIME統合を使って、preedit（入力中の文字）をアプリ内に表示する仕様のためだ。Alacrittyにはこの機能がないため、システム側のウィンドウのみ表示される。

Fcitx5側の設定で「Show preedit within application」を無効にすれば解消できる。ただ、Alacrittyでは何も設定せずに期待通りの動作だったので、この違いは気になった。

**2. フォント表示の違い**

Ghosttyのフォント表示も綺麗だ。しかし、Alacrittyの方が私の好みだった。

以前使っていたWeztermでは文字のジャギーが気になることがあったが、Alacrittyではそれがない。Ghosttyも綺麗だが、Alacrittyの方が「しっくりくる」感覚があった。これは完全に主観の問題だ。

**3. 日本語入力時の挙動**

Alacrittyでは日本語入力時にチラつきを感じないが、Ghosttyでは若干の違和感があった。致命的ではないが、長時間使うと気になるかもしれない。

## Alacrittyを選んだ理由

最終的にAlacrittyを選んだ理由をまとめる。

### 1. tmuxとの相性

Alacrittyは「文字を表示する」ことに特化している。マルチプレクサ機能は持たない。その潔さが、tmuxとの組み合わせで本領を発揮する。

専門家同士がタッグを組むようなものだ。描画のプロと、セッション管理のプロ。それぞれが得意分野に集中することで、全体として最高のパフォーマンスを発揮する。

### 2. 日本語入力の安定性

私の環境（Linux Mint 22.3 + Fcitx5 SKK + X11）では、Alacrittyの方が日本語入力が安定していた。チラつきもなく、候補ウィンドウも期待通りに動作する。

### 3. フォント表示の好み

主観的な話だが、Alacrittyのフォント表示が一番好みだった。Weztermよりジャギーがなく、Ghosttyよりも「しっくりくる」。

### 4. シンプルさとパフォーマンス

Alacrittyは機能が少ない分、軽量で高速だ。起動は一瞬。スクロールはヌルヌル。大量のログ出力でももたつかない。

リガチャは使えないが、私にとってはトレードオフとして許容できる範囲だった。

## どれを選ぶべきか

ここまで読んで「結局どれがいいの？」と思った方へ。

**答え: 好みで選べばいい。**

どのツールも素晴らしい。開発者が情熱を持って作っているプロダクトだ。優劣をつけるものではない。

参考までに、私なりの選定基準を書いておく。

### Weztermがおすすめな人

- 設定を一箇所で完結させたい
- Luaでの高度なカスタマイズがしたい
- マルチプレクサを別途インストールしたくない
- クロスプラットフォームで同じ環境を使いたい

### Alacrittyがおすすめな人

- シンプルさとパフォーマンスを重視する
- tmuxやZellijを既に使っている、または使いたい
- リガチャは不要
- TOMLで設定を書きたい

### Ghosttyがおすすめな人

- リガチャが欲しい
- 組み込みテーマを活用したい
- モダンなターミナルエミュレータを試したい
- Mitchell Hashimoto氏のファン

### tmuxがおすすめな人

- 安定性を重視する
- Neovimをヘビーに使う
- 豊富なプラグインを活用したい
- 枯れた技術の安心感が欲しい

### Zellijがおすすめな人

- 直感的なUIが好き
- Rust製で統一したい
- 浮動ペイン機能を使いたい
- tmuxの学習コストを避けたい

## おわりに

2026年1月、私のターミナル環境は以下に落ち着いた。

```
Alacritty（ターミナルエミュレータ）
    └── tmux（マルチプレクサ）
          ├── Window 1: Neovim + Claude Code
          ├── Window 2: 開発サーバー
          └── Window 3: git / cargo
```

Weztermを3年使い、Zellijを1日使い、Ghosttyを1日試し、最終的にAlacritty + tmuxに落ち着いた。

長い旅だった。

...と言いたいところだが、きっとまた環境を変えたくなるだろう。Zellijの描画問題が解決されたら戻るかもしれない。Ghosttyがさらに進化したら乗り換えるかもしれない。新しいターミナルエミュレータが登場したら試すかもしれない。

この終わりなき探求こそが、ターミナル沼の醍醐味なのかもしれない。

大事なのは、自分が心地よいと思える環境を見つけること。そのために試行錯誤することは、決して無駄ではない。

今回の記事が、誰かのターミナル選定の参考になれば幸いだ。

---

...と言いつつ、明日にはまた新しいツールを調べている自分が見える。

## 参考リンク

- [Ghostty公式](https://ghostty.org/)
- [Ghostty Documentation](https://ghostty.org/docs)
- [ghostty-ubuntu（Linux Mint/Ubuntu向けパッケージ）](https://github.com/mkasberg/ghostty-ubuntu)
- [Alacritty公式](https://alacritty.org/)
- [Wezterm公式](https://wezterm.org/)
- [tmux Wiki](https://github.com/tmux/tmux/wiki)
- [Zellij公式](https://zellij.dev/)
- [Linux Mint 22.3 "Zena" Release Notes](https://www.linuxmint.com/rel_zena.php)
