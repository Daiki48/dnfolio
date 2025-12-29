+++
title = "Created share button for Bluesky"
slug = "bluesky-share-button"
description = "I created share button for Bluesky. Because I use Bluesky frequently."
draft = false
[taxonomies]
tags = ["Bluesky", "Tera"]
languages = ["en"]
+++

## Bluesky official documentation

I checked official documentation.

> The compose action URL pre-populates the compose post UI in the Bluesky Social app. A common use case for this action is to implement "Share on Bluesky" style buttons, where a brief description and URL are pre-populated in the post compose box. Similarly, "Share this Article" or "Share your Achievement".
> The web URL endpoint is https://bsky.app/intent/compose, with the HTTP query parameter text. Remember to use URL-escaping on the query parameter value, and that the post length limit on Bluesky is 300 characters (more precisely, 300 Unicode Grapheme Clusters).
> The mobile app URI endpoint is bluesky://intent/compose, with the same query parameter.

It is written.

{{ card(title="Action Intent Links | Bluesky Developer APIs", url="https://docs.bsky.app/docs/advanced-guides/intent-links") }}

## Create `templates/macros/share.html`

Zola is [Tera](https://keats.github.io/tera/).

I using [Macros](https://keats.github.io/tera/docs/#macros) function.

```html
{% macro bsky(text, url) %}
<div class="share-button">
  <a href="https://bsky.app/intent/compose?text={{text}}+{{url}}" target="_blank"><img
      src="/sns/bluesky-logo.svg"
      alt="bluesky icon"
      width="30px"
      height="auto"
    />Share on Bluesky</a>
</div>
{% endmacro bsky %}
```

## Added `share-button` class style

I added `share-button` class style in classes.scss.

Set this style to your own preference.

```css
.share-button {
  display: flex;
  justify-content: center;
  align-items: center;
}

.share-button a {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1rem;
  border: 1px solid #caccca;
  border-radius: 4px;
  padding: 10px;
  margin: 20px 0;
  box-shadow: 0 0 10px rgba(99, 113, 133, 0.4);
}

.share-button a:hover {
  box-shadow: 0 0 10px rgba(99, 113, 133, 0.8);
}
```

## Update `templates/base.html`

This code added first line.

```html
{% import "macros/share.html" as share %}
```

## Update `templates/blog-page.html`

Using `macros/share.html` macro.

```html
{{ share::bsky(text=page.title, url="https://dnfolio.dev/" ~ page.slug) }}
```

I could now add **Share on Bluesky** button :tada:
