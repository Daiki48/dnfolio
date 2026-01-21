+++
title = "俺のNeovimっぽいもの、Webに現る - dnfolio v6.0.0"
slug = "dnfolio-v6-my-neovim-style-ui"
description = "個人サイトを「私のNeovim環境っぽいUI」に全面改修した。sakurajima.nvimカラーテーマ、行番号、:qコマンド、snacks.nvim風検索まで実装した狂気の記録。"
created = "2026-01-22"
draft = false
[taxonomies]
tags = ["Rust", "CSS", "JavaScript", "Neovim"]
languages = ["ja"]
+++

## はじめに

このサイト、気づいただろうか。見た目が完全に変わっている。

v5.4.7までは「左に記事一覧、真ん中にコンテンツ、右に目次」という、まあ普通のブログレイアウトだった。シンプルで見やすい。堅実。無難。

でも私は思ってしまった。

**「普段Neovimの中で過ごしているのに、自分のサイトがNeovimっぽくないのは寂しくないか？」**

この一言が全ての始まりだった。

2026年1月、私は3,000行以上のコードを書き換え、このサイトを「Neovimの中で記事を読んでいる感覚」に生まれ変わらせた。

ただし「完全再現」ではない。普段はoil.nvimを画面中央フローティングで使っているし、プラグイン構成も人それぞれだ。あくまで **Neovim + sakurajima.nvim のカラースキームを踏襲した「私のNeovimっぽい」環境** である。あなたのNeovimと違っても、それは仕様です。

これはその狂気の記録である。


## 新UIの全体像

まずは見た目の変化を整理しよう。

### Before（v5.4.7）
- 3カラムレイアウト（左: 記事一覧、中央: コンテンツ、右: 目次）
- 白基調のシンプルなデザイン
- 普通のヘッダーとフッター

### After（v6.0.0）
- 2カラムレイアウト（EXPLORER + エディター）
- sakurajima.nvim（自作カラーテーマ）ベースのダークテーマ
- ターミナルアプリ風タイトルバー + ウィンドウボタン
- 行番号表示
- ステータスライン + コマンドライン
- snacks.nvim風の検索UI

要するに、**私のNeovim環境っぽいものをブラウザで表現した**ようなものだ。

「そこまでやる？」という声が聞こえる気がする。

いや、まだ足りない。Vimmerなら分かるはず。`.vimrc`や`init.lua`を延々と弄り続けるあの感覚。「もうちょっとだけ」が止まらないあの沼。個人サイトも同じだ。こだわれるところは、とことんこだわりたい。


## sakurajima.nvim カラーパレット

まず最初に取り組んだのは、カラーテーマの統一だ。

普段の開発環境は Neovim + sakurajima.nvim（自作カラーテーマ）なので、そのまま再現することにした。

```css
:root {
    /* 背景系 */
    --bg-primary: #22272e;     /* メイン背景 */
    --bg-secondary: #2D333B;   /* サイドバー */
    --bg-elevated: #1D3A64;    /* 選択・ホバー */

    /* テキスト系 */
    --text-primary: #8b9aaa;   /* メインテキスト */
    --text-secondary: #7c6f64; /* 控えめテキスト */
    --text-bright: #ebdbb2;    /* 強調テキスト */
    --text-muted: #7D7D7D;     /* コメント */

    /* アクセント */
    --accent-cyan: #3B7B7D;    /* キーワード */
    --accent-yellow: #B3AF78;  /* 関数 */
    --accent-green: #658D50;   /* 定数 */
    --accent-orange: #97812C;  /* 文字列 */
    --accent-blue-light: #82ade0; /* リンク */
    --accent-red-bright: #E34C36; /* エラー */

    /* ボーダー */
    --border-color: #3D4450;
}
```

CSS変数として定義することで、サイト全体で一貫したカラーが使えるようになった。

エディタで見慣れた色がブラウザに表示されると、不思議な安心感がある。「帰ってきた」という感覚。


## タイトルバーとウィンドウボタン

Neovimっぽさを出すために、タイトルバーをデスクトップアプリ風にした。

