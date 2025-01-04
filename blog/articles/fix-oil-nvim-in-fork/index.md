---
title: Fix oil.nvim in fork
description: I resolved the error with a forked oil.nvim.
tags:
  - Tech
  - Neovim
  - Lua
  - Oil
---

## Error

```shell
...local/nvim-data/lazy/oil.nvim/lua/oil/adapters/files.lua:59: attempt to index local 'meta' (a nil value)
```

## Resolve

The error location was checked.

59 lines in `lua/oil/adapters/files.lua`.

[files.lua#59 | lua/oil/adapters](https://github.com/stevearc/oil.nvim/blob/c6a39a69b2df7c10466f150dde0bd23e49c1fba3/lua/oil/adapters/files.lua#L59)

```lua
file_columns.size = {
  require_stat = true,

  render = function(entry, conf)
    local meta = entry[FIELD_META]
    local stat = meta.stat
    if not stat then
      return columns.EMPTY
    end
    if stat.size >= 1e9 then
      return string.format("%.1fG", stat.size / 1e9)
    elseif stat.size >= 1e6 then
      return string.format("%.1fM", stat.size / 1e6)
    elseif stat.size >= 1e3 then
      return string.format("%.1fk", stat.size / 1e3)
    else
      return string.format("%d", stat.size)
    end
  end,

  get_sort_value = function(entry)
    local meta = entry[FIELD_META]
    local stat = meta.stat
    if stat then
      return stat.size
    else
      return 0
    end
  end,

  parse = function(line, conf)
    return line:match("^(%d+%S*)%s+(.*)$")
  end,
}
```

Added code to perform meta initialization.

```lua
file_columns.size = {
  require_stat = true,

  render = function(entry, conf)
    local meta = entry[FIELD_META]
    local stat = meta.stat
    if not stat then
      return columns.EMPTY
    end
    if stat.size >= 1e9 then
      return string.format("%.1fG", stat.size / 1e9)
    elseif stat.size >= 1e6 then
      return string.format("%.1fM", stat.size / 1e6)
    elseif stat.size >= 1e3 then
      return string.format("%.1fk", stat.size / 1e3)
    else
      return string.format("%d", stat.size)
    end
  end,

  get_sort_value = function(entry)
    local meta = entry[FIELD_META]
    local stat = meta.stat
    if stat then
      return stat.size
    else
      return 0
    end
  end,

  parse = function(line, conf)
    return line:match("^(%d+%S*)%s+(.*)$")
  end,
}
```

This corrective code resolved the error.

## A PR had already been created

[Fix: Resolve 'attempt to index local 'meta' (a nil value)' #547](https://github.com/stevearc/oil.nvim/pull/547)

This PR modifies `lua/oil/columns.lua`.

Until this PR is merged, use the forked project.
