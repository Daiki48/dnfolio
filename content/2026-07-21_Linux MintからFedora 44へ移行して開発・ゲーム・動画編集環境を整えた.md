+++
title = "Linux MintからFedora 44へ移行して開発・ゲーム・動画編集環境を整えた"
slug = "linux-mint-to-fedora-44-migration"
description = "Linux Mint 22.3からFedora Linux 44 KDE Plasmaへ移行した。dotfilesを使った開発環境の再構築、Steamライブラリの共有、KdenliveとFFmpeg、Docker・Android開発環境まで、移行後に行った作業をまとめる。"
created = "2026-07-21"
draft = false
[taxonomies]
tags = ["Fedora", "Linux Mint", "KDE Plasma", "Steam", "Kdenlive"]
languages = ["ja"]
+++

2026年1月に書いた[開発環境2026](/posts/develop-environment-2026/)では、メインPCのOSにLinux Mint 22.2を使っていた。

記事の最後には、こんなことを書いている。

> 1年後、この記事を読み返したとき、私はどんな環境を使っているだろうか。Linux Mintは22.3になっているだろうか。

Linux Mintは22.3になった。

そして、その半年後にはFedoraへ移行していた。

未来はわからないから面白い。

今回は、Linux MintからFedoraへ移行したあと、開発、ゲーム、動画編集の環境をどのように整えたのかを記録しておく。

移行直後はLinux Mintとのデュアルブート構成を残していた。Fedoraですべての作業ができると確認する前に、長く使ってきた環境を消す勇気はさすがになかった。

## 移行後の環境

2026年7月時点の環境はこちら。

| 項目 | 環境 |
|---|---|
| OS | Fedora Linux 44 KDE Plasma Desktop Edition |
| ディスプレイサーバー | Wayland |
| CPU | AMD Ryzen 5 5600X |
| GPU | AMD Radeon RX 9060 XT 16GB |
| RAM | 32GB |
| ターミナル | WezTerm + tmux |
| エディタ | Neovim |
| 日本語入力 | Fcitx5 + SKK |

Fedoraを使うこと自体は初めてではない。以前から別のPCへ入れて遊んでいたので、`dnf`やKDE Plasmaには触れたことがあった。

しかし、遊ぶためのサブPCと、毎日使うメインPCでは求められるものが違う。

開発できること。

ゲームで遊べること。

動画を編集できること。

この3つをFedoraへ持ってこられなければ、移行したとは言えない。

## 最初に行ったセットアップ

Fedoraを起動して最初にシステムを更新した。

```bash
sudo dnf upgrade --refresh
```

その後、普段使うものを順番に導入した。

- Google Chrome
- Fcitx5、SKK辞書、KDE用の設定ツール
- Git、GitHub CLI
- Rust
- C/C++コンパイラーとビルドツール
- zsh
- WezTerm
- tmux

Rustを入れているのは開発のためでもあるが、私のdotfilesに含まれるセットアップCLI自体がRust製だからでもある。

```bash
git clone https://github.com/Daiki48/dotfiles.git
cd dotfiles

cargo run -- --distro fedora zsh
cargo run -- --distro fedora tmux
cargo run -- --distro fedora wezterm
```

dotfiles側は以前からUbuntuとFedoraを切り替えられるようにしていた。新しいPCでコマンドを実行すれば、見慣れた環境が戻ってくる。

少なくとも、そう思っていた。

## dotfilesだけでは環境を再現できなかった

設定ファイルをGitで管理していても、OSにインストールされている実行環境までは自動的に生えてこない。

Fedoraで最初にzshを起動すると、Denoの初期化ファイルが存在しないためエラーになった。

```zsh
# 変更前
. "$HOME/.deno/env"

# 変更後
[ -f "$HOME/.deno/env" ] && . "$HOME/.deno/env"
```

Linux MintにはDenoが入っていたため気づかなかった。長く使っている環境では「存在して当たり前」になっていたものが、まっさらな環境では一つずつ姿を現す。

Node.js、Python、Denoをどのように管理するかも見直した。個別のインストーラーを増やすのではなく、miseへまとめることにした。

dotfilesのCLIへFedora用のmiseセットアップを追加し、現在は次のコマンドで実行環境まで導入できる。

```bash
cargo run -- --distro fedora mise node@lts python@latest deno@latest
```