```css
.titlebar {
    background: linear-gradient(180deg, var(--bg-secondary) 0%, var(--bg-primary) 100%);
    border-bottom: 1px solid var(--border-color);
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.window-button {
    width: 28px;
    height: 28px;
    border-radius: 4px;
}

.btn-minimize::before { content: "\2212"; } /* − */
.btn-maximize::before { content: "\25A1"; } /* □ */
.btn-close::before { content: "\00D7"; }    /* × */
```

右上には最小化・最大化・閉じるボタンが並んでいる。Linux Mint の Cinnamon デスクトップ風だ。

もちろん、押しても何も起きない。Webサイトだから。

でもホバーするとちゃんと色が変わる。閉じるボタンは赤くなる。無意味だけど芸が細かい。


## EXPLORER - ファイルツリー風サイドバー

左サイドバーは「EXPLORER」と名付けた。VS Codeっぽいけど、Neovimのファイラープラグインをイメージしている。

...と言いつつ、実は私は普段 Oil.nvim を画面中央にフローティング表示するスタイルで使っている。左サイドバーにファイラーを常駐させる派ではない。

まあ、Webサイトのレイアウトとして自然なのは左サイドバーだし、細かいことは気にしない。「雰囲気」が大事なのだ。

### 年月別グループ化

記事を年 → 月 → 記事タイトルの階層で表示するようにした。

```rust
// 記事を年月別にグループ化
fn group_articles_by_year_month(articles: &[Article]) -> Vec<YearGroup> {
    let mut year_map: HashMap<i32, HashMap<u32, Vec<Article>>> = HashMap::new();

    for article in articles {
        if let Some(date) = extract_date_from_filename(&article.source_path) {
            let year = date.year();
            let month = date.month();
            year_map
                .entry(year)
                .or_default()
                .entry(month)
                .or_default()
                .push(article.clone());
        }
    }

    // 新しい年/月が上に来るようにソート
    // ...
}
```

表示はこんな感じ:

```
EXPLORER
v 2026
  v 01
    - dnfolio v6.0.0 - Neovim風...
    - WeztermからAlacritty+Zell...
    - Ghosttyを試してみた結果...
  v 12
    - ...
v 2025
  v 11
    - ...
```

ツリー記号（`v`, `-`）もファイラー風だ。現在閲覧中の記事には `v`（展開）がつく。

### OUTLINE - 目次をエクスプローラー内に統合

以前は右サイドバーに目次を配置していたが、今回は左サイドバーの上部に移動した。

```css
.toc-section {
    padding: 6px 12px;
    border-bottom: 1px solid var(--border-color);
    margin-bottom: 4px;
}

.toc-header {
    font-size: 0.85rem;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    gap: 8px;
}
```

「目次」という日本語ではなく「≡ OUTLINE」と表示している。Neovim の symbols-outline.nvim や aerial.nvim っぽさを意識した。


## 行番号表示 - CSSカウンターの魔法

エディタといえば行番号だ。これがないとNeovim感が出ない。

CSSの `counter-increment` を使って実装した。

```css
.main-content {
    counter-reset: line-number;
}

.main-content > h1,
.main-content > h2,
.main-content > p,
.main-content > pre,
/* ... その他のブロック要素 */ {
    position: relative;
    counter-increment: line-number;
}

.main-content > p::before {
    content: counter(line-number);
    position: absolute;
    left: -3.5em;
    color: var(--text-secondary);
    font-family: var(--font-mono);
    font-size: 0.85rem;
}
```

各ブロック要素（見出し、段落、リスト、コードブロックなど）に対して、擬似要素で行番号を表示している。

行番号をクリックすると、その行がハイライトされる機能も実装した。

```javascript
const setupLineNumberClicks = () => {
    const elements = getLineElements();
    elements.forEach((el, index) => {
        el.addEventListener('click', (e) => {
            const rect = el.getBoundingClientRect();
            if (e.clientX < rect.left) {
                elements.forEach(el => el.classList.remove('current-line'));
                el.classList.add('current-line');
            }
        });
    });
};
```

