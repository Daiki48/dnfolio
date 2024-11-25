---
title: Updated ogp setting in post page
description: I changed the handle of my Bluesky account to the domain I hold.
tags:
  - Tech
  - OGP
---

## Added page data in post markdown

Description added to the post. This description is also reflected in the OGP for each post.

```yaml
title: Changed Bluesky's handle to my domain
url: /posts/changed-bluesky-handle-to-my-domain/
description: I changed the handle of my Bluesky account to the domain I hold.
draft: false
tags:
  - life
  - bluesky
  - cloudflare
```

## Updated `src/_includes/layouts/main.tsx`

If value such as **title** , **url** , **description** , etc. exist, they are reflected in the OGP;
if they not exist, the default values are reflected.

```tsx
import { globalStyle } from "../../_styles/global.ts";

export default (
  { title, url, description, children, meta }: Lume.Data,
  _helpers: Lume.Helpers,
) => (
  <html lang={meta.lang}>
    <head>
      <meta charSet="UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      {title
        ? <title>{`${title} - ${meta.name}`}</title>
        : <title>{meta.name}</title>}
      <meta name="description" content={meta.description} />
      {meta.styles.map((style: string, index: number) => (
        <link key={index} rel="stylesheet" href={style} />
      ))}
      <style>{globalStyle}</style>
      <link rel="icon" href={meta.icon} />
      <link
        rel="stylesheet"
        href="https://maxcdn.bootstrapcdn.com/font-awesome/4.7.0/css/font-awesome.min.css"
      />
      {url
        ? <meta name="og:url" content={`${meta.ogUrl}${url}`} />
        : <meta name="og:url" content={meta.ogUrl} />}

      {title
        ? <meta name="og:title" content={`${title} - ${meta.ogTitle}`} />
        : <meta name="og:title" content={meta.ogTitle} />}
      <meta name="og:site_name" content={meta.ogSiteName} />
      <meta name="og:image" content={meta.ogImage} />

      {description
        ? <meta name="og:description" content={description} />
        : <meta name="og:description" content={meta.ogDescription} />}

      <meta name="og:type" content={meta.ogType} />
    </head>
    <body className="bg-gray-100">{children}</body>
  </html>
);
```

## Updated `src/_includes/layouts/post.tsx`

On individual pages, a description has been added under the title.

```tsx
export const title = "Dnfolio post";
export const layout = "layouts/main.tsx";

export default ({ title, children }: Lume.Data, _helpers: Lume.Helpers) => (
export default ({ title, description, children }: Lume.Data, _helpers: Lume.Helpers) => (
  <>
    <div className="flex flex-col items-center justify-center px-4">
      <a
        href="/"
        className="flex justify-center w-full max-w-2xl no-underline text-slate-500 hover:text-slate-800"
      >
        ← Home
      </a>
      <header className="text-center text-sm sm:text-xl font-bold text-wrap">
        <h1 className="leading-loose">{title}</h1>
        <p className="leading-loose">{description}</p>  { /* Here adding */ }
      </header>
    </div>

    <main className="flex justify-center items-center max-w-4xl mx-auto px-4 leading-10 sm:px-6 lg:px-8">
      {children}
    </main>
  </>
);
```

## Functions to be added in the future

- Twitter ogp
- Generate images for each post page

I want to be able to display the OGP correctly when sharing my personal website.
