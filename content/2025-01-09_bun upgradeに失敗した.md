+++
title = "bun upgradeに失敗した"
slug = "bun-upgrade-failed"
description = "bun v1.1.42からv1.1.43にアップグレードしようとした時に失敗した。"
draft = false
[taxonomies]
tags = ["Bun"]
languages = ["ja"]
+++

## 実行した結果

[bun](https://bun.sh) をアップグレードしようと、 `bun upgrade` を実行した。

```sh
bun upgrade

Bun v1.1.43 is out! You're on v1.1.42
error: Failed to move new version of Bun to C:\Users\Owner\.bun\bin\bun.exe to EBUSY

Please reinstall Bun manually with the following command:
   powershell -c 'irm bun.sh/install.ps1|iex'
```

失敗した。

## 再インストール

再インストールするよう書かれているので実施してみる。

```sh
powershell -c 'irm bun.sh/install.ps1|iex'

############################################################ 100.0%
Bun 1.1.43 was installed successfully!
The binary is located at C:\Users\Owner\.bun\bin\bun.exe

To get started, restart your terminal/editor, then type "bun"
```

## バージョンを確認

`bun --version` で現在のバージョンを確認してみる。

```sh
bun --version

1.1.43
```

初めて `bun upgrade` に失敗したが、実行結果に再インストールするよう記載があることで躓くことなくアップグレードに成功した。
