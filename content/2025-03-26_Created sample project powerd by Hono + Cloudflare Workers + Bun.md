+++
title = "Created sample project powerd by Hono + Cloudflare Workers + Bun"
slug = "hono-cloudflare-workers-bun"
description = "First time using Cloudflare Workers and Hono. I am very easy to use."
draft = false
[taxonomies]
tags = ["Cloudflare Workers", "Hono", "Bun"]
languages = ["en"]
+++

I am writing to express my interest in developing a web application.
To this end, I recently had the opportunity to explore the tutorials for [Cloudflare Workers](https://workers.cloudflare.com/) and [Hono](https://hono.dev),
as they have garnered my attention recently.
My work throughout this process has been conducted entirely in accordance with the Hono documentation,
which I found to be very clear and easy to understand.

While I currently utilize [Cloudflare Pages](https://www.cloudflare.com/developer-platform/products/pages/) for the deployment of [my personal website](https://dnfolio.me),
I had not previously used Cloudflare Workers.
Based on my recent experience, I believe that Hono will be my first choice for developing web applications using Cloudflare Workers in the future.
Thank you for providing such excellent resources.

**I will now proceed to document my work record.**

## Create project

Ref: [Getting started | Cloudflare Workers](https://hono.dev/docs/getting-started/cloudflare-workers)

Using [Bun](https://bun.sh).

```sh
bun create hono@latest cloudflare-workers-hono-bun

create-hono version 0.15.4
✔ Using target directory … cloudflare-workers-hono-bun
? Which template do you want to use? cloudflare-workers
? Do you want to install project dependencies? yes
? Which package manager do you want to use? bun
✔ Cloning the template
✔ Installing project dependencies
🎉 Copied project files
Get started with: cd cloudflare-workers-hono-bun
```

- Template: `cloudflare-workers`
- Package manager: `bun`

## Install dependencies

Move to created directory.

```sh
cd cloudflare-workers-hono-bun
```

Install dependencies.

```sh
bun install
```

## Edit `src/index.ts`

Initial code content when the project is created.

```ts
import { Hono } from 'hono'

const app = new Hono()

app.get('/', (c) => {
  return c.text('Hello Hono!')
})

export default app
```

For example, try changing only the content of the output.

```sh
import { Hono } from 'hono'

const app = new Hono()

app.get('/', (c) => {
  return c.text('Hello Cloudflare Workers!') // here
})

export default app
```

## Starting local server

```sh
bun run dev
```

Then, this message was output.

```sh
 ⛅️ wrangler 3.114.2 (update available 4.4.0)
-------------------------------------------------------

▲ [WARNING] The version of Wrangler you are using is now out-of-date.

  Please update to the latest version to prevent critical errors.
  Run `npm install --save-dev wrangler@4` to update to the latest version.
  After installation, run Wrangler with `npx wrangler`.
```

Humm, checking `package.json`.

```json
{
  "name": "cloudflare-workers-hono-bun",
  "scripts": {
    "dev": "wrangler dev",
    "deploy": "wrangler deploy --minify"
  },
  "dependencies": {
    "hono": "^4.7.5"
  },
  "devDependencies": {
    "@cloudflare/workers-types": "^4.20250214.0",
    "wrangler": "^3.109.2"
  }
}
```

Update the version of `wrangler` to the latest version.

```sh
bun add --dev wrangler@latest
bun add v1.2.6 (8ebd5d53)

installed wrangler@4.4.0 with binaries:
 - wrangler
 - wrangler2

9 packages installed [16.59s]
```

One more checking.

```json
{
  "name": "cloudflare-workers-hono-bun",
  "scripts": {
    "dev": "wrangler dev",
    "deploy": "wrangler deploy --minify"
  },
  "dependencies": {
    "hono": "^4.7.5"
  },
  "devDependencies": {
    "@cloudflare/workers-types": "^4.20250214.0",
    "wrangler": "^4.4.0" // Checking here!
  }
}
```

Run again.

```sh
bun run dev
$ wrangler dev

Cloudflare collects anonymous telemetry about your usage of Wrangler. Learn more at https://github.com/cloudflare/workers-sdk/tree/main/packages/wrangler/telemetry.md

 ⛅️ wrangler 4.4.0
------------------

No bindings found.
⎔ Starting local server...
[wrangler:inf] Ready on http://localhost:8787
[wrangler:inf] GET / 200 OK (3ms)
[wrangler:inf] GET /favicon.ico 404 Not Found (2ms)
╭──────────────────────────────────────────────────────────────────────────────────────────────────╮
│  [b] open a browser, [d] open devtools, [l] turn off local mode, [c] clear console, [x] to exit  │
╰──────────────────────────────────────────────────────────────────────────────────────────────────╯
```

Successfuly!

Pushing `b` button.
Then, opened the browser!

## Deploy

```sh
bun run deploy
```

Then, execute authentication for Cloudflare.

Finaly output.

```sh
bun run deploy
$ wrangler deploy --minify

 ⛅️ wrangler 4.4.0
------------------

Attempting to login via OAuth...
Opening a link in your default browser: https://dash.cloudflare.com/oauth2/auth? ~
Successfully logged in.
Total Upload: 19.94 KiB / gzip: 7.99 KiB
No bindings found.
Uploaded cloudflare-workers-hono-bun (4.98 sec)
Deployed cloudflare-workers-hono-bun triggers (0.85 sec)
  https://cloudflare-workers-hono-bun.daiki48-engineer.workers.dev
Current Version ID: 3998d8bb-cd9f-4196-ada6-d0bc386ecc1a
```

## (Bonus) Customize 404 page

Editing `src/index.ts`.

```ts
import { Hono } from 'hono'

const app = new Hono()

app.get('/', (c) => {
  return c.text('Hello Cloudflare Workers!')
})

app.notFound((c) => {
	return c.text('ページが見つかりませんでした。', 404) // Japanese message mean "page not found"
})

export default app
```

Checking local server.

```sh
bun run dev
```

Enter a non-existent URL and verify that it is displayed correctly.\
For example, `/about`.

## Deploy again

```sh
bun run deploy

$ wrangler deploy --minify

 ⛅️ wrangler 4.4.0
------------------

Total Upload: 20.06 KiB / gzip: 8.06 KiB
No bindings found.
Uploaded cloudflare-workers-hono-bun (2.42 sec)
Deployed cloudflare-workers-hono-bun triggers (0.32 sec)
  https://cloudflare-workers-hono-bun.daiki48-engineer.workers.dev
Current Version ID: b5b1c75f-c675-4e2f-a18a-8a7aeccef538
```

After the second time, `Opening a link in your default browser` doesn't seem to show up.

Please check the tutorial for a detailed explanation.

[Getting started | Cloudflare Workers](https://hono.dev/docs/getting-started/cloudflare-workers)

I wanted to create a web application using **Hono** and **Cloudflare Workers** !

Thank you :smile:
