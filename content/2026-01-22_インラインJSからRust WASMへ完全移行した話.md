+++
title = "インラインJSからRust WASMへ完全移行した話"
slug = "inline-js-to-rust-wasm-migration"
description = "MaudテンプレートにベタベタのインラインJavaScriptで書いていたVim風UIを、Rust + WebAssemblyで完全に書き直した。「100% Rust」を名乗るための長い道のり。"
created = "2026-01-22"
draft = false
[taxonomies]
tags = ["Rust", "WASM", "wasm-bindgen", "Security"]
languages = ["ja"]
+++

## はじめに

先日、[大好きなRustのみで個人サイトを構築した話](/posts/rust-only-personal-website/)という記事を書いた。

その中で「Rust100%」と豪語していたのだけど、実は嘘があった。ブラウザ側のインタラクション（カーソル移動、検索モーダル、トースト通知など）は全部JavaScriptで書いていたのだ。しかも、Maudテンプレートの中に直接埋め込む形で。

約2,000行のインラインJavaScript。これを「100% Rust」と呼ぶのは詐欺だろう。

というわけで、重い腰を上げてRust + WebAssemblyへの移行を決意した。これはその記録である。


## なぜWASMなのか

「JavaScriptでちゃんと動いてるなら、そのままでいいじゃん」

その通りである。機能的には何の問題もなかった。

でも、私は「Rust100%の個人サイト」を作りたかったのだ。Aboutページに「ロジックは100% Rustで書いています」と書いておきながら、裏ではJavaScriptが動いているなんて気持ち悪いじゃないか。

それに、WebAssemblyをちゃんと触ったことがなかった。良い機会だ。


## 移行前のカオスな状態

まず、移行前のコードがどんな状態だったかを見せたい。

`src/templates/base.rs`の中に、こんな感じでJavaScriptが埋め込まれていた。

```rust
html! {
    // ...HTMLテンプレート...

    script {
        (PreEscaped(r#"
            // 2000行のJavaScriptがここに...

            class VimModal {
                constructor() {
                    this.mode = 'NORMAL';
                    this.lastKey = null;
                    // ...
                }

                handleKeydown(e) {
                    if (this.mode === 'NORMAL') {
                        switch (e.key) {
                            case 'h': this.moveLeft(); break;
                            case 'j': this.moveDown(); break;
                            case 'k': this.moveUp(); break;
                            case 'l': this.moveRight(); break;
                            // ...100行以上のswitch文...
                        }
                    }
                }
            }
        "#))
    }
}
```

これがMaudテンプレートの一部だ。HTMLとCSSとJavaScriptが混在して、カオスの極み。

エディタのシンタックスハイライトも効かないし、LSPの補完も効かない。文字列リテラルの中に書いているから当然だ。タイポしてもコンパイル時には気づけない。


## 新しいプロジェクト構造

移行後の構造はこうなった。

```
dnfolio/
├── Cargo.toml                    # ワークスペース定義
├── crates/
│   ├── dnfolio-ssg/              # 既存SSG（Maud + pulldown-cmark）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── build.rs
│   │       └── templates/
│   │           └── base.rs       # JS削除済み、WASMロードのみ
│   │
│   └── dnfolio-wasm/             # 新規WASMクレート
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs            # エントリポイント
│           ├── error.rs          # 共通エラー型
│           ├── dom/              # DOM操作ヘルパー
│           ├── vim/              # Vim機能（hjkl移動、コマンド）
│           ├── search/           # 検索・ハイライト
│           ├── ui/               # トースト、ステータスライン
│           └── events.rs         # イベントハンドラ
```

ワークスペース構成にして、SSGとWASMを分離した。WASMクレートは`wasm-pack`でビルドして、`static/`に出力する。


## 技術選定

### wasm-bindgen

RustからJavaScriptの世界とやり取りするための橋渡し役。これがないとWASM開発は始まらない。

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // WASMロード時に自動実行
    init_app()?;
    Ok(())
}
```

`#[wasm_bindgen(start)]`を付けると、WASMがロードされた瞬間に実行される。DOMContentLoadedを待つ必要がない。

### web-sys

ブラウザAPIへのRustバインディング。`document.querySelector`とか`element.classList.add`とか、普段JavaScriptで書いていることをRustで書ける。

