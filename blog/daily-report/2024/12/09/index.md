---
title: 9 December 2024 Daily Report
description: This is the daily report for 9 December 2024.
tags:
  - Diary
---

## I'm back to coc.nvim

coc.nvim is best.
I have moved to Neovim builtin LSP.

## For me

I believe the current mainstream method is to use Neovim builtin LSP.
I went back to coc.nvim because...

- Fewer plugins to dependencies.
- Easy to set up.
- Can clearly indicate which LSP are used in a development project.

## Fewer plugins to dependencies

When using Neovim builtin LSP, plugins such as `nvim-lspconfig`, `nvim-cmp`, and `mason.nvim` are required.
In other words, users can customize it freely.
For me, this is a disadvantage.
I would like to have less description of my dotfiles.
Because I want to reduce the number of times I have to maintain them.

`coc.nvim` depends on `node.js`.
I always install `node.js`, so no problem.

## Easy to set up

In `coc.nvim`, LSP can be used immediately after installing LSP with `:CocInstall`.

[Implemented coc extensions](https://github.com/neoclide/coc.nvim/wiki/Using-coc-extensions#implemented-coc-extensions)

## Can clearly indicate which LSP are used in a development project

Individual settings can be configured explicitly in `coc-settings.json`.
Project-specific settings can be configured in `.vim/coc-settings.json`.

This experience with builtin LSP made me realize how wonderful `coc.nvim` is.
