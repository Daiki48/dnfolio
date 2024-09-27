---
title: WSL2の割り当てドライブを変更する
description: WSL2の割り当ててあるドライブを変更する手順です。
createdAt: '2024-4-6'
updatedAt: '2024-4-7'
tags:
  - WSL2
published: true
---

<script>
  import Img from '$components/modules/Img.svelte';
  import HL from '$components/modules/HL.svelte';
</script>

<HL el="h2" text="デフォルトユーザーのUIDを確認" />

今回、割り当てドライブを変更するディストリビューションでデフォルトユーザーのUIDを確認します。  
割り当て後に設定するため事前に控えておく必要があります。

ディストリビューションで下記コマンドを実行します。

```shell
$ id -u
```

**1000** と表示されました。メモしておきます。

<HL el="h2" text="ディストリビューションのエクスポート" />

今後割り当てる予定のドライブに移動してディレクトリを作成します。  
私は、 **Uドライブ** に `wsl` ディレクトリを作りました。

**Uドライブへ移動** します。

```powershell
$ cd U:\\
```

`wsl` ディレクトリを作成します。

```powershell
$ mkdir wsl
```

`wsl` ディレクトリへ移動します。

```powershell
$ cd .\\wsl\\
```

移動するディストリビューションの名前を確認します。

```powershell
$ wsl -l -v
```

<Img src="/images/tool/002-change-drive-wsl2/06-wsl-l-v.png" alt="wsl -l -v" />

`Ubuntu` という名前のディストリビューションをUドライブに割り当てるので、エクスポートします。

```powershell
$ wsl --export Ubuntu ubuntu.tar
```

<Img src="/images/tool/002-change-drive-wsl2/01-wsl-export-tar.png" alt="wsl export tar file" />

<HL el="h2" text="ディストリビューションのアンインストール" />

Uドライブでも同じディストリビューションの名前を使いたいので、削除します。

```powershell
$ wsl --unregister Ubuntu
```

<Img src="/images/tool/002-change-drive-wsl2/02-unregister.png" alt="unregister" />

<HL el="h2" text="ディストリビューションのインポート" />

先ほどエクスポートした `ubuntu.tar` をインポートします。

```powershell
$ wsl --import Ubuntu Ubuntu ubuntu.tar --version 2
```

<Img src="/images/tool/002-change-drive-wsl2/03-wsl-import-tar.png" alt="wsl import tar file" />

<HL el="h2" text="ユーザー設定" />

最初の **デフォルトユーザーのUIDを確認** でメモした数字を設定します。

windowsのタスクバーにある検索か、もしくはwindowsキー（スタートボタン）押下後の画面で一番上にある検索欄に **regedit** と入力します。  
レジストリエディターを開きます。

**HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Lxss** に文字列ディレクトリが存在します。  
**DistributionName** が今回のデータの文字列ディレクトリを押下して、 **DefaultUid** をダブルクリックします。

<Img src="/images/tool/002-change-drive-wsl2/04-default-user-uid.png" alt="default user uid" />

10進数で **1000** を設定します。

<Img src="/images/tool/002-change-drive-wsl2/05-default-user-uid-1000.png" alt="default user uid 1000" />

<HL el="h2" text="終わり" />

以上でディストリビューションの割り当てドライブを変更出来ました。

<HL el="h2" text="参考資料" />

- [ディストリビューションをエクスポートする](https://learn.microsoft.com/ja-jp/windows/wsl/basic-commands#export-a-distribution)
- [ディストリビューションをインポートする](https://learn.microsoft.com/ja-jp/windows/wsl/basic-commands#import-a-distribution)
- [WSL で使用する Linux ディストリビューションをインポートする](https://learn.microsoft.com/ja-jp/windows/wsl/use-custom-distro)
- [Linuxディストリビューションの登録解除またはアンインストール](https://learn.microsoft.com/ja-jp/windows/wsl/basic-commands#unregister-or-uninstall-a-linux-distribution)