```toml
[dependencies.web-sys]
version = "0.3"
features = [
    "Document",
    "Element",
    "HtmlElement",
    "HtmlInputElement",
    "Selection",
    "TreeWalker",
    "KeyboardEvent",
    # ...必要な機能を列挙
]
```

特徴的なのは、使う機能を明示的にfeatureとして指定する必要があること。最初は「なんでこんな面倒なことを…」と思ったけど、これのおかげでバイナリサイズが抑えられる。

### wasm-bindgen-futures

非同期処理に必要。`fetch`でJSONを取得するときなどに使う。

```rust
use wasm_bindgen_futures::JsFuture;

async fn fetch_search_index() -> Result<Vec<Article>> {
    let window = web_sys::window().unwrap();
    let resp = JsFuture::from(window.fetch_with_str("/search-index.json")).await?;
    // ...
}
```

`JsFuture`でJavaScriptのPromiseをRustのFutureに変換できる。async/awaitがそのまま使えるのは嬉しい。


## 大変だったポイント

ここからが本題。移行作業で苦労した部分を紹介する。

### 1. TreeWalkerによるテキストノード走査

Vim風のカーソル移動を実現するには、DOM内のテキストノードを順番に辿る必要がある。

JavaScriptでは`document.createTreeWalker`を使っていた。web-sysにも同じAPIがある。

```rust
use web_sys::{Document, HtmlElement, Node, NodeFilter, TreeWalker};

pub struct TextNodeWalker {
    walker: TreeWalker,
}

impl TextNodeWalker {
    pub fn new(root: &HtmlElement) -> Result<Self> {
        let document = document()?;

        // SHOW_TEXT = テキストノードのみを対象
        let walker = document
            .create_tree_walker_with_what_to_show(root, NodeFilter::SHOW_TEXT)
            .map_err(|e| DnfolioError::TreeWalkerError(format!("{:?}", e)))?;

        Ok(Self { walker })
    }

    pub fn next(&self) -> Result<Option<Node>> {
        self.walker
            .next_node()
            .map_err(|e| DnfolioError::TreeWalkerError(format!("{:?}", e)))
    }
}
```

ここまでは順調だった。問題はフィルタリングだ。

コードブロックの「Copy」ボタンやヘッダー部分にカーソルが入り込まないようにしたい。JavaScriptでは`NodeFilter.acceptNode`をオーバーライドしていたのだが、web-sysでは少し勝手が違った。

結局、TreeWalkerで取得した後に自前でフィルタリングする方式にした。

```rust
pub fn collect_filtered(&self) -> Result<Vec<Node>> {
    let mut nodes = Vec::new();

    while let Some(node) = self.next()? {
        // vim-cursorクラスを持つ親は除外（カーソル自体を拾わない）
        if let Some(parent) = node.parent_element() {
            if parent.class_list().contains("vim-cursor") {
                continue;
            }
        }

        // 空白のみのノードは除外
        if let Some(text) = node.text_content() {
            if !text.trim().is_empty() {
                nodes.push(node);
            }
        }
    }

    Ok(nodes)
}
```

### 2. コードブロック内でのカーソル消失問題

これが一番厄介だった。

`j`キーで下に移動しているとき、コードブロックを通過するとカーソルが消える。でも`k`キーで上に戻ると正常に表示される。

原因を調べたら、コードブロックのヘッダー部分（言語名表示とCopyボタン）を通過するときに、カーソルがボタン要素の中に入り込んでいた。

```rust
// ボタンやヘッダー要素をスキップする
fn should_skip_element(node: &Node) -> bool {
    if let Some(element) = node.dyn_ref::<Element>() {
        let tag_name = element.tag_name().to_lowercase();
        if tag_name == "button" {
            return true;
        }

        let class_list = element.class_list();
        if class_list.contains("code-block-header")
            || class_list.contains("code-copy-btn")
            || class_list.contains("code-lang")
        {
            return true;
        }
    }
    false
}
```

さらに、改行文字（`\n`）の位置にカーソルが止まると見えなくなる問題もあった。

```rust
// カーソル位置が改行文字なら、前後の有効な文字を探す
let chars: Vec<char> = text.chars().collect();
let mut char_at_cursor = chars.get(offset).copied();

if char_at_cursor == Some('\n') || char_at_cursor == Some('\r') {
    // 前方に有効な文字を探す
    for i in (offset + 1)..chars.len() {
        let c = chars[i];
        if c != '\n' && c != '\r' {
            offset = i;
            char_at_cursor = Some(c);
            break;
        }
    }
}
```

