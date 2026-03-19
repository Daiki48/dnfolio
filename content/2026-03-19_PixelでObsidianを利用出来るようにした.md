+++
title = "PixelでObsidianを利用出来るようにした"
slug = "pixel-obsidian-using"
description = "Pixel 10 Pro XLを利用していて、Obsidianに書いたメモを確認したい時、メモしたい時があったので標準ターミナルアプリを使って実現しました"
created = "2026-03-19"
draft = false
[taxonomies]
tags = ["Pixel", "Android", "Obsidian"]
languages = ["ja"]
+++

## はじめに

ObsidianというMarkdownノートアプリを愛用している。

これまでは、プライベート(Linux Mint)と仕事(Windows)でのみObsidianを利用して、コミュニティプラグインの [obsidian-git](https://github.com/Vinzent03/obsidian-git) を利用してGit管理していた。
GitHubに専用リポジトリを用意して、仮に外出先でも閲覧したい際はGitHub MobileアプリでPixelから直接確認していた。ただこれもなかなかめんどくさいし、書き込みに関しては難しいので困っていた。GoogleのKeepとかで一旦メモしてパソコンを触れる環境に戻ってから清書するような運用だった。

## Android 15

[開発者向けオプションを私用してデバイスでLinux開発環境を有効にする](https://developer.android.com/about/versions/15/release-notes?hl=ja#linux-development-environment)

ちゃんとしたDebian環境っぽい。

`obsidian-git` のモバイル対応はJavaScriptで再実装してるみたいで不安定と公式が注意書きしているので使えない。なのでこのAndroid標準のターミナルで `Git` を使った管理をすれば良いということになった。
モバイル画面でのターミナル操作は、最初こそ慣れなかったけど今はそこそこ慣れた。やっぱり使えば慣れるものだ。

最初の `git` をインストールする部分だけ時間がかかるけど、それ以外はスムーズだった。

## 普段利用の流れ

私の場合、モバイル環境でメモしたいという場面はそこまで多くない。基本的にはPCでいろいろメモしたノートを隙間時間に見ることが目的。

なので、まずは

```sh
git pull origin main
```

そしたらPixel内のVaultが最新状態になるので閲覧って感じ。

逆にPixelでObsidianにメモしたら、都度コミットしていかないといけない。じゃないと次回移行でコンフリクトチャンスきちゃうかもしれないので。

## Fold欲しくなってきた

Pixel 10 Pro XLも充分大きいサイズの端末だと思っていたが、こういう使い方が増えてくるとFold環境が欲しくなる。Pixel 11 Pro Foldが発売されるか分からないけど、Android標準ターミナルはチップの関係上Galaxyには搭載されないと思うからPixel系の次回以降の端末を楽しみにウォッチし続けようと思う。

## まとめ

標準でLinux環境を提供してくれるAndroidありがとう。普段からコーディングしてる身からするとGitで管理出来る点は嬉しい。
