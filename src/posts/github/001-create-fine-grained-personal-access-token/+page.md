---
title: fine-grained personal access tokenの作成手順
description: fine-grained personal access tokenの作成手順です。
createdAt: '2024-4-6'
tags:
  - GitHub
published: true
---

<script>
  import Img from '$components/modules/Img.svelte';
  import HL from '$components/modules/HL.svelte';
</script>

<HL el="h2" text="gitでcloneしたときに..." />

`fine-grained personal access token` が必要になった場面は、 `git clone` を行ったときに下記のメッセージが表示されたためです。

```shell
$ git clone https://github.com/hoge/bar.git
Cloning into 'bar'...
git: 'credential-netrc' is not a git command. See 'git --help'.
Username for 'https://github.com':
Password for 'https://github.com':
```

[公式で解決手順](https://docs.github.com/ja/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#fine-grained-personal-access-token-%E3%81%AE%E4%BD%9C%E6%88%90) が公開されていたため、本記事では画像付きでまとめたいと思います。  
**※まだBeta機能っぽいのでページUIは変更される可能性があります。**

<HL el="h2" text="開発者設定を開く" />

右上のアカウントアイコンから、 **Settings** を押下します。

<Img src="/images/tool/001-create-fine-grained-personal-access-token/00-settings.png" alt="settings" />

**Developer settings** を押下します。

<Img src="/images/tool/001-create-fine-grained-personal-access-token/01-developer-settings.png" alt="developer settings" />

<HL el="h2" text="アクセストークンのページを開く" />

**Personal access tokens** > **Fine-grained tokens** を押下します。

<Img src="/images/tool/001-create-fine-grained-personal-access-token/02-fine-grained-tokens.png" alt="fine grained tokens" />

<HL el="h2" text="トークンを作る" />

**Generate new token** を押下してトークンを生成します。

<Img src="/images/tool/001-create-fine-grained-personal-access-token/03-generate.png" alt="generate" />

PINコードやモバイル端末での認証を行います。

<Img src="/images/tool/001-create-fine-grained-personal-access-token/04-use-security-key.png" alt="use security key" />

<HL el="h2" text="トークンの設定をする" />

トークン名や有効期限、リポジトリのアクセス権限などを設定します。

トークン発行までします。

<Img src="/images/tool/001-create-fine-grained-personal-access-token/05-token-entry.png" alt="token entry" />

<HL el="h2" text="終わり" />

生成されたトークンを控えます。  
これを `git clone` 時のパスワードで入力すると `git clone` 出来ます。

ただし、このままだとGitHub操作時に毎回ユーザー名とパスワードを入力する必要があります。  
セキュリティを考えると正しいですが、めんどくさいので何か方法がないか考えたいと思います。
