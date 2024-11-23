---
title: Add @mdit/plugin-alert
description: I have added mdit-plugin-alert to my personal website.
tags:
  - tech
  - css
---

## Added `@mdit/plugin-alert`

Cool, I got the
[@mdit/plugin-alert](https://www.npmjs.com/package/@mdit/plugin-alert) plugin
from npm.\
This will let me add some fun colors to the messages on my website.

## Install

```shell
> deno add npm:@mdit/plugin-alert
```

And, setup stylesheet.

## Config for common

```css
.markdown-alert {
  padding: 8px;
  border-radius: 4px;
}
```

## Config for note

```css
.markdown-alert-note {
  background-color: rgba(150, 210, 255, 0.1);
  border: 2px solid rgba(0, 77, 135, 0.4);
}
.markdown-alert-note .markdown-alert-title {
  color: rgba(35, 66, 89, 1);
  font-weight: bold;
}
```

> [!note]
> This is note text

## Config for important

```css
.markdown-alert-important {
  background-color: rgba(241, 148, 255, 0.1);
  border: 2px solid rgba(129, 3, 148, 0.4);
}
.markdown-alert-important .markdown-alert-title {
  color: rgba(86, 47, 92, 1);
  font-weight: bold;
}
```

> [!important]
> This is important text

## Config for tip

```css
.markdown-alert-tip {
  background-color: rgba(191, 255, 192, 0.1);
  border: 2px solid rgba(1, 143, 4, 0.4);
}
.markdown-alert-tip .markdown-alert-title {
  color: rgba(45, 173, 48, 1);
  font-weight: bold;
}
```

> [!tip]
> This is tip text

## Config for warning

```css
.markdown-alert-warning {
  background-color: rgba(252, 255, 166, 0.1);
  border: 2px solid rgba(133, 92, 5, 0.4);
}
.markdown-alert-warning .markdown-alert-title {
  color: rgba(219, 150, 0, 1);
  font-weight: bold;
}
```

> [!warning]
> This is warning text

## Config for caution

```css
.markdown-alert-caution {
  background-color: rgba(245, 141, 127, 0.1);
  border: 2px solid rgba(161, 20, 2, 0.4);
}
.markdown-alert-caution .markdown-alert-title {
  color: rgba(242, 88, 68, 1);
  font-weight: bold;
}
```

> [!caution]
> This is caution text

I was on the fence about using a
[@mdit/plugin-container](https://www.npmjs.com/package/@mdit/plugin-container),
but I might need it for something else later. So I went with an
`@mdit/plugin-alert` for now.\
**markdown-it** is so flexible, it's actually kind of fun to use!
