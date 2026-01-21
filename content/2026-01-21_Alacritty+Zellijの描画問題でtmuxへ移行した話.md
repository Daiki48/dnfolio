+++
title = "Alacritty+Zellijの描画問題でtmuxへ移行した話"
slug = "migrate-alacritty-zellij-to-tmux"
description = "昨日Alacritty + Zellijへ移行したばかりだが、描画抜けの問題に直面。原因を調査した結果、Zellijが原因と判明し、tmuxへ移行することにした。"
draft = false
[taxonomies]
tags = ["Alacritty", "Zellij", "tmux", "Terminal", "Neovim"]
languages = ["ja"]
+++

## 昨日の宣言から24時間

[昨日の記事](/posts/migrate-wezterm-to-alacritty-zellij/)で、私はこう書いた。

> WeztermからAlacritty + Zellijへの移行は、予想以上にスムーズだった。

そう、スムーズだった。**移行作業は**。

しかし、実際に開発を始めてみると、ある問題に気づいてしまった。

**描画が抜ける。**

## 問題の発生状況

症状は以下の通り。

- Neovimのterminalモード内でClaude Codeを使用中に発生
- 稀にドット抜けのような黒い箇所が出現
- カーソルを移動して該当行に当てると消える
- 再描画がトリガーされると修正される

開発中にコードを読んでいると、突然一部の文字が欠けている。最初は目の錯覚かと思ったが、明らかに描画の問題だ。カーソルを動かすと直るので致命的ではないが、地味に気が散る。

集中してコードを書いているときに「あれ、ここ何か抜けてない？」と思うたびに意識が途切れる。これは開発体験として良くない。

## 原因の切り分け

まずは原因を特定するために、いくつかの仮説を立てて検証した。

### 仮説1: Neovimの`termsync`オプション

Neovim 0.10以降、`termsync`オプションがデフォルトで有効になっている。これはCSI 2026（synchronized renders）というターミナルエスケープシーケンスを使用する機能だ。

```rust
// nvim-oxiで設定
api::set_option_value("termsync", false, &opts)?;
```

**結果**: 効果なし。

### 仮説2: NVIDIA + X11での透明度問題

私の環境はNVIDIA GPU + X11だ。透明度設定が描画問題を起こすことがあると聞いて、`opacity`を1.0に変更してみた。

```toml
[window]
opacity = 1.0
```

**結果**: 効果なし。

### 仮説3: Alacritty単体での検証

ここで発想を変えた。Zellijを経由せずにAlacritty単体で起動してみよう。

```bash
alacritty -e zsh
```

Neovimを起動し、terminalモードでClaude Codeを動かす。

**結果**: 描画抜けが発生しない。

**原因はZellijだった。**

## Zellijの描画問題

調査を進めると、この問題は既知のものだった。

