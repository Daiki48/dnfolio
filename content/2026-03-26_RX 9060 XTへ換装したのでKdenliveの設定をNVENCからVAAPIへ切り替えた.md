+++
title = "RX 9060 XTへ換装したのでKdenliveの設定をNVENCからVAAPIへ切り替えた"
slug = "kdenlive-nvenc-to-vaapi-after-rx9060xt"
description = "RTX 3060 TiからRX 9060 XTへGPUを換装したことで、Kdenliveのハードウェアエンコード設定もNVENCからVAAPIへ切り替えた。レンダリング設定とトランスコード設定の変更点、NVENCに警告が出る理由、VAAPI H264で不安定な場合の代替手段をまとめる。"
created = "2026-03-26"
draft = false
[taxonomies]
tags = ["Kdenlive", "AMD", "RX 9060 XT", "Linux Mint", "動画編集"]
languages = ["ja"]
+++

先日、[MSI X570マザーボードのBIOSをアップデートした](/posts/msi-x570-bios-update/) あとに [RTX 3060 TiからRX 9060 XTへGPUを換装した](/posts/gpu-swap-rtx3060ti-to-rx9060xt/)。
ついでに [M.2 SSDを増設してパーティション作成と自動マウントを設定した](/posts/m2-ssd-add-partition-automount/) ので、ハードウェア周りはかなり入れ替わった。

そこで最後に見直したのが、普段使っているKdenliveの設定だった。
これまでの環境ではNVIDIA GPU向けの `NVENC` を使っていたが、AMD GPUへ変わったことで、そのままでは噛み合わなくなった。
今回はKdenlive側で実際に変更した内容だけをまとめておく。

以前のNVIDIA向け設定は [Kdenlive v25とNVIDIA GPUで高画質動画編集したい](/posts/kdenlive-v25-nvidia-gpu-movie-edit/) にまとめているので、そちらの続きとして読むと流れが分かりやすいはず。
この頃はRTX 3060 TiなのでNVidiaドライバも入れたり別で準備が必要。

## 結論

変更したのは大きく2か所だけだった。

| 項目 | 変更前 | 変更後 |
|---|---|---|
| レンダリング | NVENC系プロファイル | **VAAPI H264** |
| トランスコード | Lossy x264 I frame only (NVidia GPU) | **Lossy x264 I frame only (VAAPI GPU)** |

要するに、NVIDIA専用の `NVENC` を使っていた部分を、LinuxでAMD GPUが扱いやすい `VAAPI` ベースへ置き換えた形になる。

## なぜ変更が必要だったのか

`NVENC` はNVIDIA専用のハードウェアエンコード機能なので、GPUがAMDに変わるとそのままでは使えない。
Kdenlive上にプロファイル自体は残っていても、実際には利用できず、警告マーク付きで表示される状態になっていた。

これは設定が壊れているわけではなく、単に「このGPUではそのプロファイルを使えない」という意味の正常な表示だった。
なので、古いNVENCプロファイルを無理に触るより、AMD環境向けのプロファイルへ切り替えてしまう方が分かりやすい。

> そもそもNVIlDIA GPUが挿さっていないので選択すらできなくなってるはず。

## Kdenliveで変更した内容

### レンダリング設定

レンダリング時に使っていた `NVENC H264 VBR` や `NVENC H265 ABR` などのNVIDIA向けプロファイルは、今回の環境では使えなくなった。
ここは素直に `VAAPI H264` を使う形へ切り替えた。

以前の環境ではYouTube向けに `NVENC H264 VBR` をよく使っていたが、AMD GPUへ移行した今はその役割を `VAAPI H264` に任せる形になる。

### トランスコード設定

編集前の変換に使っていた `Lossy x264 I frame only (NVidia GPU)` も、同じくNVIDIA専用だった。
こちらは `Lossy x264 I frame only (VAAPI GPU)` へ変更した。

I frame only のプロファイルを使っておくと、シークしやすくて編集も軽くなるので、この方向性自体はそのままで問題なかった。
変わったのは、GPUバックエンドが `NVENC` から `VAAPI` になった部分だけだ。

## NVENCに警告が出ていても慌てなくてよかった

GPU換装後のKdenliveでは、`NVENC AV1 VBR`、`NVENC H264 ABR/VBR`、`NVENC H265 ABR` などに警告マークが付いていた。
最初は少し気になるが、NVIDIA GPUが見つからない以上、この表示は自然な挙動になる。

むしろ分かりやすいのは、

- NVIDIA向け: `NVENC`
- AMD向け: `VAAPI`

と頭の中で切り分けてしまうことだった。
今回の環境では `NVENC` を見る必要はほぼなくなったので、使うプロファイルを `VAAPI` 系に寄せておけば混乱しにくい。

## VAAPI H264で不安定な場合

RDNA系のAMD GPUでは、環境によって `VAAPI H264` で不安定さやクラッシュが出ることがあるらしい。
もし挙動が怪しい場合は、以下を代替候補として見ておくとよさそうだった。

### ソフトウェアエンコーディングへ切り替える

`libx264` ベースのソフトウェアエンコードにすれば、速度は落ちるものの動作は安定しやすい。
ハードウェアエンコードが怪しい時の逃げ道としては一番分かりやすい。
これはRX 9060 XTへの換装関係無く、RTX 3060 Tiの頃からフォールバックとして稀に利用してた。

### VAAPI AV1を試す

RX 9060 XTはAV1ハードウェアエンコードに対応しているので、`VAAPI H264` ではなく `VAAPI AV1` なら問題を避けられる可能性がある。
ファイルサイズや対応環境との兼ね合いはあるが、候補として覚えておく価値はありそうだった。

## まとめ

今回のKdenlive側の対応は、やってみるとかなりシンプルだった。
GPU換装によって使うAPIが `NVENC` から `VAAPI` に変わったので、レンダリング設定とトランスコード設定をそれに合わせて入れ替えただけである。

ハードウェア周りの変更は [BIOSアップデート](/posts/msi-x570-bios-update/)、[GPU換装](/posts/gpu-swap-rtx3060ti-to-rx9060xt/)、[SSD増設](/posts/m2-ssd-add-partition-automount/) で一区切りついていたが、普段使うアプリ側まで揃えてようやく移行完了という感じになった。
Kdenliveを日常的に使っているなら、この設定変更もセットで見ておいた方が後から戸惑わずに済みそうだ。
