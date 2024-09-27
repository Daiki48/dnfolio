---
title: お世話になっているNeovimプラグイン達-2024年6月
description: Neovimでお世話になっているプラグインをまとめます。定期的に見直してるので2024年6月現在の情報です。
createdAt: '2024-06-07'
tags:
  - Neovim
published: true
---

<script>
  import HL from '$components/modules/HL.svelte';
  import Img from '$components/modules/Img.svelte';
</script>

<HL el="h2" text="はじめに" />

Neovimの設定は [dotfiles](https://github.com/Daiki48/dotfiles) で管理しています。

私は、普段からNeovimを使用しています。使っている環境は、仕事用のwindowsPCとプライベートではManjaro KDE Plasma v6です。  
windowsPCではPowershellを使用して、Manjaroではzshを使用しています。  
ちなみに、windowsPCのWSL2でもかつて使用していましたが、メモリとCPU使用率が激しすぎて辞めました。.wslconfigで制御したかったのですが、私のWSL2勉強意欲がそこまで湧かず断念...といった感じです。あらかじめご了承ください。

使用しているプラグインは21個と、そこまで多くはないです。今回は含まれていませんが、`ddc.vim` や `ddu.vim` を使用していた頃は50個〜100個ほどのプラグインを使用することになっていました。  
プラグインの選定基準として、なるべくLua製のものを選んでいます。(ルアルアしたいだけ)

<HL el="h2" text="プラグインマネージャー" />

[folke/lazy.nvim](https://github.com/folke/lazy.nvim)

Lua製Neovimをたくさん開発されているfolke氏によるプラグインマネージャーです。

長らく [Shougo/dein.vim](https://github.com/Shougo/dein.vim) を使用してきましたが、開発が [Shougo/dpp.vim](https://github.com/Shougo/dpp.vim) へと移行したようです。  
[作者の記事](https://zenn.dev/shougo/articles/dpp-vim-beta#%E5%A7%8B%E3%82%81%E3%81%AB) で確認出来ます。

`dpp.vim` の設定は楽しそうで、興味はあります。テキストエディタに時間を割けるようになったら触ってみたいと思っています。

<HL el="h3" text="lazy.nvimの設定" />

詳しい設定は [README](https://github.com/folke/lazy.nvim/blob/main/README.md) をご覧ください。

以下は私の設定です。ほとんどREADMEの内容です。

```lua
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
	vim.fn.system({
		"git",
		"clone",
		"--filter=blob:none",
		"https://github.com/folke/lazy.nvim.git",
		"--branch=stable", -- latest stable release
		lazypath,
	})
end
vim.opt.rtp:prepend(lazypath)

local plugins = require("plugins")

local opts = {
	defaults = {
		lazy = true,
	},
	performance = {
		cache = {
			enabled = true,
		},
	},
	dev = {
		path = "/mnt/sabrent/dev/nvim-plugin-dev/Daiki48",
	},
}

require("lazy").setup(plugins, opts)
```

`Lazy` コマンドで下記のようにプラグイン管理画面を表示して使っています。  
それぞれプラグインの横に、どのタイミングで起動するか、どのプラグインに依存しているかなどが表示されていて分かりやすいです。

<Img src="/images/neovim/003-mysetup-202406/lazy-nvim.webp" alt="lazy.nvim" />

<HL el="h2" text="カラースキーム" />

[Daiki48/sakurajima.nvim](https://github.com/Daiki48/sakurajima.nvim)

自作カラースキームです。

作ったばかりということもあり、Neovimを開く度にワクワクしています。

nvim-treesitter対応を進めています。

試しに使って頂けるだけでも嬉しいです。

<Img src="/images/neovim/003-mysetup-202406/sakurajima-typescript.webp" alt="sakurajima.nvim" />

<HL el="h3" text="sakurajima.nvimの設定" />

```lua
return {
	{
		"Daiki48/sakurajima.nvim",
		lazy = false,
		branch = "main",
		config = function()
			vim.cmd([[colorscheme sakurajima]])
		end,
	},
}
```

<HL el="h2" text="ファイラー" />

[nvim-tree/nvim-tree.lua](https://github.com/nvim-tree/nvim-tree.lua)

Lua製ファイラー。  
ファイル名変更やファイル作成、ファイル削除など基本的なCRUD対応。
自動同期なので、別ウィンドウなどでファイルを追加した場合にも反映してくれます。  
Gitの差分表示などにも対応しています。便利。

<Img src="/images/neovim/003-mysetup-202406/nvim-tree-lua.webp" alt="nvim-tree.lua" />

以前までは標準のnetrwや[vim-fern](https://github.com/lambdalisue/vim-fern)、 [ddu.vim](https://github.com/Shougo/ddu.vim) を使用していましたが、ルアルアしたくなったタイミングでnvim-tree.luaへ移行しました。  
vim-fernはVim Script製で、操作感はnvim-tree.luaと似たような感じの印象です。  
ddu.vimはTypeScript製で、ddu-ui-filerをUIにファイラーとして使用出来ます。  
[作者の記事](https://zenn.dev/shougo/articles/ddu-ui-filer) で詳しく解説されています。ファイラープラグインの歴史がとてもおもしろいです。

<HL el="h3" text="nvim-tree.luaの設定" />

[README](https://github.com/nvim-tree/nvim-tree.lua/blob/master/README.md) を参考に設定しました。

```lua
-- recommended settings from nvim-tree documentation
vim.g.loaded_netrw = 1
vim.g.loaded_netrwPlugin = 1

require("nvim-tree").setup({
	view = {
		width = 40,
		relativenumber = true,
		float = {
			enable = false,
			quit_on_focus_loss = true,
			open_win_config = {
				relative = "editor",
				border = "rounded",
				width = 30,
				height = 30,
				row = 1,
				col = 1,
			},
		},
	},
	-- change folder arrow icons
	renderer = {
		indent_markers = {
			enable = true,
		},
		icons = {
			glyphs = {
				folder = {
					arrow_closed = "",
					arrow_open = "",
				},
			},
		},
	},
	-- disable window_picker for
	-- explorer to work well with
	-- window splits
	actions = {
		open_file = {
			window_picker = {
				enable = false,
			},
		},
	},
	filters = {
		custom = { ".DS_Store" },
	},
	git = {
		ignore = false,
	},
})

-- set keymaps
local keymap = vim.keymap -- for conciseness

keymap.set("n", ";ee", "<cmd>NvimTreeToggle<CR>", { desc = "Toggle file explorer" }) -- toggle file explorer
keymap.set("n", ";ef", "<cmd>NvimTreeFindFileToggle<CR>", { desc = "Toggle file explorer on current file" }) -- toggle file explorer on current file
keymap.set("n", ";ec", "<cmd>NvimTreeCollapse<CR>", { desc = "Collapse file explorer" }) -- collapse file explorer
keymap.set("n", ";er", "<cmd>NvimTreeRefresh<CR>", { desc = "Refresh file explorer" }) -- refresh file explorer
```

<HL el="h2" text="ファジーファインダー" />

[nvim-telescope/telescope.nvim](https://github.com/nvim-telescope/telescope.nvim)

Lua製のファジーファインダー。  
私は、あいまい検索とgrepで使用しています。

windowsで使用する際は若干重いです。

<Img src="/images/neovim/003-mysetup-202406/telescope-nvim.webp" alt="telescope.nvim" />

拡張機能を作ることが出来るようです。

<HL el="h3" text="telescope.nvimの設定" />

```lua
require("telescope").setup({
	pickers = {
		find_files = {
			theme = "ivy",
		},
	},
})

local builtin = require("telescope.builtin")
vim.keymap.set("n", ";ff", builtin.find_files, {})
vim.keymap.set("n", ";fg", builtin.live_grep, {})
vim.keymap.set("n", ";fb", builtin.buffers, {})
vim.keymap.set("n", ";fh", builtin.help_tags, {})
```

<HL el="h2" text="補完" />

[neoclide/coc.nvim](https://github.com/neoclide/coc.nvim)

コードの補完では `coc.nvim` を使用しています。

<Img src="/images/neovim/003-mysetup-202406/sakurajima-coc.webp" alt="coc.nvim" />

最近だと [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig) を使うことが多いと思います。私もNeovim使い始めた頃は [ddc.vim](https://github.com/Shougo/ddc.vim) と組み合わせて使っていました。一時期は [nvim-cmp](https://github.com/hrsh7th/nvim-cmp) と組み合わせて使ってみたりもしていました。  
coc.nvimよりも柔軟に設定することが出来ました。  
当時はNeovimの設定が楽しかったので多くのプラグインを管理出来ていましたが、最近はdotfilesのメンテナンスを高頻度で行えないためcoc.nvimへ移行しました。

coc.nvimは、自分で細かい設定をする必要はありません。 `CocInstall` でLSPをインストールすると動きます。設定が必要な言語は `coc-settings.json` に設定を書くと動きます。  
とても使いやすいです。

<HL el="h3" text="coc.nvimの設定" />

```lua
vim.opt.backup = false
vim.opt.writebackup = false

vim.opt.updatetime = 300

vim.opt.signcolumn = "yes"

local keyset = vim.keymap.set
function _G.check_back_space()
	local col = vim.fn.col(".") - 1
	return col == 0 or vim.fn.getline("."):sub(col, col):match("%s") ~= nil
end

local opts = { silent = true, noremap = true, expr = true, replace_keycodes = false }
keyset("i", "<TAB>", 'coc#pum#visible() ? coc#pum#next(1) : v:lua.check_back_space() ? "<TAB>" : coc#refresh()', opts)
keyset("i", "<S-TAB>", [[coc#pum#visible() ? coc#pum#prev(1) : "\<C-h>"]], opts)

keyset("i", "<cr>", [[coc#pum#visible() ? coc#pum#confirm() : "\<C-g>u\<CR>\<c-r>=coc#on_enter()\<CR>"]], opts)

keyset("i", "<c-j>", "<Plug>(coc-snippets-expand-jump)")
keyset("i", "<c-space>", "coc#refresh()", { silent = true, expr = true })

keyset("n", "[g", "<Plug>(coc-diagnostic-prev)", { silent = true })
keyset("n", "]g", "<Plug>(coc-diagnostic-next)", { silent = true })

keyset("n", "gd", "<Plug>(coc-definition)", { silent = true })
keyset("n", "gy", "<Plug>(coc-type-definition)", { silent = true })
keyset("n", "gi", "<Plug>(coc-implementation)", { silent = true })
keyset("n", "gr", "<Plug>(coc-references)", { silent = true })

function _G.show_docs()
	local cw = vim.fn.expand("<cword>")
	if vim.fn.index({ "vim", "help" }, vim.bo.filetype) >= 0 then
		vim.api.nvim_command("h " .. cw)
	elseif vim.api.nvim_eval("coc#rpc#ready()") then
		vim.fn.CocActionAsync("doHover")
	else
		vim.api.nvim_command("!" .. vim.o.keywordprg .. " " .. cw)
	end
end
keyset("n", "K", "<CMD>lua _G.show_docs()<CR>", { silent = true })

vim.api.nvim_create_augroup("CocGroup", {})
vim.api.nvim_create_autocmd("CursorHold", {
	group = "CocGroup",
	command = "silent call CocActionAsync('highlight')",
	desc = "Highlight symbol under cursor on CursorHold",
})

keyset("n", "<leader>rn", "<Plug>(coc-rename)", { silent = true })

keyset("x", "<leader>f", "<Plug>(coc-format-selected)", { silent = true })
keyset("n", "<leader>f", "<Plug>(coc-format-selected)", { silent = true })

vim.api.nvim_create_autocmd("FileType", {
	group = "CocGroup",
	pattern = "typescript,json",
	command = "setl formatexpr=CocAction('formatSelected')",
	desc = "Setup formatexpr specified filetype(s).",
})

vim.api.nvim_create_autocmd("User", {
	group = "CocGroup",
	pattern = "CocJumpPlaceholder",
	command = "call CocActionAsync('showSignatureHelp')",
	desc = "Update signature help on jump placeholder",
})

local opts = { silent = true, nowait = true }
keyset("x", "<leader>a", "<Plug>(coc-codeaction-selected)", opts)
keyset("n", "<leader>a", "<Plug>(coc-codeaction-selected)", opts)

keyset("n", "<leader>ac", "<Plug>(coc-codeaction-cursor)", opts)
keyset("n", "<leader>as", "<Plug>(coc-codeaction-source)", opts)
keyset("n", "<leader>qf", "<Plug>(coc-fix-current)", opts)

keyset("n", "<leader>re", "<Plug>(coc-codeaction-refactor)", { silent = true })
keyset("x", "<leader>r", "<Plug>(coc-codeaction-refactor-selected)", { silent = true })
keyset("n", "<leader>r", "<Plug>(coc-codeaction-refactor-selected)", { silent = true })

keyset("n", "<leader>cl", "<Plug>(coc-codelens-action)", opts)

keyset("x", "if", "<Plug>(coc-funcobj-i)", opts)
keyset("o", "if", "<Plug>(coc-funcobj-i)", opts)
keyset("x", "af", "<Plug>(coc-funcobj-a)", opts)
keyset("o", "af", "<Plug>(coc-funcobj-a)", opts)
keyset("x", "ic", "<Plug>(coc-classobj-i)", opts)
keyset("o", "ic", "<Plug>(coc-classobj-i)", opts)
keyset("x", "ac", "<Plug>(coc-classobj-a)", opts)
keyset("o", "ac", "<Plug>(coc-classobj-a)", opts)

keyset("n", "<C-s>", "<Plug>(coc-range-select)", { silent = true })
keyset("x", "<C-s>", "<Plug>(coc-range-select)", { silent = true })

vim.api.nvim_create_user_command("Format", "call CocAction('format')", {})

vim.api.nvim_create_user_command("Fold", "call CocAction('fold', <f-args>)", { nargs = "?" })

vim.api.nvim_create_user_command("OR", "call CocActionAsync('runCommand', 'editor.action.organizeImport')", {})

vim.opt.statusline:prepend("%{coc#status()}%{get(b:,'coc_current_function','')}")

local opts = { silent = true, nowait = true }
keyset("n", "<space>a", ":<C-u>CocList diagnostics<cr>", opts)
keyset("n", "<space>l", ":<C-u>CocList extensions<cr>", opts)
keyset("n", "<space>c", ":<C-u>CocList commands<cr>", opts)
keyset("n", "<space>o", ":<C-u>CocList outline<cr>", opts)
keyset("n", "<space>s", ":<C-u>CocList -I symbols<cr>", opts)
keyset("n", "<space>j", ":<C-u>CocNext<cr>", opts)
keyset("n", "<space>k", ":<C-u>CocPrev<cr>", opts)
keyset("n", "<space>p", ":<C-u>CocListResume<cr>", opts)

local function switch_coc_ts()
	local path = vim.fn.expand("%:p:h")
	if path == "" then
		path = "."
	end

	if vim.fn.empty(vim.fn.finddir("node_modules", path .. ";")) == 1 then
		vim.fn["coc#config"]("deno.enable", true)
		vim.fn["coc#config"]("tsserver.enable", false)
	else
		vim.fn["coc#config"]("deno.enable", false)
		vim.fn["coc#config"]("tsserver.enable", true)
	end
end

vim.api.nvim_create_autocmd("FileType", {
	pattern = { "typescript", "typescript.tsx" },
	callback = switch_coc_ts,
	once = true,
})

vim.keymap.set("n", "<space>e", ":<C-u>CocCommand document.showIncomingCalls<CR>", { silent = true, noremap = true })

vim.api.nvim_create_user_command("Prettier", function()
	vim.fn.CocAction("runCommand", "prettier.formatFile")
end, {})
```

グローバルな `coc-settings.json` は下記のように設定しています。

`tsserver` と `deno` のLSPはどちらも無効にしています。そして、それぞれのプロジェクト配下で有効にしています。

```json
{
	"suggest.noselect": true,
	"suggest.preferCompleteThanJumpPlaceholder": true,
	"suggest.floatConfig": {
		"border": true
	},
	"diagnostic.floatConfig": {
		"border": true
	},
	"signature.floatConfig": {
		"border": true
	},
	"hover.floatConfig": {
		"border": true
	},
	"Lua.diagnostics.globals": ["vim"],
	"svelte.enable-ts-plugin": true,
	"tsserver.enable": false,
	"deno.enable": false
}
```

<HL el="h2" text="特定の位置にジャンプ" />

[smoka7/hop.nvim](https://github.com/smoka7/hop.nvim)

単語ごとにマーカーを付与して、その位置にカーソルジャンプ出来ます。  
目線を動かさずに移動出来るのが便利で、私のコーディングでは多用しています。

<Img src="/images/neovim/003-mysetup-202406/sakurajima-hop.webp" alt="hop.nvim" />

<HL el="h3" text="hop.nvimの設定" />

```lua
-- place this in one of your configuration file(s)
local hop = require("hop")
local directions = require("hop.hint").HintDirection

hop.setup({
	multi_windows = true,
})

vim.keymap.set("n", "<space>f", "<cmd>HopWord<CR>")

-- vim.keymap.set('n', 'f', function()
--   hop.hint_words({ direction = directions.AFTER_CURSOR, current_line_only = true })
-- end, {remap=true})
vim.keymap.set("n", "F", function()
	hop.hint_char1({ direction = directions.BEFORE_CURSOR, current_line_only = true })
end, { remap = true })
vim.keymap.set("n", "t", function()
	hop.hint_char1({ direction = directions.AFTER_CURSOR, current_line_only = true, hint_offset = -1 })
end, { remap = true })
vim.keymap.set("n", "T", function()
	hop.hint_char1({ direction = directions.BEFORE_CURSOR, current_line_only = true, hint_offset = 1 })
end, { remap = true })
```

<HL el="h2" text="ステータスバー" />

[nvim-lualine/lualine.nvim](https://github.com/nvim-lualine/lualine.nvim)

ステータスバー関連のプラグインは他にもたくさんありますが、タブラインのカスタマイズも可能な `lualine.nvim` を使用しています。  
ちなみに、 [Daiki48/sakurajima.nvim](https://github.com/Daiki48/sakurajima.nvim) は `lualine.nvim` に対応しています。

<HL el="h3" text="lualine.nvimの設定" />

```lua
	-- lualine
	local status, lualine = pcall(require, "lualine")
	if not status then
		return
	end

	-- vim_mode select function
	local function vim_mode()
		local map = {
			["n"] = "N",
			["no"] = "O",
			["nov"] = "O",
			["noV"] = "O",
			["no\\22"] = "O",
			["niI"] = "N",
			["niR"] = "N",
			["niV"] = "N",
			["nt"] = "N",
			["v"] = "V",
			["vs"] = "V",
			["V"] = "L",
			["Vs"] = "L",
			["\\22"] = "V",
			["\\22s"] = "VB",
			["s"] = "S",
			["S"] = "SL",
			["\\19"] = "SB",
			["i"] = "I",
			["ic"] = "I",
			["ix"] = "I",
			["R"] = "R",
			["Rc"] = "R",
			["Rx"] = "R",
			["Rv"] = "VR",
			["Rvc"] = "VR",
			["Rvx"] = "VR",
			["c"] = "C",
			["cv"] = "X",
			["ce"] = "X",
			["r"] = "P",
			["rm"] = "M",
			["r?"] = "F",
			["!"] = "S",
			["t"] = "T",
		}
		local mode_code = vim.api.nvim_get_mode().mode
		local mode_sign = map[mode_code]
		if mode_sign == nil then
			return mode_code
		else
			return mode_sign
		end
	end

	lualine.setup({
		options = {
			icons_enabled = true,
			theme = "sakurajima",
			component_separators = { left = "", right = "" },
			section_separators = { left = "", right = "" },
			disabled_filetypes = {},
		},
		sections = {
			lualine_a = {
				{ vim_mode },
				"g:coc_status",
			},
			lualine_b = {
				"g:coc_git_status",
				"branch",
				{
					"diff",
					diff_color = {
						added = {
							fg = "#DA523A",
						},
						modified = {
							fg = "#E8C473",
						},
						removed = {
							fg = "#659AD2",
						},
					},
				},
				{
					"diagnostics",
					source = { coc },
					diagnostics_color = {
						error = {
							fg = "#8f3231",
						},
						warn = {
							fg = "#C7A252",
						},
						info = {
							fg = "#E6D2C9",
						},
						hint = {
							fg = "#717375",
						},
					},
				},
			},
			lualine_c = {
				{
					"filename",
					path = 1,
				},
			},
			lualine_x = { "encoding", "filetype" },
			lualine_y = { "progress" },
			lualine_z = { "location" },
		},
		inactive_sections = {
			lualine_a = {},
			lualine_b = {},
			lualine_c = { "filename" },
			lualine_x = { "location" },
			lualine_y = {},
			lualine_z = {},
		},
		tabline = {
			lualine_a = { "buffers" },
			lualine_b = {},
			lualine_c = {},
			lualine_x = {},
			lualine_y = {},
			lualine_z = { "tabs" },
		},
		extensions = {},
	})
```

<HL el="h2" text="treesitter" />

[nvim-treesitter/nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter)

構文解析してくれるプラグインです。設定によっては、コードのハイライトやインデントなど細かい制御が出来るようになります。

設定する際は、 `nvim-treesitter.configs` からセットアップを行います。READMEでもそのように書いていました。

<HL el="h3" text="nvim-treesitterの設定" />

```lua
require("nvim-treesitter.configs").setup({
	-- A list of parser names, or "all" (the five listed parsers should always be installed)
	ensure_installed = { "c", "lua", "vim", "vimdoc", "query", "rust", "typescript", "svelte" },

	-- Install parsers synchronously (only applied to `ensure_installed`)
	sync_install = false,

	-- Automatically install missing parsers when entering buffer
	-- Recommendation: set to false if you don't have `tree-sitter` CLI installed locally
	auto_install = true,

	-- List of parsers to ignore installing (or "all")
	ignore_install = { "javascript" },

	---- If you need to change the installation directory of the parsers (see -> Advanced Setup)
	-- parser_install_dir = "/some/path/to/store/parsers", -- Remember to run vim.opt.runtimepath:append("/some/path/to/store/parsers")!

	highlight = {
		-- disable = true,
		-- enable = true,

		-- Setting this to true will run `:h syntax` and tree-sitter at the same time.
		-- Set this to `true` if you depend on 'syntax' being enabled (like for indentation).
		-- Using this option may slow down your editor, and you may see some duplicate highlights.
		-- Instead of true it can also be a list of languages
		additional_vim_regex_highlighting = false,
	},

	-- https://github.com/windwp/nvim-ts-autotag/commit/6e9742a006ae69c015e6dc1ed9b477033193778b
	-- "If you are setting up via nvim-treesitter.configs it has been deprecated! Please migrate to the new way. It will be removed in 1.0.0."
	-- autotag = {
	-- 	enable = true
	-- }
})
```

他にもありますが、以上がよく利用しているプラグインです。

私の普段開発しているSvelteの環境構築についても別記事で書きたいと思います。