JavaScriptでは「なんとなく動いていた」部分が、Rustに移植すると問題が顕在化する。型システムのおかげでエッジケースに気づけたとも言える。

### 3. Selection APIの癖

カーソルの位置を取得・設定するにはSelection APIを使う。

```rust
pub struct SelectionHelper {
    selection: Selection,
}

impl SelectionHelper {
    pub fn get() -> Result<Self> {
        let window = window()?;
        let selection = window
            .get_selection()
            .map_err(|e| DnfolioError::SelectionError(format!("{:?}", e)))?
            .ok_or_else(|| DnfolioError::SelectionError("No selection".into()))?;

        Ok(Self { selection })
    }

    pub fn collapse(&self, node: &Node, offset: u32) -> Result<()> {
        self.selection
            .collapse_with_offset(Some(node), offset)
            .map_err(|e| DnfolioError::SelectionError(format!("{:?}", e)))
    }
}
```

`Selection.modify()`がhjkl移動の核になる。

```rust
// lキー（右移動）
self.selection
    .modify("move", "forward", "character")
    .map_err(|e| DnfolioError::SelectionError(format!("{:?}", e)))?;
```

これでキャレットが1文字右に動く。JavaScriptと全く同じAPIだけど、Rustで書くと型安全になる。`"move"`を`"moove"`とタイポしたらコンパイルは通るけど実行時エラーになる…と思うじゃん？

実は`modify`の引数は文字列リテラルだから、タイポしてもコンパイルは通る。ここはTypeScriptの型定義の方が優秀かもしれない。

### 4. 非同期処理とクロージャの罠

検索機能では`search-index.json`をfetchで取得する必要がある。

```rust
pub async fn load(&self) -> Result<()> {
    let window = web_sys::window().unwrap();

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);

    let request = Request::new_with_str_and_init("/search-index.json", &opts)?;

    let resp = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp.dyn_into()?;

    let json = JsFuture::from(resp.json()?).await?;
    let articles: Vec<SearchArticle> = serde_wasm_bindgen::from_value(json)?;

    // ...
}
```

async関数自体は普通に書ける。問題はこれをイベントハンドラから呼ぶとき。

```rust
// これはコンパイルエラー
input.add_event_listener_with_callback("input", |e| async {
    search_modal.search(&query).await;
})?;
```

クロージャはasyncにできない。`wasm_bindgen_futures::spawn_local`を使う必要がある。

```rust
let handler = Closure::wrap(Box::new(move |_: web_sys::InputEvent| {
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = do_search().await {
            web_sys::console::error_1(&format!("Search error: {e}").into());
        }
    });
}) as Box<dyn Fn(web_sys::InputEvent)>);

input.add_event_listener_with_callback("input", handler.as_ref().unchecked_ref())?;
handler.forget();  // メモリリークに注意
```

`handler.forget()`でクロージャをリークさせている。これはイベントハンドラがページ生存中ずっと必要だから。でも、動的に追加・削除するケースでは適切に管理しないとメモリリークになる。

### 5. 検索結果へのスクロールタイミング

検索モーダルから記事を選んで遷移すると、該当キーワードにハイライトが付いてスクロールする…はずだった。

でも実際はスクロールしたりしなかったり、不安定だった。

原因はDOM更新とスクロールのタイミング。ハイライトを適用した直後にスクロールしても、DOMがまだ完全に更新されていないことがある。

```rust
// ハイライト適用直後にスクロール → 不安定
el.scroll_into_view();

// 少し待ってからスクロール → 安定
let callback = Closure::once(Box::new(move || {
    let options = ScrollIntoViewOptions::new();
    options.set_behavior(ScrollBehavior::Instant);
    options.set_block(ScrollLogicalPosition::Center);
    el.scroll_into_view_with_scroll_into_view_options(&options);
}) as Box<dyn FnOnce()>);

window.set_timeout_with_callback_and_timeout_and_arguments_0(
    callback.as_ref().unchecked_ref(),
    200,  // 200ms待つ
)?;
callback.forget();
```

結局、`setTimeout`で200ms待つという古典的な解決策に落ち着いた。エレガントではないけど、確実に動く。

### 6. hjklカーソル移動の地獄

これが一番心を折られた。

