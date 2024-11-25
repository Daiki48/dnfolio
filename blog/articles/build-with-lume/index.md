---
title: Build with lume
description: This is the story of my personal website migration from SvelteKit to Lume.
tags:
  - Tech
  - Lume
  - Deno Deploy
  - Cloudflare Pages
---

## I've decided to migrate my personal website from SvelteKit to Lume and from Cloudflare Pages to Deno Deploy

I've been working with **VitePress** and felt the itch to explore other
technologies. That's when I remembered that **Deno** had reached **version
2.0**. Oh, and I also recalled building a website with Lume a while back. For
those unfamiliar, [Lume](https://lume.land) is a static site generator built
with Deno, featuring a rather fiery dino as its icon. However, I remember
struggling with its template engine, which I think was **njk**.

Intrigued, I revisited Lume to find that it had also reached version 2, complete
with significantly improved documentation. After spending a day diving into it,
I thought, "Hey I could actually do this." While the default template engine has
switched from **njk** to **vto**, it also supported **md** and **tsx**, which
was a plus. As I worked, I found myself getting more and more excited about the
project.

And then it hit me: "Why not deploy it to
[Deno Deploy](https://deno.com/deploy)?" I'd been using
[Cloudflare Pages](https://www.cloudflare.com/developer-platform/products/pages/)
for my current site, and while I've found it incredibly user-friendly
(especially with its Japanese dashboard), I was curious about Deno Deploy. After
all, part of the fun of having a personal website is experimenting with
different technologies.

So, I decided to make the switch. I already has the domain registered with
[Cloudflare Registrar](https://www.cloudflare.com/products/registrar/). I
remember purchasing it when the prices seemed particularly attractive compared
to other domestic registrars. I'm not sure if that's still the case.

Setting up a custom domain on Deno Deploy was pretty straightforward. I simply
obtained the **DNS records** from the Deno Deploy dashboard and added them to
Cloudflare. After verifying the domain on Deno Deploy, I configured **SSL/TLS**.
I learned that setting the encryption mode to full is crucial to avoid redirect
loops...\
Finally , I generated a key on Cloudflare and added it to Deno Deploy.

While setting up a Cloudflare Registrar domain on Cloudflare Pages was a breeze,
**I was pleasantly surprised by how easy it was to do the same on Deno Deploy.**
As someone who doesn't consider themselves a networking expert, I was thrilled
to successfully complete the configuration.
