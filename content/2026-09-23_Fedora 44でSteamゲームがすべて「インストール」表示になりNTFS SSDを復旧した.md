+++
title = "Fedora 44でSteamゲームがすべて「インストール」表示になりNTFS SSDを復旧した"
slug = "fedora-44-steam-library-ntfs-mount-issue"
description = "Fedora 44で突然Steamゲームが未インストールのように見え、ゲーム用NTFS SSDとHDDが自動マウントされなくなった。kernel 7.2.6のNTFS構成変更との関連を調べ、ntfs-3gで恒久マウントへ切り替えて復旧した記録。"
created = "2026-09-23"
draft = false
[taxonomies]
tags = ["Fedora 44", "Steam", "NTFS", "Linux", "systemd"]
languages = ["ja"]
+++

## はじめに

先日、[Fedora 44をメイン環境として2ヶ月使った](/posts/fedora-44-after-two-months/)という記事を書いた。

その中で、Steamも普段どおり使えていて、Fedoraだから特別な作業をする感覚はなくなったと書いた。

その数日後。

Steamを起動すると、インストールしていたはずのゲームがすべて「インストール」表示になっていた。

最初はSteam側の不具合かと思った。

しかし、ゲーム本体を置いているSSDを確認すると、そもそもFedoraへマウントされていなかった。

さらに確認すると、ゲーム用SSDだけではなく、別のNTFS HDDも同時にマウントされていなかった。

今回は、この問題を切り分けて復旧し、再起動後も自動マウントされる状態へ戻すまでを記録しておく。

## 私のSteam環境

Fedoraへ移行したとき、Steamゲームをすべて再ダウンロードするのは避けたかった。

そのため、[Linux MintからFedora 44へ移行したときの記事](/posts/linux-mint-to-fedora-44-migration/)にも書いたように、ゲーム本体は既存のNTFS SSDをそのまま使っている。

構成としては大まかに次のようになっている。

```text
/media/ssd
└── SteamLibrary
    └── steamapps
        ├── common
        └── ...
```

Protonの`compatdata`は別のext4領域へ分離しているが、ゲーム本体はNTFS SSDに置いている。

そのため、`/media/ssd`がマウントされなければ、Steamから見るとゲーム本体そのものが存在しない。

今回、Steamでゲームがすべて「インストール」表示になったのは、そのためだった。

## まずSSDが壊れたのか確認した

最初に確認したのは、SSD自体がOSから見えているかどうかだった。

```bash
lsblk -o NAME,MODEL,SIZE,FSTYPE,FSVER,LABEL,UUID,MOUNTPOINTS
```

問題のSSDは正常に認識されていた。

```text
sdc                 Samsung SSD 860 QVO 1TB  931.5G
├─sdc1                                        16M
└─sdc2                                      931.5G ntfs SSD <SSD_UUID>
```

パーティションも残っている。

UUIDも以前と同じ。

ただし、`MOUNTPOINTS`が空だった。

さらに、別のTOSHIBA 1TB HDDも同じ状態だった。

```text
sdb
└─sdb1  931.5G ntfs HDD <HDD_UUID>
```

一方、ext4で使っているSSDは正常にマウントされていた。

```text
/media/p3plus
/media/worktrees
```

つまり、

```text
Fedora
├── btrfs   正常
├── ext4    正常
└── NTFS
    ├── HDD 未マウント
    └── SSD 未マウント
```

という状態だった。

ここで、SSDそのものの故障よりもNTFSのマウント処理を疑うことにした。

## /etc/fstabではntfs3を指定していた

私の`/etc/fstab`では、NTFSのSSDとHDDを起動時に自動マウントするようにしていた。

問題発生時は次のような設定だった。

```fstab
UUID=<SSD_UUID> /media/ssd ntfs3 uid=1000,gid=1000,rw,exec,umask=022,nofail,x-systemd.device-timeout=10s 0 0
UUID=<HDD_UUID> /media/hdd ntfs3 uid=1000,gid=1000,rw,exec,umask=022,nofail,x-systemd.device-timeout=10s 0 0
```

UUIDは`lsblk`で確認した値と一致していた。

つまり、

- ディスクは見えている
- パーティションもある
- UUIDも正しい
- mount pointも存在する
- しかし自動マウントされない

という状態だった。

## 手動マウントするとntfs3が存在しないと言われた

そこで、ゲーム用SSDを手動でマウントしてみた。

```bash
sudo mount -v /media/ssd
```

