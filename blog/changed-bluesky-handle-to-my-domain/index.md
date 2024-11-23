---
title: Changed Bluesky's handle to my domain
description: I changed the handle of my Bluesky account to the domain I hold.
tags:
  - life
  - bluesky
  - cloudflare
---

My Bluesky account used **daiki48.bsky.social** .

And now I've updated my Bluesky account handle to **dnfolio.dev** .
I will share how to set this up for **Cloudflare Registrar** , where I manage the domain.

## How to set up

### Bluesky side

Select Bluesky **settings** button.  

![1-settings](./1-settings.webp)

Select **Account** button.

![2-account](./2-account.webp)

Select **Handle** button.

![3-handle](./3-handle.webp)

Select **I have my own domain** button.

![4-own-domain](./4-own-domain.webp)

Enter the domain you want to use.

![5-change-handle](./5-change-handle.webp)

Select **DNS Panel** tab.

### Cloudflare side

In the account home, select the domain.

![6-cloudflare-account-home](./6-cloudflare-account-home.webp)

In the left sidebar, Select **DNS Records** button.

![7-dns-records](./7-dns-records.webp)

Select **Add record** button.

![8-add-record](./8-add-record.webp)

Enter the **Type**, **Name**, **Content** .

> [!NOTE]
> **Name** corresponds to **Host** and **Value** corresponds to **Content** .

![9-save-record](./9-save-record.webp)

Added record.

![10-record](./10-record.webp)

### Return Bluesky side

Press the **Verify DNS Record** button.

Then I was able to update the Bluesky handle. Congratulations!!

![11-confirm](./11-confirm.webp)

When you refresh the Bluesky screen, the handle displayed on your profile screen is set to the value you have set.
Glad to see that.
