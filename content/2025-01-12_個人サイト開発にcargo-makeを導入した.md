+++
title = "個人サイト開発にcargo-makeを導入した"
slug = "cargo-make-introduction"
description = "ここ数日、個人サイト開発に熱中している。複数のファイル形式で構築しているからコードの整形をする際に簡単に実行出来るタスクランナーを実行したくなる。deno tasksでなんとかやっていたが、複雑なタスクランナーとなると大変だったので初めてcargo-makeを試してみた。"
draft = false
[taxonomies]
tags = ["cargo-make"]
languages = ["ja"]
+++

## Denoでタスクランナーの導入

[cargo-make](https://github.com/sagiegurari/cargo-make) 導入前は、[Deno](https://deno.com/) の[deno task](https://docs.deno.com/runtime/reference/cli/task/) でやっていた。
こんな感じで。

```json
{
  "version": "4.7.0",
  "tasks": {
    "dev": "deno task dprint fmt && djlint . --extension=html --reformat && zola serve",
    "build": "zola build",
    "fmt": "deno task dprint fmt && djlint . --extension=html --reformat",
    "fmt:check": "deno task dprint check && djlint . --extension=html --check",
    "dprint": "deno run -A npm:dprint"
  }
}
```

`version` はなんちゃってバージョン。モチベーションのために設定していたが、GitHubのタグにも登録しているので設定する必要は無い。
次の `tasks` でタスクランナーを設定していた。

ローカルの開発サーバーを起動するためのコマンドは

```sh
deno task dev
```

ビルドするときは

```sh
deno task build
```

`dprint` と `djlint` を利用してプロジェクト内のファイルを全てフォーマットする場合は

```sh
deno task fmt
```

フォーマットで書き込みするのではなく、フォーマット箇所の確認では

```sh
deno task fmt:check
```

`dprint` を `fmt` で利用する際の `deno run -A npm:dprint` を

```sh
deno task dprint
```

に設定していた。こうすると `node_modules` が生成されない。

これでも良かったが、 `Zola` というRust製SSGを利用しているのでRust系でタスクランナーがあれば試してみたいなぁと。

## `cargo-make` を発見

`cargo-make` というタスクランナーを見つけた。

{{ card(title="cargo-make | GitHub", url="https://github.com/sagiegurari/cargo-make") }}

個人的なメリット

- `TOML` 管理なのでコメント可能。 `JSON` 管理の `deno.json` はコメント出来ないので私のようにコメント見たい人間にはちょっと不便に感じる。
- 結構複雑なタスクランナーも出来た。便利かも。
- ログレベルの設定まで出来るのでデバッグ助かる。[CLI Options | README cargo-make](https://github.com/sagiegurari/cargo-make?tab=readme-ov-file#cli-options)

## `cargo-make` を導入

まずは現在利用中の `deno.json` を全て再現出来ることが重要なので書き直した。

`cargo-make` をインストール。バイナリでインストールすることも出来るが `Cargo` でインストールした。

```sh
cargo install --force cargo-make
```

{{ card(title="Installation | cargo-make GitHub", url="https://github.com/sagiegurari/cargo-make?tab=readme-ov-file#installation") }}

プロジェクトのルートに `Makefile.toml` を作成する。

### ローカルサーバーの起動

これはシンプル。

```toml
[tasks.dev]
command = "zola"
args = ["serve"]
```

`cargo-make` をインストールすると `makers` コマンドを利用出来る。

```sh
makers --help
cargo-make 0.37.23
Sagie Gur-Ari <sagiegurari@gmail.com>
Rust task runner and build tool.

USAGE:
    [makers | cargo make | cargo-make make] [OPTIONS] [--] [<TASK_CMD>...]

ARGS:
    <TASK_CMD>    The task to execute, potentially including arguments which can be accessed in the task itself.

OPTIONS:
    --help, -h                           Print help information
    --version, -V                        Print version information
    --makefile <FILE>                    The optional toml file containing the tasks definitions
    --task, -t <TASK>                    The task name to execute (can omit the flag if the task name is the last argument) [default: default]
    --profile, -p <PROFILE>              The profile name (will be converted to lower case) [default: development]
    --cwd <DIRECTORY>                    Will set the current working directory. The search for the makefile will be from this directory if defined.
    --no-workspace                       Disable workspace support (tasks are triggered on workspace and not on members)
    --no-on-error                        Disable on error flow even if defined in config sections
    --allow-private                      Allow invocation of private tasks
    --skip-init-end-tasks                If set, init and end tasks are skipped
    --skip-tasks <SKIP_TASK_PATTERNS>    Skip all tasks that match the provided regex (example: pre.*|post.*)
    --env-file <FILE>                    Set environment variables from provided file
    --env, -e <ENV>                      Set environment variables
    --loglevel, -l <LOG LEVEL>           The log level (verbose, info, error, off) [default: info]
    --verbose, -v                        Sets the log level to verbose (shorthand for --loglevel verbose)
    --quiet                              Sets the log level to error (shorthand for --loglevel error)
    --silent                             Sets the log level to off (shorthand for --loglevel off)
    --no-color                           Disables colorful output
    --time-summary                       Print task level time summary at end of flow
    --experimental                       Allows access unsupported experimental predefined tasks.
    --disable-check-for-updates          Disables the update check during startup
    --output-format <OUTPUT FORMAT>      The print/list steps format (some operations do not support all formats) (default, short-description, markdown, markdown-single-page, markdown-sub-section, autocomplete)
    --output-file <OUTPUT_FILE>          The list steps output file name
    --hide-uninteresting                 Hide any minor tasks such as pre/post hooks.
    --print-steps                        Only prints the steps of the build in the order they will be invoked but without invoking them
    --list-all-steps                     Lists all known steps
    --list-category-steps <CATEGORY>     List steps for a given category
    --diff-steps                         Runs diff between custom flow and prebuilt flow (requires git)

See more info at: https://github.com/sagiegurari/cargo-make
```

`makers` コマンドを使用してローカルサーバーの起動

```sh
makers dev
```

同じような要領でビルド、フォーマット、フォーマット箇所の確認を実装した。

ビルドはこれ。

```toml
[tasks.build]
command = "zola"
args = ["build"]
```

フォーマット箇所の確認とフォーマットの実行は `dprint` と `djlint` それぞれを実行するため、こう書いた。

```toml
############################################
# Check
############################################
[tasks.check]
dependencies = ["check_dprint", "check_djlint"]

############################################
# Format
############################################
[tasks.fmt]
dependencies = ["fmt_dprint", "fmt_djlint"]

############################################
# https://dprint.dev
############################################
[tasks.install_dprint]
command = "cargo"
args = ["install", "dprint", "--locked"]

[tasks.fmt_dprint]
dependencies = ["install_dprint"]
command = "dprint"
args = ["--config", "dprint.json", "fmt"]

[tasks.check_dprint]
dependencies = ["install_dprint"]
command = "dprint"
args = ["--config", "dprint.json", "check"]

############################################
# https://djlint.com
############################################
[tasks.fmt_djlint]
command = "djlint"
args = [".", "--reformat"]

[tasks.check_djlint]
command = "djlint"
args = [".", "--check"]
```

これでフォーマット箇所の確認時は

```sh
makers check
```

フォーマット実行時は

```sh
makers fmt
```

個人的にかなり見やすくなった。

## 今後 `Rust` コードを書くために

更に `Makefile.toml` を編集しておいた。これは随時更新していくつもり。

```toml
############################################
# Starting local server
############################################
[tasks.dev]
dependencies = ["check", "fmt", "test"]
command = "zola"
args = ["serve"]

############################################
# Build
############################################
[tasks.build]
dependencies = ["check", "fmt", "test"]
command = "zola"
args = ["build"]

############################################
# Check
############################################
[tasks.check]
dependencies = ["check_dprint", "check_djlint", "clippy_cargo"]

############################################
# Format
############################################
[tasks.fmt]
dependencies = ["fmt_dprint", "fmt_djlint", "fmt_cargo"]

############################################
# Test
############################################
[tasks.test]
dependencies = ["test_cargo"]

############################################
# Cargo
############################################
[tasks.clean_cargo]
command = "cargo"
args = ["clean"]

[tasks.build_cargo]
command = "cargo"
args = ["build"]

[tasks.test_cargo]
command = "cargo"
args = ["test"]

[tasks.fmt_cargo]
install_crate = "rustfmt"
command = "cargo"
args = ["fmt"]

[tasks.clippy_cargo]
install_crate = "clippy"
command = "cargo"
args = ["clippy"]

############################################
# https://dprint.dev
############################################
[tasks.install_dprint]
command = "cargo"
args = ["install", "dprint", "--locked"]

[tasks.fmt_dprint]
dependencies = ["install_dprint"]
command = "dprint"
args = ["--config", "dprint.json", "fmt"]

[tasks.check_dprint]
dependencies = ["install_dprint"]
command = "dprint"
args = ["--config", "dprint.json", "check"]

############################################
# https://djlint.com
############################################
[tasks.fmt_djlint]
command = "djlint"
args = [".", "--reformat"]

[tasks.check_djlint]
command = "djlint"
args = [".", "--check"]
```

探し求めていたタスクランナーだったので、現時点では満足している。
例えば `Rust` のみのプロジェクトであれば `rustfmt` で満足していたし、 `Node.js` プロジェクトだと `Biome` や `Prettier` 、`ESLint` などを利用したことがある。
`Deno` であれば `deno fmt` だったり。
今回は個人サイトでいろいろなファイルを扱ってみるので `cargo-make` がちょうど良かった。