行番号の左側をクリックすると、その行に青いハイライトが入る。Neovimの `cursorline` っぽい演出だ。

### モバイル対応

ただし、モバイルでは行番号を非表示にした。

```css
@media screen and (max-width: 992px) {
    .main-content > h1::before,
    .main-content > h2::before,
    .main-content > p::before,
    /* ... */ {
        display: none;
    }
}
```

モバイルで行番号を表示しても邪魔なだけだし、そもそもNeovimをスマホで使う人はいないだろう。


## コードブロックのコピー機能

コードブロックにヘッダーバーを追加し、言語表示とコピーボタンを実装した。

```rust
fn highlight_code(lang: &str, code: &str) -> String {
    let display_lang = if lang.is_empty() { "text" } else { lang };

    let escaped_code = code
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");

    format!(
        r#"<div class="code-block-wrapper">
<div class="code-block-header">
<span class="code-lang">{}</span>
<button class="code-copy-btn" data-code="{}" title="コピー">
<svg>...</svg>
<span class="copy-text">Copy</span>
</button>
</div>
{}</div>"#,
        display_lang, escaped_code, highlighted_html
    )
}
```

GitHub風のデザインで、ヘッダー左に言語名、右にコピーボタンが表示される。

コピーボタンを押すと「Copied!」に変わる。地味だけど便利。


## ステータスライン - lualine風

Neovimといえばステータスラインだ。lualineを意識したデザインにした。

```css
.statusline {
    position: fixed;
    bottom: var(--commandline-height);
    left: 0;
    right: 0;
    height: var(--statusline-height);
    background: var(--bg-secondary);
    border-top: 1px solid var(--border-color);
    display: flex;
    font-family: var(--font-mono);
    font-size: 0.75rem;
}

.statusline-mode {
    background: var(--accent-cyan);
    color: var(--bg-primary);
    padding: 0 12px;
    font-weight: 600;
}
```

左側:
- **NORMAL** - モード表示（もちろん常にNORMAL）
- **main** - Gitブランチ風

右側:
- **検索カウント** - 検索結果の件数
- **UTF-8** - エンコーディング
- **markdown** - ファイルタイプ
- **Top** - スクロール位置

「NORMAL」と表示しておきながらINSERTモードに切り替わることはない。だってWebサイトだから。スクロール位置は「Top」「Bot」「All」「XX%」のように変化する。これもNeovimのステータスライン風だ。


## コマンドライン - Vimコマンドの再現

ステータスラインの下にコマンドラインを配置した。

`:` を押すとコマンドモードに入り、Neovim風のコマンドが使える。

```javascript
const commands = {
    ':q': () => showToast('E37: No write since last change',
        'add ! to override (冗談です、ここはWebです)', '!', 'warn'),
    ':q!': () => showToast('E37: No write since last change',
        '...だから、Webサイトなんですって', '!', 'warn'),
    ':wq': () => showToast('Already saved',
        'This is a static site. すでに保存済みです。', '💾', 'info'),
    ':help': () => showToast('Help - Keybindings',
        '/ or Ctrl+K: 検索\ngg: ページトップ\nG: ページボトム\n...',
        '❓', 'info'),
    ':version': () => showToast('dnfolio ' + document.body.dataset.version,
        'Built with Rust + maud\nTheme: sakurajima.nvim\nby Daiki Nakashima',
        '🦀', 'info'),
    ':colorscheme': () => showToast(':colorscheme',
        '現在: sakurajima.nvim (変更不可)', '🎨', 'info'),
    // ... 他にもいろいろ
};
```

`:q` で閉じようとしても閉じられない。これはVimを終了できない初心者あるあるへのオマージュだ。何度 `:q!` を押しても「Webサイトなんですって」と返される。

`:version` で現在のバージョンが表示される。これは後述するビルドスクリプトで自動的にgit tagから取得している。

行番号ジャンプも実装した。`:42` と入力すると42行目にジャンプする。

```javascript
const lineMatch = cmd.trim().match(/^:(\d+)$/);
if (lineMatch) {
    const lineNum = parseInt(lineMatch[1], 10);
    jumpToLine(lineNum);
    return true;
}
```