OS移行は面倒だが、dotfilesの暗黙的な依存関係を見つける良いテストにもなった。

## WezTermとtmuxで制御文字が表示された

ターミナル環境では、Git操作のたびに`1337;SetUserVar=...`という謎の文字列が表示される問題が発生した。

最初はGitの問題に見えたが、実際にはWezTermのシェル統合が出力するOSC制御シーケンスを、tmuxが正しく処理できていなかった。

tmuxではパススルーを有効にした。

```tmux
set -g allow-passthrough on
```

さらに、tmux内のNeovimで起動するターミナルでは、子シェルのWezTermシェル統合を無効化した。

```lua
if vim.env.TMUX then
  vim.env.WEZTERM_SHELL_SKIP_ALL = "1"
end
```

外側にWezTerm、その中にtmux、さらにNeovimの`:terminal`がある。

ターミナルの入れ子が深い。

しかし、ここまでしないと落ち着いて開発できないのだから仕方がない。

## Steamゲームは再ダウンロードしたくない

Linux MintではSteamを使い、OverwatchやDead by Daylightなどを別のSSDへインストールしていた。

Fedoraへ移行したからといって、何十GBもあるゲームをすべてダウンロードし直すのは避けたい。

既存のゲーム本体はそのまま共有し、Protonの`compatdata`だけをFedora用に分離する構成にした。

```text
既存のSteamライブラリ
├── steamapps/common/       ゲーム本体はLinux MintとFedoraで共有
└── steamapps/compatdata/   Fedora用のext4領域をbind mount
```

ゲーム本体が置かれているSSDはNTFSだった。読み込み中心のゲーム本体は共有しつつ、Wine prefixを含む`compatdata`はLinuxのパーミッションやシンボリックリンクを安全に扱えるext4へ置く。

Fedora用のディレクトリを作り、systemdのmount unitでSteamが期待する場所へbind mountした。

これにより、Linux Mint側のProton環境を壊さず、Fedoraでは別のprefixを利用できる。

同じゲーム本体を使いながら、OSごとの実行環境は混ぜない。デュアルブート中の安全策として、かなり気に入っている構成だ。

## KdenliveはAppImage版を継続した

Linux Mintでは、KdenliveのFlatpak版で不具合に悩まされたため、最終的にAppImage版を使っていた。

自動更新で突然挙動が変わらず、自分で更新するタイミングを決められる。Fedoraでも同じ運用を続けることにした。

Kdenlive 26.04.3のAppImageを起動し、Pixelで撮影した動画を使って実際の編集フローを確認した。

扱った動画はHEVC Main 10、HLG HDR、約60fps。GPUは[RTX 3060 Tiから換装したRX 9060 XT](/posts/gpu-swap-rtx3060ti-to-rx9060xt/)である。

ここで、いくつか問題が見つかった。

### AppImageとVAAPIの相性

システム側のFFmpegでは、RX 9060 XTを使ったAV1 VAAPIエンコードに成功した。

一方、Kdenlive AppImageでは、同梱されているlibvaとFedora側のMesa VA-APIドライバーの組み合わせで初期化に失敗した。

また、Kdenliveの`Lossy x264 I frame only (VAAPI GPU)`は8bitの`nv12`へ変換するため、10bit HLG素材の中間ファイルには向いていなかった。

GPUが使えるからといって、何でもGPUへ任せれば良いわけではない。

安定性と画質を優先し、中間ファイルにはCPU版の`Lossy x264 I frame only`を使った。10bitとHLGの色情報を維持できることも確認した。

### HLG HDRをSDRへ変換する

普段作っている猫動画は、Pixelで撮影した本編へ5秒のオープニングを付けてYouTubeへアップロードする程度だ。

しかし、オープニングは30fpsのSDR、本編は約60fpsのHLG HDR。同じタイムラインへ置くだけでは、フレームレートも色空間も揃わない。

FFmpegの`zscale`と`tonemap`を使い、HLG HDRからRec.709 SDRへ変換する実験を行った。

変換後は次の内容を確認した。

- 1920x1080
- 60fps固定
- 10bit
- BT.709
- AAC 48kHzステレオ

この調査は、定型的な動画結合だけを行うRust製デスクトップアプリ「catroom-generate」を作るきっかけにもなった。

