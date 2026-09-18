+++
title = "Ryzen 9 PRO 8945HSとRadeon 780MでローカルLLMを動かせるか考えた"
slug = "ryzen-9-pro-8945hs-radeon-780m-local-llm"
description = "仕事用Windows PCのタスクマネージャーを見ると、Radeon 780Mに共有GPUメモリ約14.9GBとNPUが表示されていた。RX 9060 XT 16GBと同じようにローカルLLMを動かせるのか整理した。"
created = "2026-09-15"
draft = false
[taxonomies]
tags = ["Ryzen 9 PRO 8945HS", "Radeon 780M", "NPU", "ローカルLLM", "Windows"]
languages = ["ja"]
+++

## はじめに

仕事用Windows PCのタスクマネージャーを見ていたところ、GPUの欄に気になる数字が表示されていた。

- 専用GPUメモリ 約2GB
- 共有GPUメモリ 約14.9GB

さらにNPUという項目もある。

自宅PCにはRadeon RX 9060 XT 16GBを搭載しており、[gpt-oss-20bをローカルで使い始めている](/posts/rx-9060-xt-16gb-gpt-oss-20b/)。

そこで、

> 共有GPUメモリが約15GBあるなら、仕事PCも16GB GPUと同じくらいローカルLLMを動かせるのでは

と思った。

CPUはRyzen 9 PRO 8945HS、内蔵GPUはRadeon 780Mだった。

## Ryzen 9 PRO 8945HSの構成

AMD公式によるとRyzen 9 PRO 8945HSは、

- 8コア / 16スレッド
- Radeon 780M
- 12 GPUコア
- 最大16 TOPSのNPU
- 最大39 TOPSのシステム全体AI性能

という構成になっている。

[AMD Ryzen 9 PRO 8945HS](https://www.amd.com/en/products/processors/laptop/ryzen-pro/8000-series/amd-ryzen-9-pro-8945hs.html)

CPU、GPU、NPUが一つのSoCに入っている。

タスクマネージャーにNPU欄があったのは、このRyzen AI NPUが搭載されているためだった。

## 共有GPUメモリはVRAMとは違う

一番気になったのが共有GPUメモリだった。

約14.9GBという数字だけを見ると、自宅のRX 9060 XT 16GBとかなり近い。

しかし中身はまったく違う。

RX 9060 XTには16GBのGDDR6がGPU専用メモリとして搭載されている。

一方、Radeon 780Mは内蔵GPUなので、必要に応じてPCのシステムRAMをGPUと共有する。

```text
RX 9060 XT
GPU
└─ 専用GDDR6 16GB

Radeon 780M
CPU / GPU
└─ システムRAMを共有
```

タスクマネージャーに14.9GBと表示されていても、14.9GBの高速な専用VRAMが載っているわけではない。

## メモリ帯域がかなり違う

LLMでは容量だけでなく、モデルの重みをどれだけ速く読み出せるかも重要になる。

RX 9060 XT 16GBはAMD公式で最大320GB/sのメモリ帯域幅を持つ。

Radeon 780MはシステムRAMをCPUと共有するため、専用GPUのGDDR6とは条件が違う。

そのため、

> どちらも約16GB使えるから同じ速度

とは考えられない。

モデルが入る可能性と、実用的な速度で動くかは別だ。

## NPUがあるならLLMを速くできるのか

次に気になったのがNPUだった。

Ryzen 9 PRO 8945HSのNPUは最大16 TOPS。

AI専用の処理領域なので、

> OllamaのモデルもNPUで高速に動くのでは

と思いたくなる。

ただしNPUは、存在するだけで一般的なLLMランタイムが自動的に利用してくれるものではない。

実際に利用するには、ソフトウェア側がそのNPU向けの実行経路を持っている必要がある。

現在のローカルLLM環境を考えると、まずCPUやGPU側の対応を見る方が分かりやすい。

NPUのTOPSだけを見て、RX 9060 XTの代わりになるとは考えない方が良い。

## 仕事PCでも小さなモデルなら面白い

だからといって、Ryzen 9 PRO 8945HSがローカルLLMに使えないわけではない。

8コア16スレッドのCPUとRadeon 780M、十分なシステムRAMがある。

小さめの量子化モデルを動かしたり、軽い推論を試したりする用途なら面白い。

特に、

> 専用GPUなしのミニPCでどこまでできるか

という実験としてはかなり興味がある。

ただ、自宅のRX 9060 XT 16GBと同じクラスだと思って使うものではない。

## NPUは今後のソフトウェア対応に期待したい

NPUがタスクマネージャーに表示されているのを見ると、せっかくあるのだから使いたくなる。

現時点では自分のローカルLLM用途で中心になるのはCPU/GPUだが、今後NPU対応のソフトウェアが増えれば役割が変わる可能性がある。

AI PCという名前のハードウェアが増えているので、NPUをどのアプリが実際に使えるかは今後も追いたい。

## まとめ

仕事用PCの共有GPUメモリ約14.9GBを見て、RX 9060 XT 16GBと同じようにローカルLLMを動かせるのか考えた。

結論として、数字だけを比較することはできない。

```text
RX 9060 XT 16GB
  → GPU専用GDDR6
  → 最大320GB/s

Radeon 780M
  → システムRAM共有
  → CPUとも帯域を共有

Ryzen AI NPU
  → 最大16 TOPS
  → 対応ソフトウェアが必要
```

という違いがある。

共有GPUメモリが大きく見えても、専用VRAMと同じものではない。

それでも、専用GPUのないミニPCでローカルLLMを試す環境としては面白いので、機会があれば実際の速度も確認してみたい。