結果はかなり分かりやすかった。

```text
mount: /media/ssd: unknown filesystem type 'ntfs3'.
       dmesg(1) may have more information after failed mount system call.
```

`ntfs3`というfilesystem type自体を現在のカーネルが認識していない。

この時点で、SteamやSSDの問題ではなく、`fstab`で指定している`ntfs3`と現在のFedoraカーネルの組み合わせに原因がありそうだと分かった。

## 問題発生時の環境

調査時点の環境は次の通りだった。

```text
Fedora Linux 44
kernel 7.2.6-200.fc44.x86_64
udisks2 2.11.2-1.fc44.x86_64
ntfs-3g 2026.2.25-1.fc44.x86_64
```

特に気になったのはkernel 7.2.6だった。

最近Fedoraを更新したあとに発生していたため、カーネル側の変更も確認した。

## Fedora 7.2.6のNTFS変更が関係している可能性が高い

Fedora Packagesのkernel changelogを確認すると、7.2.6には次の変更が入っている。

> Switch to NTFS_FS module for Fedora

[Fedora Packagesのkernel changelog](https://packages.fedoraproject.org/pkgs/kernel/kernel-core/fedora-44-updates.html)で確認できる。

さらに7.2.7のchangelogには、

> Keep the older NTFS3 module around as well for Fedora

という変更が追加されている。

この流れを見ると、Fedora 44の7.2.6でNTFSドライバ構成が変更され、従来使っていた`ntfs3`が私の環境では利用できなくなった可能性が高い。

実際に私の環境では、

```text
kernel 7.2.6
    ↓
fstabでntfs3を指定
    ↓
unknown filesystem type 'ntfs3'
    ↓
SSD / HDDがマウントされない
```

という状態になった。

ただし、ここは「7.2.6のこの変更だけが今回の障害を引き起こした」とまでは断定しない。

私の環境で確認できた事実は、

- 7.2.6を使っていた
- `ntfs3`でのmountが`unknown filesystem type`になった
- Fedoraの7.2.6 changelogにNTFSモジュール変更がある
- 7.2.7では旧NTFS3モジュールを残す変更が入っている

というところまでだ。

タイミングと挙動はかなり一致しているため、今回の原因候補としては非常に強いと考えている。

## 新しいntfsドライバではゲームSSDを読めた

データが残っているか確認するため、まずカーネル側の`ntfs`ドライバを使って手動マウントした。

```bash
sudo modprobe ntfs

sudo mount -i -t ntfs \
  -o uid=1000,gid=1000,rw,exec,umask=022 \
  /dev/sdc2 /media/ssd
```

すると正常にマウントできた。

```bash
findmnt /media/ssd
```

```text
TARGET      SOURCE     FSTYPE
/media/ssd  /dev/sdc2  ntfs
```

中身も残っていた。

```text
SteamLibrary
...
```

SteamLibraryも無事だった。

ここで、少なくともゲームデータそのものが消えたわけではないことを確認できた。

Steamで何十GB、何百GBと再ダウンロードする必要はなさそうだった。

## 恒久対応はntfs-3gへ切り替えた

手動マウントだけでは、再起動するとまた同じ状態になる。

そこで、今回は`fstab`の`ntfs3`固定をやめ、`ntfs-3g`を使う構成へ変更した。

```fstab
UUID=<SSD_UUID> /media/ssd ntfs-3g uid=1000,gid=1000,rw,exec,umask=022,nofail,x-systemd.device-timeout=10s 0 0
UUID=<HDD_UUID> /media/hdd ntfs-3g uid=1000,gid=1000,rw,exec,umask=022,nofail,x-systemd.device-timeout=10s 0 0
```

`exec`を残しているのは、SteamやWine系の利用を考慮しているためだ。

NTFS-3Gの公式ドキュメントにも、起動時に`/etc/fstab`からmountする例が掲載されている。

[NTFS-3G Manual](https://github.com/tuxera/ntfs-3g/wiki/Manual)

変更後は設定を検証した。

```bash
sudo findmnt --verify --verbose
```

結果は、

```text
Success, no errors or warnings detected
```

だった。

## systemdからもfstab由来のmountとして認識された

再起動前にsystemd側も確認した。

```bash
systemctl status media-ssd.mount media-hdd.mount --no-pager
```

SSD、HDDとも、

```text
Loaded: loaded (/etc/fstab; generated)
Active: active (mounted)
```

となった。

さらに、

```bash
systemctl cat media-ssd.mount
systemctl cat media-hdd.mount
```

を確認すると、

```text
# Automatically generated by systemd-fstab-generator
SourcePath=/etc/fstab
```

となっていた。

つまり、手動でたまたまmountできているだけではなく、`/etc/fstab`からsystemdがmount unitを生成できている状態まで確認できた。

## 再起動後もSSDとHDDが自動マウントされた

別の作業が終わったあと、Fedoraを再起動した。

起動後に確認する。

```bash
findmnt /media/ssd
findmnt /media/hdd
```

結果はこちら。

```text
TARGET     SOURCE    FSTYPE  OPTIONS
/media/ssd /dev/sdc2 fuseblk rw,relatime,user_id=0,group_id=0,default_permissions,allow_other,blksize=4096

TARGET     SOURCE    FSTYPE  OPTIONS
/media/hdd /dev/sdb1 fuseblk rw,relatime,user_id=0,group_id=0,default_permissions,allow_other,blksize=4096
```

NTFS-3GでmountしたNTFSは`findmnt`上で`fuseblk`と表示されることがある。

両方とも再起動後に正常にmountされていた。

念のため書き込みも確認した。

```bash
touch /media/ssd/.mount-test
rm /media/ssd/.mount-test

touch /media/hdd/.mount-test
rm /media/hdd/.mount-test
```

こちらも問題なかった。

## Steamを起動するとゲームも元に戻った

最後にSteamを起動した。

最初に見たときは、ゲームがすべて「インストール」表示になっていた。

SSDを復旧してからSteamを起動すると、以前と同じようにインストール済みとして認識された。

再ダウンロードは不要だった。

今回の流れは、

```text
Steamのゲームが「インストール」表示になる
                  ↓
SteamLibraryが見えていない
                  ↓
/media/ssdが未マウント
                  ↓
NTFSのHDDも同時に未マウント
                  ↓
fstabはntfs3を指定
                  ↓
mountすると
unknown filesystem type 'ntfs3'
                  ↓
NTFSのmount方式を変更
                  ↓
再起動後も自動マウント
                  ↓
Steamも元通り
```

というものだった。

Steam側を調べ続けていたら、かなり遠回りしていたと思う。

## 「ゲームが消えた」と思ったらまずmountを確認する

Steamで突然すべてのゲームが未インストールのように見えると、かなり焦る。

しかし、別ディスクへSteamLibraryを置いている場合、最初に確認した方が良いのはSteamそのものではなくmount状態かもしれない。

今回なら、

```bash
lsblk -o NAME,MODEL,SIZE,FSTYPE,LABEL,UUID,MOUNTPOINTS
findmnt /media/ssd
```

だけでもかなり状況が分かった。

ディスク自体が見えているのにmount pointがない場合は、

```bash
sudo mount -v /media/ssd
```

で具体的なエラーを見る。

今回はここで、

```text
unknown filesystem type 'ntfs3'
```

が出たことが決定的だった。

## まとめ

Fedora 44で突然、Steamゲームがすべて「インストール」表示になった。

原因を追っていくと、Steamではなくゲーム用NTFS SSDが`/media/ssd`へマウントされていなかった。

さらにNTFS HDDも同時にマウントされておらず、`fstab`で指定していた`ntfs3`を手動でmountすると、

```text
unknown filesystem type 'ntfs3'
```

となった。

調査時のkernelは`7.2.6-200.fc44.x86_64`。

Fedoraのchangelogには同じ7.2.6でNTFSモジュール構成を変更した記録があり、7.2.7では旧NTFS3モジュールを残す変更も入っている。

そのため、今回の症状とFedoraのカーネル更新には関連がある可能性が高いと考えている。

最終的には`ntfs-3g`を使ってSSDとHDDを`fstab`から自動マウントする構成へ変更した。

再起動後も、

```text
/media/ssd
/media/hdd
```

の両方が正常にマウントされ、Steamもすべてのゲームを元どおり認識した。

ゲームデータは一つも消えていなかった。

Fedoraをメイン環境として使っていると、こういうアップデート由来かもしれないトラブルに遭遇することもある。

ただ、今回も一つずつ状態を確認していけば、原因の場所はかなり絞り込めた。

「Steamがおかしい」と思って始まった問題が、最終的にはLinuxのmountとNTFSドライバの話だった。

こういうところもLinuxをメイン環境で使う面白さの一つだと思う。
