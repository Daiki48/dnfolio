+++
title = "WSL2のUbuntuにZigをインストールする"
slug = "wsl2-ubuntu-zig-install"
description = "ZigをUbuntuにインストールしてみる。"
draft = false
[taxonomies]
tags = ["Zig"]
languages = ["ja"]
+++

## ダウンロード

`wget` コマンドからダウンロードするでも良いが今回はZig公式のダウンロードページからダウンロードしてみる。

[Releases | Zig](https://ziglang.org/download/)

最新バージョン `0.14.0` から `x86_64` を選択した。これは自分が利用しているマシンによって選択する。

`zig-linux-x86_64-0.14.0.tar.xz`

Windows側にダウンロードされると思うのでUbuntu側に移動する。\
私の場合は `$HOME/tmp/` ディレクトリ内に移動した。

## インストールするディレクトリを作成

Ubuntu側で作成する。
`/usr/local` ディレクトリはすでに存在するはずなのでその中に `zig` ディレクトリを作成する。

```sh
mkdir /usr/local/zig
```

## インストール

では、準備は整ったので早速インストールしてみる。

```sh
cd ~/tmp
```

そしてインストールを実行する。

```sh
sudo tar Jxvf zig-linux-x86_64-0.14.0.tar.xz -C /usr/local/zig --strip-components 1
```

## PATH追加

私はZshを利用しているので `.zshrc` に以下を追記してパスを通す。

```sh
export PATH="/usr/local/zig:$PATH"
```

そしてターミナルを再起動する。

## 確認

Zigのバージョンを確認するコマンドを実行する。

```sh
zig version
```

これでバージョンが返ってきた！

Zigを利用する予定は無いけどね...

とても参考になった [Zig言語をUbuntu(WSL2)にインストール](https://qiita.com/bam_b0o_/items/27770ffcf0551ce9e813) に感謝！
