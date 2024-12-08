---
title: 8 December 2024 Daily Report
description: This is the daily report for 8 December 2024.
tags:
  - Diary
---

## Migrated to mason-lspconfig

I have been rewriting my Neovim setup since yesterday.
At first, I was trying to set it up with `neovim/nvim-lspconfig` only.
However, the TypeScript settings do not work properly.
I manage my Neovim LSP with `mason.nvim`.
And a plugin exists to easily configure `mason.nvim` and `nvim-lspconfig`.
That is `mason-lspconfig.nvim`.
I was able to set up.
However, special language-specific settings may be required.
That setup will be done now.
