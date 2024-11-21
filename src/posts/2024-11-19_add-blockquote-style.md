---
title: Add blockquote style
url: /posts/add-blockquote-style/
description: I have added a citation block to my personal website.
draft: false
tags:
  - tech
  - css
---

## Support blockquote

![Confirm blockquote dom](/assets/posts/add-blockquote-style/add-blockquote.webp)

> This blockquote is beautiful!!
> <cite>Daiki Nakashima</cite>

```css
blockquote {
  position: relative;
  border-left: 6px solid rgba(227, 227, 227, 1);
  border-radius: 6px;
  padding: 8px;
  background-color: rgba(227, 227, 227, 0.4);
}
blockquote cite {
  display: block;
  font-size: 0.8rem;
  text-align: right;
  color: rgba(126, 128, 130, 1);
  padding-right: 10px;
}
```

I've updated the citation style throughout the site.\
I've also experimented with adding support for the `cite` tag, but we'll see how
much use it gets.\
It's awesome seeing this site come together!
