+++
title = "WSL2環境構築(Zsh, Neovim)"
description = "WSL2にZshとNeovim環境を構築した。WSL2はデフォルトでBashとViが導入されているが、私自身他のLinux環境ではZshとNeovimを利用しているため。"
slug = "WSL2環境構築(Zsh, Neovim)"
draft = false
[taxonomies]
tags = ["WSL2", "Zsh", "Neovim"]
languages = ["日本語"]
+++

## なぜWSL2の環境構築をするのか

`PowerShell` の環境が落ち着いてきたため、次は `WSL2` の環境を整備したいと思う。
昔導入していた頃、 `.wslconfig` みたいなファイルで制御出来なかったメモリとCPUの爆食い症状を改善して導入したいというモチベーション。

## 状態を確認

現在、WSL2の状態はこのようになっている。

```sh
wsl -l -v
  NAME                   STATE           VERSION
* Ubuntu                 Stopped         2
  docker-desktop         Stopped         2
  docker-desktop-data    Stopped         2
```

この `Ubuntu` を再インストールする。

この時、停止していなければシャットダウンする。

```sh
wsl --shutdown
```

## 登録を解除

`Ubuntu` の登録を解除する。

```sh
wsl --unregister ubuntu
登録解除。
この操作を正しく終了しました
```

## Windowsの機能の有効化または無効化

Windowsスタート画面の検索に「Windowsの機能の有効化または無効化」と入力すると **Windowsの機能** というウィンドウが開く。
**Linux用Windowsサブシステム** と **Virtual Machine Platform** のチェックを外して **OK** を押す。

再起動はまだしない。

## インストールされているアプリでアンインストール

`Ubuntu` の停止と登録解除まで終わったら、次はアプリをアンインストールする。
設定を開いて **アプリ** > **インストールされているアプリ** で以下をアンインストールする。

- Ubuntu on Windows
- Windows Subsystem for Linux Update
- Windows Subsystem for Linux WSLg Preview
- Linux用Windowsサブシステム

## PC再起動

反映するために再起動する。

## `Ubuntu` の再インストール

Windowsスタート画面の検索に「Windowsの機能の有効化または無効化」と入力すると **Windowsの機能** というウィンドウが開く。
**Linux用Windowsサブシステム** と **Virtual Machine Platform** にチェックして **OK** を押す。

## PC再起動

**Windowsの機能の有効化または無効化** を変更すると再起動する。

## `Windows PowerShell` を管理者権限で実行

管理者権限で以下のコマンドを実行する。

```sh
wsl --install
```

> このコマンドで **Linux用Windowsサブシステム** と **Virtual Machine Platform** もインストール出来るらしい。なのでひとつ前の手順をせずにここでインストールしても良さそう。(試してはいない)

## バージョンを確認

```sh
cat /etc/os-release
PRETTY_NAME="Ubuntu 24.04.1 LTS"
NAME="Ubuntu"
VERSION_ID="24.04"
VERSION="24.04.1 LTS (Noble Numbat)"
VERSION_CODENAME=noble
ID=ubuntu
ID_LIKE=debian
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
UBUNTU_CODENAME=noble
LOGO=ubuntu-logo
```

現時点で最新安定板の `Ubuntu 24.04.1 LTS` であることが確認出来る。

> どこかの手順でユーザー名を登録したりする。

## `zsh --version`

そもそも `Zsh` がまだ導入されていないことを確認する。

```sh
zsh --version
```

ここでバージョンが出力された場合、Zsh導入手順はスキップする。

## `Zsh` を導入

