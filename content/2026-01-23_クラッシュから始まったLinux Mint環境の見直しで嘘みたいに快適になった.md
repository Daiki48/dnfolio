+++
title = "クラッシュから始まったLinux Mint環境の見直しで嘘みたいに快適になった"
slug = "linux-mint-nvidia-prime-remove"
description = "Linux Mint 22.3に最近アップグレードしたのだが、そこから頻発するようになった突然発生するフリーズ体験。なかなか不快なので原因の調査を進めて、いくつか設定を見直したら嘘みたいに快適になった。"
created = "2026-01-23"
draft = false
[taxonomies]
tags = ["Linux Mint", "NVIDIA", "トラブルシューティング"]
languages = ["ja"]
+++

Linux Mint 22.3 + NVIDIA RTX 3060 Ti環境で、開発中に頻繁にフリーズする問題に悩まされていた。結論から言うと、シングルGPU環境には不要な`nvidia-prime`を削除し、カーネルパラメータを調整することで解決した。

## 環境

- OS: Linux Mint 22.3 Zena
- CPU: AMD Ryzen 5 5600X（iGPU **なし**）
- GPU: NVIDIA GeForce RTX 3060 Ti
- マザーボード: MSI MPG X570 GAMING PLUS
- RAM: 32GB
- ドライバ: nvidia 570.211.01
- カーネル: 6.8.0-90-generic（LTSカーネル、[アップグレードでは自動変更されない](https://blog.linuxmint.com/?p=4980)）

普段の作業環境はAlacritty + tmux + Neovim。Dockerを立ち上げながらRustでの開発やdotfilesの盆栽をしている。

## 症状

開発作業中に突然システムがフリーズする。マウスもキーボードも反応せず(厳密にはドット単位で動いているように見えるが)、電源長押しで強制シャットダウンするしかない。

興味深いのは、**ゲーム中には一度もフリーズしない**こと。GPU負荷が高い時は安定していて、Neovimでコードを書いている低負荷時にクラッシュする。

## 原因調査

### nvidia-primeの存在

まず`prime-select query`を実行してみた。

```bash
$ prime-select query
on-demand
```

`on-demand`モードになっている。これはNVIDIA Optimus（Intel iGPU + NVIDIA dGPUのハイブリッド構成）向けの設定だ。

でも、Ryzen 5 5600Xには**iGPUがない**。RTX 3060 Tiしか搭載していないシングルGPU構成なのに、なぜかnvidia-primeがインストールされていて、意味のないon-demandモードで動作していた。

### IRQエラー

`journalctl -b -p err`でログを確認すると、こんなエラーが大量に出ていた。

```
kernel: __common_interrupt: X.55 No irq handler for vector
```

X570チップセット + NVIDIAの組み合わせでは、[PCIeのエラーレポーティング（AER）周りで問題が起きやすい](https://forums.linuxmint.com/viewtopic.php?t=440122)らしい。

## 解決方法

### 1. nvidia-primeの削除

実際にインストールされているか確認する。

```bash
$ dpkg -l | grep nvidia-prime
ii  nvidia-prime        ...
ii  nvidia-prime-applet ...
```

やはり入っている。nvidia-primeはiGPU + dGPUのハイブリッド構成でGPU切り替えを管理するためのパッケージであり、シングルGPU環境には不要だ。

削除して大丈夫なのかは正直不安だった。「これを消したらGPU使えなくなるのでは？」と思ったが、GPUを実際に動かしているのは`nvidia-driver-570`（NVIDIAドライバ本体）であり、nvidia-primeはあくまでGPU切り替えの管理ツールに過ぎない。切り替える対象のiGPUがないのだから、削除しても何も失われない。

```bash
sudo apt remove nvidia-prime nvidia-prime-applet
```

削除後も`nvidia-smi`でGPUが正常に認識されていることを確認できる。

### 2. カーネルパラメータの追加

`/etc/default/grub`を編集。

```bash
GRUB_CMDLINE_LINUX_DEFAULT="nvidia_drm.fbdev=1 nvidia_drm.modeset=1 pci=noaer quiet splash"
```

各パラメータの役割：

| パラメータ | 効果 |
|-----------|------|
| `nvidia_drm.modeset=1` | [Kernel Mode Setting](https://wiki.archlinux.org/title/NVIDIA#DRM_kernel_mode_setting)を有効化、GPU初期化を最適化 |
| `nvidia_drm.fbdev=1` | NVIDIAの[DRMフレームバッファデバイス](https://forums.developer.nvidia.com/t/drm-fbdev-wayland-presentation-support-with-linux-kernel-6-11-and-above/307920)を有効化し、汎用simpledrm を置き換える |
| `pci=noaer` | [PCIe Advanced Error Reporting](https://docs.kernel.org/PCI/pcieaer-howto.html)を無効化、X570で頻発するエラー通知を抑止 |

### 3. 設定の適用

```bash
sudo update-grub
sudo reboot
```

## 結果

再起動後、体感でわかるレベルで改善した。

- タイピング時のレスポンスが明らかに良くなった
- マウス移動がスムーズになった
- これまで感じていた微妙な引っかかりがなくなった

nvidia-primeを削除したことで、不要な電力管理のオーバーヘッドがなくなったのだと思う。on-demandモードは「必要な時だけGPUを使う」ための仕組みだが、シングルGPU環境では常にNVIDIA GPUを使っているわけで、その状態監視・遷移判断が無駄に走っていたことになる。

## なぜ低負荷時にクラッシュしていたのか

GPUの電力状態（[P-State](https://docs.nvidia.com/gameworks/content/gameworkslibrary/coresdk/nvapi/group__gpupstate.html)）の問題だったと推測している。nvidia-primeのon-demandモードが不要な電力状態管理を行っていたことで、以下のような遷移失敗が起きていたのではないかと考えている。

```
開発作業中 (P8: 省電力状態)
    ↓
Alacritty再描画 / カーソル点滅 / tmux更新
    ↓
一瞬だけGPUが起きる (P8 → P5 → P8)
    ↓
この微小な遷移が繰り返される
    ↓
どこかで遷移に失敗 → フリーズ
```

ゲーム中はGPUがP0（フルパワー）で動作し続けるから、電力状態の遷移が起きない。だから安定していた。

## まとめ

- シングルGPU環境に`nvidia-prime`は不要
- `prime-select query`が`on-demand`を返していたら要注意
- X570 + NVIDIAでPCIeエラーが出るなら`pci=noaer`でAERを無効化すると改善するかも
- `nvidia_drm.modeset=1`と`nvidia_drm.fbdev=1`で入力遅延が改善する可能性あり

Linux Mintを選んだのは安定性を期待してのことだったが、パッケージの自動インストールで意図しない設定が入ることもあるんだなと学んだ。自分の環境に本当に必要なものは何かを把握しておくことが大事。

しばらく様子を見て、まだフリーズが起きるようなら[`nvidia-persistenced`](https://docs.nvidia.com/deploy/driver-persistence/index.html)でPersistence Modeを有効にすることも検討する。GPUドライバを常にロード状態に保つことで、電力状態の不要な遷移を減らせる。

とりあえず今は快適。開発に集中できる環境が戻ってきた。