「hjklで上下左右に動くだけでしょ？簡単じゃん」

そう思っていた時期が私にもありました。

#### シンタックスハイライトの罠

まず、シンタックスハイライトされたコードブロック内で`j`（下移動）が動かない。

原因を調べると、syntectによるハイライトでコードが細かい`<span>`に分割されていた。

```html
<pre><code>
<span style="color:#b48ead;">fn</span> <span style="color:#8fa1b3;">main</span>() {
    <span style="color:#96b5b4;">println!</span>(<span style="color:#a3be8c;">"hello"</span>);
}
</code></pre>
```

1行が複数のテキストノードに分割されている。`Selection.modify("move", "forward", "line")`は「視覚的な行」で動くため、これらの`<span>`境界を正しく認識できない。

結局、自前で「改行文字を探して越える」ロジックを実装した。

```rust
/// 次の改行の後の位置を探す（シンタックスハイライトされたコード対応）
fn find_position_after_next_newline(
    start_node: &Node,
    start_offset: u32,
    block: &Element,
    main_content: &HtmlElement,
) -> Result<Option<(Node, u32)>> {
    let walker = TextNodeWalker::new(main_content)?;
    walker.set_current(start_node);

    // 現在のノード内で改行を探す
    let text = start_node.text_content().unwrap_or_default();
    let chars: Vec<char> = text.chars().collect();

    for (i, &c) in chars.iter().enumerate().skip(start_offset as usize) {
        if c == '\n' {
            let next_pos = i + 1;
            if next_pos < chars.len() {
                return Ok(Some((start_node.clone(), next_pos as u32)));
            } else {
                // 改行がノードの最後なので、次のノードへ
                if let Some(next_node) = walker.next()? {
                    // 同じブロック内かチェック...
                }
            }
        }
    }

    // 次のテキストノードも探す...
}
```

複数のテキストノードを跨いで改行を探し、その後の位置に移動する。単純な「下に1行」がこんなに複雑になるとは思わなかった。

#### インラインコードの境界問題

次に、インラインコード（`` `code` ``）を含む文章で`l`（右移動）が動かなくなった。

```html
<p>JavaScriptでは<code>document.createTreeWalker</code>を使っていた。</p>
```

`<code>`内の最後の文字から、次の「を」に移動できない。いや、移動したと思ったら段落の先頭「J」に戻る。

原因は`normalize()`だった。

ブロックカーソルを削除するとき、テキストノードを正規化（隣接するテキストノードを結合）している。その後、Selectionを復元するために「親要素の最初のテキストノード」を探していた。

```rust
// 問題のあるコード
parent.normalize();

// 「親の最初のテキストノード」を探す
let child_nodes = parent.child_nodes();
for i in 0..child_nodes.length() {
    if let Some(child) = child_nodes.get(i) {
        if child.node_type() == Node::TEXT_NODE {
            sel.collapse(&child, offset)?;  // ← ここで段落先頭に飛ぶ
            break;
        }
    }
}
```

`<p>`の子ノードは「JavaScriptでは」「`<code>`要素」「を使っていた。」の3つ。最初のテキストノードを探すと「JavaScriptでは」が見つかる。カーソルは「を」にいたはずなのに、「J」に戻ってしまう。

修正は、`normalize()`の前にSelectionを設定すること。

```rust
// 修正後
parent.replace_child(&text_node, &cursor_el)?;

// normalize前にSelectionを設定
sel.collapse(&text_node, 0)?;

// 正規化
parent.normalize();

// normalize後、オフセットを調整
if let Some(anchor) = sel.anchor_node() {
    if offset_in_merged_node > 0 {
        sel.collapse(&anchor, offset_in_merged_node)?;
    }
}
```

`normalize()`はSelectionを保持してくれる。ただし、隣接テキストノードが結合されるとオフセットがずれるので、その分を加算する必要がある。

#### 列位置の維持

Neovimでは、`j`で下に移動しても列位置が維持される。10列目から下に移動したら、次の行でも10列目にいたい（行が短ければ行末）。

これも自前実装が必要だった。

