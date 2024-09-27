---
title: 自作カラースキーム「sakurajima.nvim」誕生までの話
description: 暖色系カラーテーマのsakurajima.nvimを作ったので、少しだけ語りたい。
createdAt: '2024-06-01'
tags:
  - Neovim
published: true
---

<script>
  import HL from '$components/modules/HL.svelte';
  import Img from '$components/modules/Img.svelte';
</script>

<HL el="h2" text="はじめに" />

普段利用しているテキストエディタ「Neovim」のカラースキームを作りました。

[sakurajima.nvim](https://github.com/Daiki48/sakurajima.nvim)

私の地元鹿児島を代表する「桜島」がモチーフです。

<Img src="/images/neovim/002-sakurajima-theme-introduction/sakurajima-icon.png" alt="sakurajima" />

<HL el="h2" text="どうして作ったのか" />

Luaでカラースキームを作ってみるのに興味がありました。

私はすでに自作カラースキーム [coolnessFlair.vim](https://github.com/Daiki48/coolnessFlair.vim) で満足していました。

coolnessFlair.vimは寒色系で、青色メインで少し寒さをイメージして作りました。名前の通り「冷静な閃き」がコンセプトです。

<Img src="/images/neovim/002-sakurajima-theme-introduction/coolnessflair.png" alt="coolnessFlair.vim" />

この配色は [iceberg.vim](https://github.com/cocopon/iceberg.vim) の影響を強く受けています。作者の [自作Vimカラースキーム「Iceberg」の配色戦略](https://cocopon.me/blog/2016/02/iceberg/) というブログでは、その配色に関する戦略が語られており、とても参考になりました。私が感覚だけで良いと感じていた部分が言語化されており、更にiceberg.vimを好きになったと同時に、カラースキームの自作に興味を持ち、coolnessFlair.vimを作りました。

あれから2年、毎日coolnessFlair.vimを使用していましたが飽きることはありません。というよりも、自作する前までカラースキーム沼だったのが嘘のようにcoolnessFlair.vimで満足している自分がいました。

Luaでカラースキームを書きたいだけであれば、 「このcoolnessFlair.vimをLuaで書き直そうか」と最初は思っていました。しかし、書き直し作業を進めていくうちに暖色系のカラースキームを自作してみたくなりました。

<HL el="h2" text="暖色系カラースキームといえば？" />

- [gruvbox](https://github.com/morhetz/gruvbox)
- [solarized](https://ethanschoonover.com/solarized/)
- [kanagawa.nvim](https://github.com/rebelot/kanagawa.nvim)

私が暖色系のカラースキームだと思ったのは上記の3つです。

どれも有名なカラースキームです。  
気に入るものは見つからなかったため、自作の暖色系カラースキームのコンセプトを決めていきます。

<HL el="h2" text="思い付く暖かいもの" />

シンプルに、身の周りで暖かいものを探しました。  
太陽、ホットコーヒー、寝起きの猫、コンソメスープ、カレー...  
気持ちが暖かくなることもあるので、成猫が子猫を毛繕いしている姿、落とし物を拾って感謝された時、信号無しの横断歩道で通学中の小学生にお礼をされた時...

「なんか鹿児島アピールもしたいなぁ」

「黒豚とか鳥刺し、さつまあげ、美味しい食べ物たくさんあるなぁ」

「今日も噴火してるなぁ...桜島」

「風向きは...よし、大丈夫そうだな」

「他に暖かいものといえば何だろうか」

「ん？黒猫ちゃん、膝の上に乗りなさんな。暑苦しいでしょうが。」

「warmingcatってどうだろう？んー、なんか長くてキャッチーな感じが欲しいなぁ」

「え、ちょっとこれってもしかして...火山灰じゃないか？」

「えぇぇ...風向きは大丈夫だったはずなのになぜ...」

「あいかわらず元気な桜島だなぁ。」

「そういえば、桜島ってかなりアクティブ寄りな活火山じゃないか？」

「暖かいもの...桜島じゃん!!」

という感じで、sakurajima.nvim というテーマ名が決まりました。

<HL el="h2" text="桜島といえば" />

私が桜島で連想出来るものといえば

- 火山灰
- 活火山
- 人が住んでる
- 元気

こんな感じです。

火山灰は嬉しくないが鹿児島でしか味わえないものです。鹿児島を離れていた頃は、無意識に部屋干ししてしまうほど火山灰の存在が身近になっています。火山灰のイメージカラーはもちろん灰色です。

桜島はとても活発な火山です。頻繁に噴火しています。昼夜問わず噴火しますが、数年前にマグマが見えるほど激しく噴火したことがあります。あのマグマが忘れられません。マグマのイメージカラーは、赤色や橙色です。

そんな桜島ですが、普通に人が住んでいます。小学生の頃は毎週サッカーの試合で溶岩グラウンドへ行ってました。火山灰混じりの砂は汗と相性最悪でした。溶岩グラウンドの土は、火山灰が混じっていて若干灰色だった記憶です。真っ黒よりも灰色混じりのイメージカラーです。

桜島には元気をもらえます。学校に行くのが憂鬱な日、インフルエンザで寝込んだ日、運動会のかけっこで一位を獲った時、どんな時もそこに桜島がありました。最近では、仕事の合間の休憩で桜島を眺めている自分がいます。遠い山を見つめたり、緑を見ると眼が回復すると言われているような効果を実感します。(個人差があると思います)

<HL el="h2" text="ざっくりとしたカラーパレット" />

そこまできっちりとした設定はしていないですが、全体的に暗くしています。コントラストを控えめにする意味もありますが、火山灰が降った後の町並みをイメージしています。火山灰が降ると、黒い道路は白くなり、白い車は黒くなり、黒い車は白くなります。町のコントラストが全体的に控えめになるのです。

使用する色は、黒、白、灰、赤、橙、黄、青、緑を明るくしたり暗くしたりしています。

カラーコードはこのようになりました。

```lua
local colors = {}

-- Status color
colors.none = "NONE"
colors.cleared = "cleared"
colors.error = "#8f3231"
colors.warn = "#C7A252"
colors.info = "#CEB4A8"
colors.hint = "#717375"

-- style
colors.bold = "bold"
colors.underline = "underline"
colors.undercurl = "undercurl"

-- Using color
colors.black = "#22272e"
colors.black_blue = "#1D3A64"

colors.white = "#ebdbb2"
colors.dark_white = "#8b9aaa"

colors.dark_gray = "#7D7D7D"
colors.light_gray = "#989B9D"
colors.winter_gray = "#2D333B"
colors.inactivegray = "#7c6f64"

colors.green = "#3DA163"
colors.dark_green = "#658D50"

colors.blue = "#8395A5"
colors.dark_blue = "#4F6981"
colors.light_blue = "#82ade0"

colors.dark_red = "#A77169"

colors.orange = "#E38D2C"
colors.dark_orange = "#97812C"

colors.yellow = "#E3D92C"
colors.dark_yellow = "#B3A278"
colors.dark_yellow_green = "#A7B383"

colors.cyan = "#2BB6BA"
colors.dark_cyan = "#3B7B7D"
colors.light_cyan = "#5F9D9C"

return colors
```

<HL el="h2" text="サポートしているプラグイン" />

v0.1.0時点では、私が使用しているプラグインで影響のある [lualine.nvim](https://github.com/nvim-lualine/lualine.nvim) にのみ対応しています。  
他のプラグインも随時対応したいと考えています。

<Img src="/images/neovim/002-sakurajima-theme-introduction/sakurajima-typescript.png" alt="coding screen" />

<HL el="h2" text="どんな見た目か" />

[README](https://github.com/Daiki48/sakurajima.nvim/blob/main/README.md) でもスクリーンショットを何枚か貼っています。

ダッシュボードはこんな感じです。

<Img src="/images/neovim/002-sakurajima-theme-introduction/sakurajima-dashboard.png" alt="dashboard" />

LSPの補完で使用しているcoc.nvimはこんな感じです。

<Img src="/images/neovim/002-sakurajima-theme-introduction/sakurajima-coc.png" alt="coc.nvim" />

<HL el="h2" text="さいごに" />

個人ブログなのを理由に好き放題書いてしまったため、文章がおかしいかもしれません。  
coolnessFlair.vimはサポートするプラグインを増やしたりといったメンテナンスを怠っていたため、sakurajima.nvimは細々とメンテナンスをしていきたいと思います。