`:$` で最終行へジャンプ。Neovimと同じだ。


## 検索機能 - snacks.nvim風

`/` または `Ctrl+K` で検索モーダルが開く。これは snacks.nvim の grep 機能を意識したUIだ。

### INSERT/NORMALモード切り替え

検索モーダルにもモードがある。

- **INSERT**: 検索クエリを入力
- **NORMAL**: `j`/`k` で結果を移動、`Enter` で選択

```javascript
function setMode(mode) {
    currentMode = mode;
    if (modeIndicator) {
        modeIndicator.textContent = mode === 'insert' ? 'INSERT' : 'NORMAL';
        modeIndicator.classList.toggle('insert', mode === 'insert');
        modeIndicator.classList.toggle('normal', mode === 'normal');
    }

    if (mode === 'insert') {
        searchInput.focus();
    } else {
        searchInput.blur();
    }
}
```

`Escape` でINSERT → NORMAL、`i` でNORMAL → INSERTに切り替わる。

### プレビューペイン

検索結果を選択すると、右側にプレビューが表示される。マッチした行の前後のコンテキストも見える。

```javascript
function updatePreview(result) {
    if (!previewPane || !result) {
        if (previewPane) previewPane.innerHTML = '';
        return;
    }

    const lines = result.context.map((line, idx) => {
        const isMatch = idx === result.matchLineIdx;
        const lineClass = isMatch ? 'preview-line match' : 'preview-line';
        const lineText = isMatch
            ? highlightMatch(line.text, searchInput.value)
            : escapeHtml(line.text);
        return `<div class="${lineClass}">
            <span class="preview-line-num">${line.num}</span>
            <span class="preview-line-text">${lineText}</span>
        </div>`;
    }).join('');

    previewPane.innerHTML = `
        <div class="preview-header">${escapeHtml(result.title)}</div>
        <div class="preview-content">${lines}</div>
    `;
}
```

マッチした部分は `<mark>` タグでハイライトされる。

### 検索ハイライトナビゲーション

`n` で次のハイライト、`N` で前のハイライトへ移動できる。

```javascript
document.addEventListener('keydown', (e) => {
    if (e.key === 'n' && !isInputFocused()) {
        e.preventDefault();
        navigateHighlight(1);
    } else if (e.key === 'N' && !isInputFocused()) {
        e.preventDefault();
        navigateHighlight(-1);
    }
});
```

これも完全にNeovimの挙動だ。


## OUTLINEのスクロール追跡

スクロールに応じて、現在表示されている見出しがOUTLINEでハイライトされる機能を実装した。Webの世界では「Scroll Spy」と呼ばれる技法らしい。

最初は Intersection Observer API で実装したが、見出しをクリックした直後の判定がシビアだった。例えば「はじめに」をクリックしても「新UIの全体像」がハイライトされてしまう、みたいな。

結局、シンプルにスクロールイベントを監視する方式に変更した。

```javascript
const updateActiveHeading = () => {
    const scrollTop = window.scrollY;
    const offset = 100; // タイトルバー + 余裕

    let activeHeading = null;
    headings.forEach(heading => {
        const rect = heading.getBoundingClientRect();
        const headingTop = rect.top + scrollTop;
        if (headingTop <= scrollTop + offset) {
            activeHeading = heading;
        }
    });

    if (activeHeading) {
        tocItems.forEach(item => item.classList.remove('active'));
        const activeItem = tocMap.get(activeHeading.id);
        if (activeItem) {
            activeItem.classList.add('active');
        }
    }
};

// throttle付きでスクロールイベントを監視
let ticking = false;
window.addEventListener('scroll', () => {
    if (!ticking) {
        requestAnimationFrame(() => {
            updateActiveHeading();
            ticking = false;
        });
        ticking = true;
    }
});
```

アクティブな見出しには左に縦線が入る。これも symbols-outline.nvim っぽい演出だ。


## バージョン同期 - git tagとビルドスクリプト