- [Zellij Issue #1691](https://github.com/zellij-org/zellij/issues/1691) - Neovim black lines not fully rendered
- [Neovim Issue #29427](https://github.com/neovim/neovim/issues/29427) - termsync + Zellij cursor rendering issue

特に興味深いのは、Zellij開発者自身のコメントだ。

> "This issue is 99% a misimplementation of CSI 2026 (synchronized renders)"

CSI 2026の実装に問題があることを認めている。ただし、最終的にはZellij側で修正がマージされたとのことだが、私の環境（Zellij 0.43.1）ではまだ問題が発生していた。

## Neovimプラグインの影響を調査

Zellij側の問題とはいえ、Neovimのプラグインが描画問題を悪化させている可能性も考えた。特に、頻繁に画面更新を行うプラグインが怪しい。

### 調査対象

調査の結果、以下のプラグインが関係している可能性があった。

**1. fidget.nvim**

LSP進捗をフローティングウィンドウで表示するプラグイン。[Neovim Issue #26054](https://github.com/neovim/neovim/issues/26054)では、fidget.nvimと`termsync`の組み合わせでカーソルがちらつく問題が報告されている。

**2. snacks.nvim notifier**

folke氏のsnacks.nvimに含まれる通知機能。こちらもフローティングウィンドウを使用するため、Zellijの描画問題を悪化させる可能性がある。

### 検証

fidget.nvimを無効化し、lualineのステータスラインでLSP進捗を表示する方法を試みた。しかし、Zellijで起動したところ、Neovim自体が重くなり、まともにカーソルが動かない状態になった。

これは、プラグインの問題というよりも、Zellij自体のパフォーマンス問題が根本にあることを示唆している。

### 結論

Neovimのプラグイン構成を変更しても、Zellijの描画問題は解決しなかった。フローティングウィンドウを使うプラグインが問題を悪化させる可能性はあるが、根本原因はZellijのCSI 2026実装にある。

プラグインを犠牲にしてまでZellijを使い続けるメリットは薄いと判断した。

## tmuxへの移行を決断

選択肢は2つあった。

1. Zellijの修正を待つ
2. tmuxに移行する

開発中に描画が抜けるストレスは、想像以上に大きい。修正を待っている間も開発は続く。そこで、一旦tmuxに移行することにした。

### tmux vs Zellij

| 項目 | tmux | Zellij |
|------|------|--------|
| 成熟度 | 約15年の歴史 | 2021年〜 |
| TERM処理 | 独自の`tmux-256color`を使用 | 親ターミナルのTERMを継承 |
| Neovimとの互換性 | 広くテストされている | まだエッジケースが多い |
| CSI 2026対応 | 安定 | 実装に問題あり |

tmuxは長年の実績があり、Neovimとの組み合わせでも広くテストされている。安定性を求めるなら、tmuxは良い選択肢だ。

## tmux設定の作成

Zellijで使っていたキーバインドをtmuxでも再現した。

### プレフィックスなし（直接操作）

| 操作 | キー |
|------|------|
| ペイン移動 | Alt+h/j/k/l |
| ペイン分割（横） | Alt+n |
| ペイン分割（縦） | Alt+v |
| 前のウィンドウ | Ctrl+Shift+h |
| 次のウィンドウ | Ctrl+Shift+l |
| 新規ウィンドウ | Ctrl+Shift+t |

### プレフィックス付き（Ctrl+g → キー）

| 操作 | キー |
|------|------|
| ウィンドウ一覧 | Ctrl+g → w |
| セッション一覧 | Ctrl+g → s |
| デタッチ | Ctrl+g → d |
| 設定リロード | Ctrl+g → r |

Zellijの`Ctrl+g`をそのままtmuxのプレフィックスキーに設定したので、操作感はほぼ同じだ。

### Alacrittyのキーバインド設定

tmuxはCtrl+Shift系のキーを直接認識しにくいため、Alacritty側でキーバインドを設定してtmuxにコマンドを送る形にした。

```toml
[[keyboard.bindings]]
key = "T"
mods = "Control|Shift"
chars = "\u0007c"  # Ctrl+G → c

[[keyboard.bindings]]
key = "H"
mods = "Control|Shift"
chars = "\u0007p"  # Ctrl+G → p

[[keyboard.bindings]]
key = "L"
mods = "Control|Shift"
chars = "\u0007n"  # Ctrl+G → n
```

## 用語の違い

ZellijとtmuxではUIコンポーネントの呼び方が異なる。

| Zellij | tmux | 説明 |
|--------|------|------|
| Session | Session | 複数のタブをまとめる単位 |
| Tab | Window | セッション内の画面切り替え単位 |
| Pane | Pane | 画面を分割した領域 |

Zellijの「Tab」がtmuxでは「Window」と呼ばれる。概念自体は同じなので、呼び方だけ慣れれば問題ない。

## tmuxでのURL操作

Weztermではターミナル上のURLをクリックするだけでブラウザで開けた。tmuxではどうか？

tmuxは`mouse on`設定でマウスイベントをキャプチャするため、単純なクリックではURLを開けない。しかし、**Shift + クリック**でtmuxをバイパスし、Alacrittyに直接マウスイベントを渡せる。

| 操作 | 方法 |
|------|------|
| URLを開く | Shift + クリック |
| URL hints（キーボード） | Ctrl+Shift+O |

Shift + クリックで問題なくブラウザが開くので、Weztermと同等の体験が得られる。

## 今後の方針

### Zellijをウォッチし続ける

Zellijを完全に見捨てるわけではない。

- Rust製でモダンなUI
- 直感的な操作性
- 浮動ペイン機能

これらの魅力は健在だ。描画問題が解決されれば、また戻る可能性は十分にある。GitHubのissueをウォッチしつつ、状況を見守りたい。

### Ghosttyも試したい

実は、ターミナルエミュレータにはもう一つ気になる選択肢がある。[Ghostty](https://ghostty.org/)だ。

昨日の記事では「Rust製で統一したい」という理由でAlacrittyを選んだと書いた。しかし、正直に言うと、**Rust製への拘りはそこまで強くない**。

私はRustを書くのが好きだ。でも、それは「Rustで書かれたツールしか使わない」という意味ではない。良いツールは良いツール。言語は二の次だ。

GhosttyはZig製だが、Mitchell Hashimoto氏（HashiCorp共同創業者）が開発しているという点で技術的な信頼性が高い。GPUアクセラレーション、Kittyグラフィックスプロトコル対応など、機能面でも魅力的だ。

気が向いたら試してみたい。

## おわりに

昨日「Alacritty + Zellijに移行した」と堂々と宣言し、今日「やっぱりtmuxにした」と書いている。

なんとも情けない話だが、これが現実だ。

技術選定において「使ってみないと分からない」ことは多い。ベンチマークやドキュメントだけでは見えない問題がある。今回のように、特定の環境・特定の使い方でのみ発生する問題もある。

大事なのは、問題に直面したときに柔軟に対応できることだ。「昨日あれだけ褒めたのに...」というプライドは捨てて、素直に軌道修正する。開発環境は生産性を上げるためのものであり、こだわりを守るためのものではない。

とはいえ、Zellijの可能性は感じている。開発が進んで安定したら、また戻ってくるかもしれない。

それまでは、tmuxと共に歩んでいこう。15年の歴史を持つ老舗の安定感を、しばらく堪能させてもらう。

## 参考リンク

- [Alacritty公式](https://alacritty.org/)
- [Zellij公式](https://zellij.dev/)
- [tmux公式](https://github.com/tmux/tmux)
- [Ghostty公式](https://ghostty.org/)
- [Zellij Issue #1691 - Black lines not fully rendered](https://github.com/zellij-org/zellij/issues/1691)
- [Neovim Issue #29427 - termsync cursor rendering issue on Zellij](https://github.com/neovim/neovim/issues/29427)
- [Neovim Issue #26054 - Cursor flickering with termsync and fidget.nvim](https://github.com/neovim/neovim/issues/26054)
- [Alacritty Issue #6285 - Rendering problem with Zellij](https://github.com/alacritty/alacritty/issues/6285)