その前に、Windows利用出来る [Windows Terminal](https://apps.microsoft.com/detail/9n0dx20hk701?hl=ja-JP&gl=JP) をインストールしておく。Microsoft Storeにある。

WSL2のUbuntuを起動してパッケージリストを更新する。

```sh
sudo apt update
```

インストールする。

```sh
sudo apt install -y zsh
```

バージョンを確認する。

```sh
zsh --version
```

## 現在のShellを確認

現在利用中のShellを確認する。何もしていなければ `Bash` のはず。

```sh
echo $SHELL
```

## `Bash` から `Zsh` へ変更

利用するシェルを変更する。

```sh
chsh -s $(which zsh)
```

Windows Terminalを再起動してから、もう一度利用中のShellを確認する。

```sh
echo $SHELL
```

### `zsh compinit` エラー

[Deno](https://deno.com) をインストールした頃から、`Zsh` でWSL2を起動した際にメッセージが表示されるようになった。

```sh
zsh compinit: insecure directories, run compaudit for list.
Ignore insecure directories and continue [y] or abort compinit [n]?
```

このメッセージで `y` や `n` を押さずにエンター連打で以下のようにした。

```sh
zsh compinit: insecure directories, run compaudit for list.
Ignore insecure directories and continue [y] or abort compinit [n]?
compinit: initialization aborted
zsh compinit: insecure directories, run compaudit for list.
Ignore insecure directories and continue [y] or abort compinit [n]?
compinit: initialization aborted
complete:13: command not found: compdef
```

> ここで `y` を押してから `compaudit` コマンドを実行すると何も表示されない。無視される。
> `n` にするとWSL2が終了する。なのでこちらも答えずにエンターを連打する。
> この方法が正しいのかは別として私は解決した。

この状態で `compaudit` を確認する。

```sh
compaudit
There are insecure directories:
/home/daiki48/.zsh/completions
```

権限を付与する。

```sh
chmod 755 /home/daiki48/.zsh/completions
```

ターミナルを再起動すると、メッセージが出力されなかった。この方法で合っているのかは不明...

## Neovimが導入されているか確認

次はNeovimを導入する。
その前に確認する。

```sh
nvim --version
Command 'nvim' not found, but can be installed with:
sudo snap install nvim    # version v0.10.4, or
sudo apt  install neovim  # version 0.7.2-8
See 'snap info nvim' for additional versions.
```

まだ見つからない。
出力を見てみると `snap` でNeovimをインストール出来そう？

## `snap --version`

snapを確認する。

```sh
snap --version
snap    2.67
snapd   2.67
series  16
ubuntu  24.04
kernel  5.15.167.4-microsoft-standard-WSL2
```

snapでインストール出来そうなので試してみる。

> `sudo apt install neovim` も出来るがバージョンが古い。
> よって、以前までは `AppImage` からビルドする方法をしていた。今回もその方法でやるつもりだったが、ユニバーサルパッケージマネージャーsnapがすでにインストールされていたのでこっちを利用した方が今後のバージョン管理も楽だ。

以前導入していた `Ubuntu 20` では `snap` 入っていなかったような...覚えてないけど...

## `sudo snap install nvim`

早速出力されたメッセージ通りにコマンドを実行してみる。

```sh
sudo snap install nvim
error: This revision of snap "nvim" was published using classic confinement and thus may perform
       arbitrary system changes outside of the security sandbox that snaps are usually confined to,
       which may put your system at risk.

       If you understand and want to proceed repeat the command including --classic.
```

ダメだった。
なんか危険っぽいメッセージが表示されているが `--classic` オプションを付与すると出来そうなのでやってみる。

## `sudo snap install nvim --classic`

このコマンドで出来なかったら `AppImage` でビルドする方法にしよう。

```sh
sudo snap install nvim --classic
2025-02-23T01:35:14+09:00 INFO Waiting for automatic snapd restart...
nvim v0.10.4 from neovim-snap (neovim-snap) installed
```

出来た！

## `zsh: command not found: nvim`

インストールしたばかりなのになぜ...

どうもsnapのパスが通っていないっぽい。先駆者がいたので参考にして解決してみる。

{{ card(title="snapでインストールしたコマンドがzshでPATH通ってない", url="https://qiita.com/sameyasu/items/072882ee92bca54906d8") }}

## `/etc/zsh/zprofile` を編集

`zprofile` ではデフォルトで以下の内容になっていた。

```sh
# /etc/zsh/zprofile: system-wide .zprofile file for zsh(1).
#
# This file is sourced only for login shells (i.e. shells
# invoked with "-" as the first character of argv[0], and
# shells invoked with the -l flag.)
#
# Global Order: zshenv, zprofile, zshrc, zlogin
```

ここに追記する。

```sh,hl_lines=9-13
# /etc/zsh/zprofile: system-wide .zprofile file for zsh(1).
#
# This file is sourced only for login shells (i.e. shells
# invoked with "-" as the first character of argv[0], and
# shells invoked with the -l flag.)
#
# Global Order: zshenv, zprofile, zshrc, zlogin

# Expand $PATH to include the directory where snappy applications go.
snap_bin_path="/snap/bin"
if [ -n "${PATH##*${snap_bin_path}}" -a -n "${PATH##*${snap_bin_path}:*}" ]; then
    export PATH=$PATH:${snap_bin_path}
fi
```

## `nvim --version`

再度バージョンを確認する。

```sh
nvim --version
NVIM v0.10.4
Build type: RelWithDebInfo
LuaJIT 2.1.1713484068
Run "nvim -V1 -v" for more info
```

現時点で最新の `v0.10.4` を導入出来た。

## おわりに

とりあえず `Zsh` と `Neovim` が利用出来る環境を作った。このままでも良いが私は自分の [dotfiles](https://github.com/Daiki48/dotfiles) 環境を利用する。