`:version` コマンドで表示されるバージョンは、git tagから自動取得している。

### build_script.rs

```rust
use std::process::Command;

fn main() {
    let version = get_git_version().unwrap_or_else(|| "dev".to_string());
    println!("cargo:rustc-env=GIT_VERSION={}", version);
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/tags");
}

fn get_git_version() -> Option<String> {
    let output = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .output()
        .ok()?;

    if output.status.success() {
        let version = String::from_utf8(output.stdout).ok()?;
        Some(version.trim().to_string())
    } else {
        // タグがない場合はコミットハッシュを使用
        let output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()?;

        if output.status.success() {
            let hash = String::from_utf8(output.stdout).ok()?;
            Some(format!("dev-{}", hash.trim()))
        } else {
            None
        }
    }
}
```

### HTMLへの埋め込み

```rust
const GIT_VERSION: &str = env!("GIT_VERSION");

// ...

html! {
    body data-version=(GIT_VERSION) {
        // ...
    }
}
```

JavaScriptからは `document.body.dataset.version` で取得できる。

これにより、git tagを打つだけでサイトのバージョン表示が自動更新される。Cargo.tomlのバージョンを手動で管理する必要がなくなった。


## キーバインドまとめ

最終的に実装したキーバインドを整理する。

| キー | 機能 |
|------|------|
| `/` または `Ctrl+K` | 検索モーダルを開く |
| `gg` | ページトップへ移動 |
| `G` | ページボトムへ移動 |
| `n` | 次の検索ハイライトへ |
| `N` | 前の検索ハイライトへ |
| `:` | コマンドモード |
| `j` / `k` | 検索結果の移動（NORMALモード時） |
| `Escape` | モーダルを閉じる / INSERT→NORMAL |
| `i` | NORMAL→INSERT |

Vimmerなら思わず手が動くバインドばかりだ。


## 変更統計

最後に、今回の変更量をまとめる。

```
 src/templates/base.rs | 2550 +++++++++++++++++++++++++++++++++++++++++++++----
 src/build.rs          |  680 +++++++++++--
 build_script.rs       |   39 +
 pages/about.md        |   36 +
 src/ogp.rs            |   14 +-
 src/ogp_template.svg  |   76 +-
 8 files changed, 3080 insertions(+), 320 deletions(-)
```

**3,080行の追加、320行の削除。**

ほとんどが `base.rs`（CSS + JavaScript）の変更だ。Rustのコードでありながら、中身はフロントエンドの実装という不思議な構成。


## 謝辞

今回の大改修は、私一人では成し遂げられなかった。

**Claude Code（Opus 4.5）** に心から感謝したい。

「Neovim風UIにしたい」という漠然としたアイデアを伝えると、sakurajima.nvimのカラーパレットからCSS変数の設計、2カラムレイアウトの構造、JavaScriptの実装まで、一緒に考え、一緒に作り上げてくれた。

「行番号クリックしたら左にズレる」「OUTLINEの追跡がシビアすぎる」「モバイルで行番号邪魔」...私が「なんかおかしい」と言うたびに、原因を特定し、修正案を提示してくれた。

3,000行以上のコード変更を、私はNeovimで書き、Claudeは設計とレビューを担当する。この協業スタイルが、今回の狂気じみた改修を可能にした。

AIと一緒にコードを書く時代が来たんだな、と実感した数日間だった。


## 終わりに

振り返ってみると、まだまだ足りない気がする。

「シンプルな個人サイト」から「私のNeovimっぽいギークなUI」への進化。ここまで作り込んでも、「あれも実装したい」「ここも再現したい」という欲が湧いてくる。Vimmerの性だ。

誰かに使ってもらうためではなく、自分が楽しむために作る。自分の好きなものを、好きな言語で、好きなだけ作り込む。

このサイトを見て「なんかNeovimっぽい」と思ってくれる人がいたら嬉しい。「俺のNeovimと違う」と思われても、そりゃそうだ、これは私のNeovimだから。

次は何を実装しようか。`:split` でウィンドウ分割とか？LSP風の補完とか？

...割と本気で考えている。
