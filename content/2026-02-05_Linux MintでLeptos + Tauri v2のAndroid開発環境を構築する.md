+++
title = "Linux MintでLeptos + Tauri v2のAndroid開発環境を構築する"
slug = "leptos-tauri-v2-android-setup-on-linux-mint"
description = "Linux Mint 22.3でLeptos 0.8 + Tauri v2のAndroidアプリ開発環境を構築し、実機で動作確認するまでの手順を解説。"
created = "2026-02-05"
draft = false
[taxonomies]
tags = ["Rust", "Leptos", "Tauri", "Android", "Linux Mint"]
languages = ["ja"]
+++

## はじめに

RustでAndroidアプリを開発したい。Leptos 0.8とTauri v2の組み合わせでそれが実現できる。

この記事では、Linux Mint 22.3環境でLeptos + Tauri v2のAndroid開発環境を構築し、実機（Pixel 6a）で動作確認するまでの手順をまとめる。

## 環境

- OS: Linux Mint 22.3
- シェル: Zsh
- テストデバイス: Pixel 6a（USB接続） Android 16

## Step 1: Android Studioのインストール

aptには公式パッケージがないため、tar.gzを手動で展開する。

### ダウンロード

[Android Studio公式サイト](https://developer.android.com/studio)からLinux用の`.tar.gz`をダウンロード。

`apt` や `flatpack` 、 `snap` では見つけることが出来なかった。

### 展開・起動

```bash
mkdir -p ~/Android
tar -xzf ~/ダウンロード/android-studio-*.tar.gz -C ~/Android/
~/Android/android-studio/bin/studio.sh
```

### 初回セットアップ

セットアップウィザードで「Standard」を選択。以下が自動インストールされる：
- Android SDK
- Android SDK Platform (API 35)
- Android SDK Platform-Tools
- Android SDK Build-Tools

### SDK Managerで追加インストール

Customize → All settings → Languages & Frameworks → Android SDK → SDK Toolsタブで以下を追加：

- [x] NDK (Side by side)
- [x] Android SDK Command-line Tools (latest)
- [x] CMake

## Step 2: 環境変数の設定

`~/.zshrc`に以下を追加：

```bash
# Android SDK
export JAVA_HOME="$HOME/Android/android-studio/jbr"
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk 2>/dev/null | sort -V | tail -1)"
export PATH="$PATH:$ANDROID_HOME/platform-tools:$ANDROID_HOME/cmdline-tools/latest/bin"
```

反映：

```bash
source ~/.zshrc
```

> もしくはターミナルの再起動。

確認：

```bash
adb --version
sdkmanager --version
echo $NDK_HOME
```

## Step 3: Rustターゲットの追加

Android向けのコンパイルターゲットを追加：

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi \
  i686-linux-android x86_64-linux-android
```

## Step 4: udevルールの設定（Linux特有）

LinuxでAndroidデバイスを認識させるにはudevルールが必要。

```bash
sudo apt install android-sdk-platform-tools-common -y
sudo udevadm control --reload-rules
sudo udevadm trigger
adb kill-server
```

### デバイス接続確認

1. Pixel側で開発者モードを有効化（ビルド番号を7回タップ）
2. USBデバッグをON
3. PCにUSB接続
4. 「USBデバッグを許可しますか？」ダイアログで「許可」

```bash
adb devices
# デバイスIDの後に「device」と表示されればOK
```

## Step 5: Tauri CLIのインストール

```bash
cargo install create-tauri-app tauri-cli trunk
```

## Step 6: プロジェクト作成

```bash
cd /path/to/your/project
cargo create-tauri-app . --template leptos --manager cargo --identifier com.example.myapp --force
```

選択肢を直接指定することで、インタラクティブなプロンプトをスキップできる。

## Step 7: Android初期化・動作確認

### Android プロジェクト初期化

```bash
cargo tauri android init
```

成功すると以下のようなメッセージが表示される：

```
Generating Android Studio project...
victory: Project generated successfully!
Info Using installed NDK: /home/daiki/Android/Sdk/ndk/29.0.14206865
```

### 実機で起動

```bash
cargo tauri android dev
```

初回ビルドは数分かかる。成功すると実機にアプリが表示される。

## 生成されるプロジェクト構造

```
my-app/
├── src/                    # Leptos フロントエンド
│   ├── main.rs
│   └── app.rs
├── src-tauri/              # Tauri バックエンド
│   ├── src/main.rs
│   ├── tauri.conf.json
│   ├── Cargo.toml
│   └── gen/
│       └── android/        # Androidプロジェクト
├── Trunk.toml
├── index.html
└── Cargo.toml
```

## トラブルシューティング

### adb devices でデバイスが表示されない

1. USBケーブルがデータ転送対応か確認（充電専用ケーブルは不可）
2. Pixel側で「USBデバッグを許可」ダイアログを確認
3. USB接続モードが「ファイル転送」になっているか確認
4. udevルールが正しくインストールされているか確認

大前提として、転送などが可能なUSBケーブルを利用する。充電のみ対応のケーブルだと出来ない。

### adb: command not found

新しいターミナルを開くか、`source ~/.zshrc`を実行。

## Leptosのバージョン確認

`Cargo.toml`で`leptos = { version = "0.8" }`と指定すると、Cargoのセマンティックバージョニングにより自動的に最新の0.8.xが取得される。

```bash
cargo tree -p leptos | head -1
# leptos v0.8.15
```

## まとめ

Linux Mint環境でLeptos + Tauri v2のAndroid開発環境を構築できた。

ポイント：
- Android Studioは公式tar.gzを手動展開
- SDK ManagerでNDK、CMake、Command-line Toolsを追加
- Linux特有のudevルール設定を忘れずに
- `cargo tauri android dev`で実機デバッグ

RustでAndroidアプリが動いているのを見ると感動する。