```rust
fn move_to_adjacent_block_with_column(
    sel: &SelectionHelper,
    current_node: &Node,
    current_offset: u32,
    main_content: &HtmlElement,
    direction: char,
) -> Result<()> {
    // 現在の列位置を計算
    let current_col = Self::get_column_in_line(&text, current_offset as usize);

    // 次のブロック要素を探す
    let target_block = Self::find_next_block_element(current_node, main_content)?;

    if let Some(block) = target_block {
        if let Some(text_node) = Self::find_first_text_node(&block) {
            let new_text = text_node.text_content().unwrap_or_default();
            let char_count = new_text.chars().count();

            // 列位置を維持（行が短い場合は行末）
            let target_offset = if current_col < char_count {
                current_col as u32
            } else if char_count > 0 {
                (char_count - 1) as u32
            } else {
                0
            };

            sel.collapse(&text_node, target_offset)?;
        }
    }
    Ok(())
}
```

#### IndexSizeError との戦い

デバッグ中、コンソールに大量の`IndexSizeError`が出た。

```
IndexSizeError: Failed to execute 'collapse' on 'Selection':
The offset 6 is larger than the node's length (3).
```

シンタックスハイライトでテキストノードが3文字しかないのに、列位置6を指定しようとしている。全ての`collapse`呼び出しの前に範囲チェックを入れた。

```rust
let target_offset = if char_count == 0 {
    0
} else {
    let max_offset = (char_count - 1) as u32;
    (current_col as u32).min(max_offset)
};
```

#### 教訓

「カーソルを上下左右に動かす」という単純に見える機能が、DOMの構造によってここまで複雑になるとは思わなかった。

- シンタックスハイライトはテキストを細切れにする
- `normalize()`はSelectionを壊す（ことがある）
- `Selection.modify()`は万能じゃない
- オフセットは常に範囲チェックが必要

JavaScriptで「なんとなく動いていた」ものをRustで再実装すると、全てのエッジケースに向き合うことになる。型システムは助けてくれるけど、DOM操作の複雑さは変わらない。

ただ、この苦労のおかげで、今ではhjkl移動が完璧に動く。コードブロック内も、インラインコードも、どこでも。その達成感は格別だ。


## 移行の成果

### コード量の変化

| 項目 | Before (JS) | After (Rust) |
|------|-------------|--------------|
| 行数 | 約2,000行 | 約2,500行 |
| ファイル数 | 1（インライン） | 15 |
| 型安全性 | なし | コンパイル時保証 |
| LSP補完 | なし | あり |

行数は増えた。Rustは冗長になりがちだし、エラーハンドリングも明示的に書く必要があるから仕方ない。

でも、ファイルが分割されてモジュール化されたことで、可読性は格段に上がった。そしてなにより、rust-analyzerの恩恵を受けられるようになった。これが一番嬉しい。

### 機能の変化

ユーザーから見える違いは…ない。

見た目も動作も全く同じ。「100% Rust」と胸を張って言えるようになっただけだ。

自己満足？その通りである。


## 学んだこと

### web-sysのfeature管理

最初は「使うものを全部有効にすればいいや」と思っていた。でもそれだとコンパイル時間が長くなるし、バイナリサイズも膨らむ。

必要最小限のfeatureを精査して指定するのが大事。エラーメッセージを見ながら「この型がない」「このメソッドがない」と一つずつ追加していく作業は地味だけど、結果的に軽量なWASMになる。

### RefCellとの付き合い方

Rustでグローバルな状態を管理するのは面倒だ。今回は`thread_local!`と`RefCell`を組み合わせた。

```rust
thread_local! {
    static EDITOR_STATE: RefCell<EditorState> = RefCell::new(EditorState::default());
}

pub fn with_editor_state<F, R>(f: F) -> R
where
    F: FnOnce(&EditorState) -> R,
{
    EDITOR_STATE.with(|state| f(&state.borrow()))
}
```

WASMはシングルスレッドだから`Mutex`は不要。でも実行時の借用チェックは必要だから`RefCell`を使う。

`borrow()`と`borrow_mut()`を同時に呼ぶとパニックするので、借用のスコープには注意が必要。

### wasm-packの便利さ

`wasm-pack build --target web`一発でJSグルーコードとWASMバイナリが生成される。cargo-makeと組み合わせてビルドを自動化した。

```toml
[tasks.wasm]
script = '''
cd crates/dnfolio-wasm
wasm-pack build --target web --out-dir ../../static --out-name dnfolio_wasm --dev
'''

[tasks.build]
dependencies = ["wasm", "ssg"]
```

`cargo make build`で全部ビルドされる。開発体験は良好だ。


## セキュリティ対策