OS移行の確認作業から、新しいアプリが一つ生えた。

予定にはなかったが、いつものことである。

## Fedora移行直後はHEVC動画を再生できなかった

移行から数日後、GoogleフォトからダウンロードしたPixelの動画をクリックしても再生できないことに気づいた。

Kdenliveで変換したH.264動画は再生できる。元のHEVC Main 10動画だけ再生できない。

原因は動画の破損ではなく、Fedora側のコーデック構成だった。私の環境に入っていた`ffmpeg-free`では、対象動画をデコードできなかった。

RPM FusionはSteamの導入時に有効化済みだったため、完全版のFFmpegへ置き換えた。

```bash
sudo dnf swap ffmpeg-free ffmpeg --allowerasing
```

置き換え後は、システムのFFmpegで動画を最後までデコードでき、デスクトップの動画プレイヤーからも再生できた。

Linux Mintでは意識せず再生できていたファイルが、Fedoraではコーデックを明示的に用意しないと再生できない。

ディストリビューションの違いを最も強く感じた部分だった。

## DockerとAndroid開発環境

開発ではDockerを日常的に使う。

FedoraにはPodmanが入っていたが、既存プロジェクトとの互換性を優先してDocker CEを導入した。

Fedoraを入れたディスクには余裕が少なかったため、開発ソースコードや容量の大きいデータは別のext4 SSDへ置く方針にした。

Android開発環境も作り直した。

- Android Studio
- Android SDK、NDK
- RustのAndroidターゲット
- Pythonの`requests`と`cryptography`
- 32bit互換ライブラリの導入状況

最初のAABビルドでは、Linux Mintに存在していたPythonパッケージやRustターゲットが不足していた。一つずつ導入し、既存のTauri v2アプリをビルド・デプロイできるところまで確認した。

ここでも「前の環境では動いていた」が何度も登場した。

新しいOSが悪いわけではない。前のOSへ長い時間をかけて積み上げたものを、自分が把握していなかっただけだ。

## 移行して分かったこと

### dotfilesは設定ファイルしか運んでくれない

dotfilesがあれば、シェルやエディタの見た目はすぐに戻る。

しかし、LSP、言語ランタイム、コーデック、フォント、IME、コンテナー環境までは別だ。

今回の移行を通じて、dotfilesのセットアップCLIがかなり育った。新しい環境で壊れた箇所は、次回のセットアップで壊れないように修正した。

### 安定していた環境には理由がある

Linux MintではSteamも動画再生も自然に動いていた。

それはLinux Mintだから魔法のように動いていたのではなく、過去の自分が必要なパッケージや設定を少しずつ追加していた結果だった。

環境を移行すると、その積み重ねが見える。

### 作業記録は残しておいた方が良い

今回の記事は、シェル履歴、dotfilesのGit履歴、設定ファイル、AIとの会話履歴を読み返して書いた。

当時は記事を書くつもりで記録していたわけではない。それでも、実行したコマンドと判断した理由が残っていたため、移行作業を時系列で復元できた。

人間の記憶は思っている以上に曖昧だ。

一週間前の作業ですら、もう細部を忘れていた。

## まとめ

Linux MintからFedora 44へ移行し、次の環境を再構築した。

- Fcitx5 + SKKによる日本語入力
- Rust、mise、Neovimを中心とした開発環境
- WezTerm + tmuxのターミナル環境
- 既存SSDを再利用したSteam環境
- Kdenlive AppImageとFFmpegによる動画編集環境
- Docker CE
- Tauri v2向けAndroid開発環境

Fedoraへ移行しただけで、劇的に開発が速くなったわけではない。

ターミナルを開き、Neovimでコードを書き、Steamでゲームを起動し、猫動画を編集する。やっていることはLinux Mintの頃とほとんど変わらない。

OSが変わっても、私の生活はだいたいターミナルの中にある。

ただ、まっさらな環境へ移ったことで、自分が何に依存して生活しているのかはよく分かった。

半年後、私はまだFedoraを使っているだろうか。

今の自分はFedora 44を気に入ってサブのThinkPadとメインのデスクトップで利用しているが、Fedora 45やその先のバージョンでも気に入って使っているか分からない。他のディストリビューションに興味を持ったらすぐに試したくなる性格だからどんな環境でもすぐに自分専用環境を試せるようにdotfilesは育てておく。
