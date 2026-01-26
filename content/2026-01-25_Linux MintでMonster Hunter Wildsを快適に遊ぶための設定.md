+++
title = "Linux MintでMonster Hunter Wildsを快適に遊ぶための設定"
slug = "linux-mint-monster-hunter-wilds-settings"
description = "Linux Mint 22.3 + NVIDIA RTX 3060 Ti環境でMonster Hunter Wildsの描画バグに悩まされていたが、Proton HotfixとVKD3Dの環境変数、ゲーム内設定の調整で安定動作するようになった。私の環境での設定を共有する。"
created = "2026-01-25"
draft = false
[taxonomies]
tags = ["Linux Mint", "NVIDIA", "Steam", "Monster Hunter Wilds", "Proton"]
languages = ["ja"]
+++

[前回の記事](/posts/linux-mint-nvidia-prime-remove)で、Linux Mint環境のフリーズ問題を解決するために`nvidia-prime`の削除とカーネルパラメータの調整を行った。開発作業は快適になったのだが、今度はSteamでゲームを起動すると描画が崩壊するという新たな問題が発生した。

この記事では、私の環境でMonster Hunter Wildsを安定して動作させるために行った設定を共有する。

## 注意

環境によって原因や解決策は異なる可能性がある。この記事の内容は「私の環境ではこの設定で動作した」という一例であり、すべての環境で同じ結果になるとは限らない。

## 環境

- OS: Linux Mint 22.3 Zena
- CPU: AMD Ryzen 5 5600X
- GPU: NVIDIA GeForce RTX 3060 Ti
- RAM: 32GB
- ドライバ: nvidia 570.211.01
- カーネル: 6.8.0-90-generic

## 発生した問題

Monster Hunter WildsとOverwatch 2で画面の描画が激しく崩壊し、ゲームプレイが不可能な状態になった。いわゆる「vertex explosions（頂点爆発）」と呼ばれる現象で、テクスチャやメッシュが破綻して画面全体がバグったような表示になる。

調査の結果、これはNVIDIA GPU + Proton環境で広く報告されている[既知の問題](https://github.com/ValveSoftware/Proton/issues/8206)だった。

## 解決方法

### 1. Proton Hotfixを使用する

Steamライブラリでゲームを右クリック → プロパティ → 互換性 → 「特定のSteam Play互換ツールの使用を強制する」にチェック → **Proton Hotfix**を選択。

ValveはMonster Hunter Wilds向けに[Proton Hotfixを更新](https://www.gamingonlinux.com/2025/04/proton-hotfix-updated-to-fix-monster-hunter-wilds-on-linux-steam-deck/)しており、これがデフォルトの互換ツールとして設定されている。Proton Experimentalではなく、Proton Hotfixを使用することが重要。

### 2. 起動オプションに環境変数を追加

Steamライブラリでゲームを右クリック → プロパティ → 一般 → 起動オプションに以下を入力：

```
VKD3D_DISABLE_EXTENSIONS=VK_NV_low_latency2 %command%
```

`VK_NV_low_latency2`はNVIDIA Reflexのための[Vulkan拡張](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_low_latency2.html)だが、VKD3D-Protonとの組み合わせで描画バグを引き起こすことがある。この環境変数で無効化することで問題を回避できる。

### 3. ゲーム内グラフィック設定

以下は私の環境で安定動作した設定。重い処理を避けつつ、DLSSで画質を補っている。

#### 基本設定

| 設定項目 | 値 |
|----------|-----|
| グラフィックプリセット | カスタム |
| カットシーン用プリセット | 設定なし |
| アップスケーリング（超解像技術） | NVIDIA DLSS |
| フレーム生成 | OFF |
| アップスケーリングモード | バランス |
| テクスチャ品質 | 最低 |
| テクスチャフィルタリング品質 | 最低（Bilinear） |
| メッシュ品質 | 中 |
| 毛皮の描画品質 | 低 |

![基本設定](/content/mhwilds-config-for-linux-mint/mhwild-config1.png)

#### 環境描画設定

| 設定項目 | 値 |
|----------|-----|
| 空・雲の描画品質 | 低 |
| 草木の描画品質 | 低 |
| 草木の揺れ設定 | OFF |
| 風の流体シミュレーション品質 | 低 |
| 地面の描画品質 | 低 |
| 砂・雪などの描画品質 | 低 |
| 水の流体エフェクトの設定 | 無効 |
| 画面の揺らぎ表現の設定 | 無効 |
| カリング距離の設定 | 高 |
| 影の描画品質 | 低 |
| 遠景の影の描画品質 | 低 |
| 物体の影を表示する距離範囲 | 近 |
| 環境光の描画品質 | 中 |
| コンタクトシャドウ | OFF |

![環境描画設定](/content/mhwilds-config-for-linux-mint/mhwild-config2.png)

#### ポストエフェクト設定

| 設定項目 | 値 |
|----------|-----|
| アンビエントオクルージョン | 中 |
| ブルーム | 高 |
| モーションブラー | OFF |
| ビネット効果 | ON |
| スクリーンスペースリフレクション | ON |
| SSSSスキャッタリング | OFF |
| 被写界深度 | OFF |
| ボリュームフォグ | 低 |
| 可変レートシェーディング | バランス |

![ポストエフェクト設定](/content/mhwilds-config-for-linux-mint/mhwild-config3.png)

## 結果

上記の設定で2時間以上プレイしても描画バグは発生しなかった。DLSSのおかげで画質もそこまで悪くなく、快適にプレイできている。

## まとめ

- **Proton Hotfix**を使用する（Proton Experimentalではない）
- **`VKD3D_DISABLE_EXTENSIONS=VK_NV_low_latency2`**を起動オプションに追加
- グラフィック設定は控えめにして、DLSSで補う

繰り返しになるが、これは私の環境での結果であり、すべての環境で同じ結果になるとは限らない。Linux環境でのゲームプレイはまだまだ試行錯誤が必要だが、少しでも参考になれば幸いだ。
