---
title: Customised VitePress search
description: VitePress allows you to use the MiniSearch library for search functions. This time, I set the search field to Japanese.
tags:
  - Tech
  - VitePress
  - MiniSearch
---

## Default config

VitePress can implement the **MiniSearch** search function by setting the following in `.vitepress/config.ts` .

[Default theme local search | VitePress](https://vitepress.dev/reference/default-theme-search#local-search)

```ts
export default defineConfig({
    themeConfig: {
        ...
        search: {
            provider: "local",
        },
    }
})
```

This time, change the placefolder in the search field,
which is displayed as **'Search'** by default, to Japanese.

## Confirm default theme

I use the default VitePress theme.

[MiniSearch options in default theme](https://vitepress.dev/reference/default-theme-search#minisearch-options)

Check [MiniSearch official](https://lucaong.github.io/minisearch/index.html)

## Confirm `LocalSearchOptions`

```ts
export default defineConfig({
    themeConfig: {
        ...
        search: {
            provider: "local",
            detailedView: true, // https://github.com/vuejs/vitepress/blob/8451cd8ceeac35c5f29f34f24b6137b2f96d4294/types/default-theme.d.ts#L401
            translations: {}, // https://github.com/vuejs/vitepress/blob/8451cd8ceeac35c5f29f34f24b6137b2f96d4294/types/default-theme.d.ts#L408
        },
    }
})
```

[LocalSearchOptions in default theme](https://github.com/vuejs/vitepress/blob/8451cd8ceeac35c5f29f34f24b6137b2f96d4294/types/default-theme.d.ts#L387)

## Confirm `LocalSearchTranslations`

[LocalSearchTranslations in local-search.d.ts](https://github.com/vuejs/vitepress/blob/8451cd8ceeac35c5f29f34f24b6137b2f96d4294/types/local-search.d.ts#L1)

```ts
export interface LocalSearchTranslations {
    button?: ButtonTranslations
    modal?: ModalTranslations
}
```

## Confirm `ButtonTranslations`

[ButtonTranslations in local-search.d.ts](https://github.com/vuejs/vitepress/blob/8451cd8ceeac35c5f29f34f24b6137b2f96d4294/types/local-search.d.ts#L6)

```ts
export interface ButtonTranslations {
    buttonText?: string
    buttonAriaLabel?: string
}
```

## Confirm buttonText and buttonAriaLabel in the default theme

Check how `buttonText` and `buttonAriaLabel` are used in the default theme.

[VPNavBarSearchButton.vue](https://github.com/vuejs/vitepress/blob/8451cd8ceeac35c5f29f34f24b6137b2f96d4294/src/client/theme-default/components/VPNavBarSearchButton.vue#L6)

```ts
// Button-Translations
const defaultTranslations: { button: ButtonTranslations } = {
    button: {
        buttonText: 'Search',
        buttonAriaLabel: 'Search'
    }
}
```

Updating `buttonText` .

```ts
export default defineConfig({
    themeConfig: {
        ...
        search: {
            provider: "local",
            detailedView: true, // https://github.com/vuejs/vitepress/blob/8451cd8ceeac35c5f29f34f24b6137b2f96d4294/types/default-theme.d.ts#L401
            translations: {
                button: {
                    buttonText: "検索する",
                },
            }, // https://github.com/vuejs/vitepress/blob/8451cd8ceeac35c5f29f34f24b6137b2f96d4294/types/default-theme.d.ts#L408
        },
    }
})
```

With these changes, I was able to customise the placefolder in the search field.