WASMへの移行が完了した後、セキュリティ面での見直しも行った。静的サイトとはいえ、クライアントサイドで動くコードには注意が必要だ。

### XSS対策

最初の実装では`set_inner_html`を使っている箇所があった。検索結果が0件のときに「No results」と表示する部分など。

```rust
// 危険: XSS脆弱性の可能性
preview.set_inner_html("<div class=\"preview-empty\">No results</div>");
```

これを安全なDOM操作に置き換えた。

```rust
// 安全: テキストコンテンツとして設定
fn create_text_div(parent: &Element, class_name: &str, text: &str) -> Result<Element> {
    let doc = document()?;
    let div = doc.create_element("div")?;
    div.set_class_name(class_name);
    div.set_text_content(Some(text));  // XSS安全
    parent.append_child(&div)?;
    Ok(div)
}

// 使用例
create_text_div(&preview, "preview-empty", "No results")?;
```

### 入力値検証

ユーザー入力（検索クエリ、コマンド）には長さ制限を設けた。極端に長い入力はReDoS攻撃やメモリ枯渇の原因になりうる。

```rust
pub const MAX_SEARCH_QUERY_LEN: usize = 500;
pub const MAX_COMMAND_LEN: usize = 1000;

pub fn validate_search_query(query: &str) -> Result<&str> {
    if query.len() > MAX_SEARCH_QUERY_LEN {
        return Err(DnfolioError::ValidationError("検索クエリが長すぎます".into()));
    }
    Ok(query)
}
```

### URL検証

検索結果から記事に遷移する際、URLの検証を追加した。`javascript:`や`data:`スキームを使った攻撃を防ぐ。

重要なのは **Protocol-relative URL** (`//evil.com`) のチェック。これは`/`で始まるため、単純な`starts_with('/')`チェックをすり抜けてしまう。

```rust
pub fn validate_url(url: &str) -> Result<&str> {
    // Protocol-relative URL（//で始まる）を先に拒否
    // これは外部サイトへのリダイレクトに悪用される
    if url.starts_with("//") {
        return Err(DnfolioError::ValidationError(
            "Protocol-relative URLは許可されていません".into(),
        ));
    }

    // #で始まるフラグメントは許可
    if url.starts_with('#') {
        return Ok(url);
    }

    // /で始まる相対パスは許可（//は上で除外済み）
    if url.starts_with('/') {
        return Ok(url);
    }

    // 危険なスキームをチェック
    let dangerous = ["javascript:", "data:", "vbscript:", "blob:"];
    for scheme in dangerous {
        if url.to_lowercase().starts_with(scheme) {
            return Err(DnfolioError::ValidationError("危険なURLスキーム".into()));
        }
    }

    Err(DnfolioError::ValidationError("無効なURLフォーマット".into()))
}
```

### エラー情報の隠蔽

リリースビルドでは詳細なエラー情報を隠蔽するようにした。攻撃者に内部情報を与えないため。

```rust
fn console_error_panic_hook_setup() {
    std::panic::set_hook(Box::new(|panic_info| {
        #[cfg(debug_assertions)]
        {
            // デバッグ: 詳細情報を出力
            web_sys::console::error_1(&format!("WASM Panic: {}", panic_info).into());
        }

        #[cfg(not(debug_assertions))]
        {
            // リリース: 最小限の情報のみ
            web_sys::console::error_1(&"An internal error occurred".into());
        }
    }));
}
```

静的サイトでここまでやる必要があるのか？正直、過剰かもしれない。でも、Rustを使う以上は「正しく書く」ことにこだわりたかった。


## まとめ

MaudテンプレートにベタベタのインラインJavaScriptで書いていたVim風UIを、Rust + WebAssemblyで完全に書き直した。

大変だったか？大変だった。特にTreeWalkerとSelection APIの挙動を理解するのに時間がかかった。コードブロックでカーソルが消える問題は、原因究明に何時間もかかった。

でも、今は「ロジックは100% Rustで書いています」と胸を張って言える。Aboutページの記述に嘘がなくなった。

機能的には何も変わっていない。ユーザーには違いがわからない。完全に自己満足の世界だ。

でも、個人サイトってそういうものだと思う。誰かのためじゃなく、自分が納得できるものを作る。その過程で新しい技術を学ぶ。

次は何をしよう。WASMでもっと面白いことができるかもしれない。夢は広がる。

