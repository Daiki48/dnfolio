+++
title = "GitHub OrganizationでPush出来なかったため、GitHub CLIを導入した"
slug = "github-org-push-github-cli"
description = "GitHubでOrganizationで作成したリポジトリにアクセス出来ない問題が発生しました。TOKEN内の権限やOrganization内での権限に問題が無いのに発生して途方に暮れていたところGitHub CLIで認証すると解決したという声があったため試しに導入したら成功した。"
created = "2025-08-24"
draft = false
[taxonomies]
tags = ["GitHub", "GitHub CLI"]
languages = ["ja"]
+++

## 導入理由

ある日、Organizationで作成したリポジトリに対してPushしようとしましたが、権限エラーが発生しました。

```sh
git push -u origin main
remote: Write access to repository not granted.
fatal: unable to access 'https://github.com/<Organization>/<Repository>.git/': The requested URL returned error: 403
```

あらゆる権限を調査したが問題無かった。同じ組織内のユーザーはリポジトリへのアクセスが出来ていたので、私が利用しているトークンに問題があるかと思い調査しました。
Organization内のリポジトリへの書き込み権限も備わっていました。

## GitHubのコミュニティで確認

GitHub CommunityというOrganizationが存在しました。
そこで **remote: Write access to repository not granted. #46398** というタイトルでディスカッションが存在したため確認しました。

https://github.com/orgs/community/discussions/46398#discussioncomment-4872798

GCMを利用した方法もあるみたいですが、以前か気になっていた **GitHub CLI** を導入してみました。

## インストール

ドキュメントに沿って導入します。

https://github.com/cli/cli/blob/trunk/docs/install_linux.md

以下のコマンドですんなり導入出来ました。

```sh
(type -p wget >/dev/null || (sudo apt update && sudo apt install wget -y)) \
	&& sudo mkdir -p -m 755 /etc/apt/keyrings \
	&& out=$(mktemp) && wget -nv -O$out https://cli.github.com/packages/githubcli-archive-keyring.gpg \
	&& cat $out | sudo tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null \
	&& sudo chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
	&& sudo mkdir -p -m 755 /etc/apt/sources.list.d \
	&& echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
	&& sudo apt update \
	&& sudo apt install gh -y
```

## 今後のアップデート

これもドキュメントに記載があります。

```sh
sudo apt update
```

```sh
sudo apt install gh
```

## 認証

本題の認証を行います。

```sh
gh auth login
```

`github.com` で認証作業を進めます。

## おわりに

```sh
git push -u origin main
```

GitHub CLIの導入により、プッシュ出来るようになりました。
同じような問題に困っている方の参考になれば幸いです。
