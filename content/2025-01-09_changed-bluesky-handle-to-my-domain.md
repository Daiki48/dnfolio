+++
title = "Changed Bluesky's handle to my domain"
slug = "bluesky-handle-custom-domain"
description = "I changed the handle of my Bluesky account to the domain I hold."
draft = false
[taxonomies]
tags = ["Bluesky", "Cloudflare"]
languages = ["en"]
+++

My Bluesky account used **daiki48.bsky.social** .

And now I've updated my Bluesky account handle to **dnfolio.dev** .
I will share how to set this up for **Cloudflare Registrar** , where I manage the domain.

## How to set up

### Bluesky side

Select Bluesky **settings** button.

![1-settings](/content/changed-bluesky-handle-to-my-domain/1-settings.webp)

Select **Account** button.

![2-account](/content/changed-bluesky-handle-to-my-domain/2-account.webp)

Select **Handle** button.

![3-handle](/content/changed-bluesky-handle-to-my-domain/3-handle.webp)

Select **I have my own domain** button.

![4-own-domain](/content/changed-bluesky-handle-to-my-domain/4-own-domain.webp)

Enter the domain you want to use.

![5-change-handle](/content/changed-bluesky-handle-to-my-domain/5-change-handle.webp)

Select **DNS Panel** tab.

### Cloudflare side

In the account home, select the domain.

![6-cloudflare-account-home](/content/changed-bluesky-handle-to-my-domain/6-cloudflare-account-home.webp)

In the left sidebar, Select **DNS Records** button.

![7-dns-records](/content/changed-bluesky-handle-to-my-domain/7-dns-records.webp)

Select **Add record** button.

![8-add-record](/content/changed-bluesky-handle-to-my-domain/8-add-record.webp)

Enter the **Type**, **Name**, **Content** .

> **Name** corresponds to **Host** and **Value** corresponds to **Content** .

![9-save-record](/content/changed-bluesky-handle-to-my-domain/9-save-record.webp)

Added record.

![10-record](/content/changed-bluesky-handle-to-my-domain/10-record.webp)

### Return Bluesky side

Press the **Verify DNS Record** button.

Then I was able to update the Bluesky handle. Congratulations!!

![11-confirm](/content/changed-bluesky-handle-to-my-domain/11-confirm.webp)

When you refresh the Bluesky screen, the handle displayed on your profile screen is set to the value you have set.
Glad to see that.
